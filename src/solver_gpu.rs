use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    memory::allocator::StandardMemoryAllocator,
    pipeline::{Pipeline, PipelineBindPoint},
    sync::{self, GpuFuture},
};
use std::time::Instant;

use crate::{gpu_utils::{init, get_compute_pipeline}};

const RUN_COUNT: u32 = 5000;

// fn gol_to_buff(gol: &GolInstance) -> Vec<i32> {
//     let cells = gol.cells;
// }

pub fn run_gpu() {
    let before = Instant::now();
    let (device, queue) = init();
    mod cs {
        vulkano_shaders::shader! {
            ty: "compute",
            path: "./src/shaders/test_shader3.glsl",
            // types_meta: {
            //     use bytemuck::{Pod, Zeroable};
    
            //     #[derive(Clone, Copy, Zeroable, Pod)]
            // },
        }
    }
    let shader = cs::load(device.clone()).unwrap();

    let pipeline = get_compute_pipeline(device.clone(), shader.clone());

    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());
    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());
    let command_buffer_allocator =
        StandardCommandBufferAllocator::new(device.clone(), Default::default());

    let data_buffer = {
        let data_iter = 0..65536u32;
        CpuAccessibleBuffer::from_iter(
            &memory_allocator,
            BufferUsage {
                storage_buffer: true,
                ..BufferUsage::empty()
            },
            false,
            data_iter,
        )
        .unwrap()
    };

    let data_buffer2 = {
        let data_iter = 0..65536u32;
        CpuAccessibleBuffer::from_iter(
            &memory_allocator,
            BufferUsage {
                storage_buffer: true,
                ..BufferUsage::empty()
            },
            false,
            data_iter,
        )
        .unwrap()
    };

    let layout = pipeline.layout().set_layouts().get(0).unwrap();
    let set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        layout.clone(),
        [
            WriteDescriptorSet::buffer(0, data_buffer.clone()),
            WriteDescriptorSet::buffer(1, data_buffer2.clone()),
        ],
    )
    .unwrap();

    let set_reverse = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        layout.clone(),
        [
            WriteDescriptorSet::buffer(1, data_buffer.clone()),
            WriteDescriptorSet::buffer(0, data_buffer2.clone()),
        ],
    )
    .unwrap();

    // let params = cs::ty::Params {
    //     val: VALUE_MUL
    // };

    let mut builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();
    builder
        .bind_pipeline_compute(pipeline.clone());
        // .bind_descriptor_sets(
        //     PipelineBindPoint::Compute,
        //     pipeline.layout().clone(),
        //     0,
        //     set.clone(),
        // );
    
    for _i in 0..RUN_COUNT {
        builder
            .bind_descriptor_sets( // default descriptors
                PipelineBindPoint::Compute,
                pipeline.layout().clone(),
                0,
                set.clone(),
            )
            .dispatch([1024, 1, 1])
            .unwrap()
            .bind_descriptor_sets( // reversed descriptors
                PipelineBindPoint::Compute,
                pipeline.layout().clone(),
                0,
                set_reverse.clone(),
            )
            .dispatch([1024, 1, 1])
            .unwrap();
    }
    
    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let data_buffer_content = data_buffer.read().unwrap();
    for n in 0..65536u32 {
        assert_eq!(data_buffer_content[n as usize], n + (2 * RUN_COUNT));
    }

    println!("Success {:?} / {}", before.elapsed(), RUN_COUNT * 2);
}