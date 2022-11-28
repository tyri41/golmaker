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

use crate::{gpu_utils::{init, get_compute_pipeline}, gol_instance::GolInstance};

const RUN_COUNT: u32 = 10;

const WORK_GROUP_SIZE: u32 = 16;

fn ceil_div(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

pub fn iterate_gpu(gol: &mut GolInstance, t: usize) {
    let before = Instant::now();
    let (device, queue) = init();
    mod cs {
        vulkano_shaders::shader! {
            ty: "compute",
            path: "./src/shaders/gol_shader.glsl",
            types_meta: {
                use bytemuck::{Pod, Zeroable};
    
                #[derive(Clone, Copy, Zeroable, Pod)]
            },
        }
    }
    let shader = cs::load(device.clone()).unwrap();

    let pipeline = get_compute_pipeline(device.clone(), shader.clone());

    let memory_allocator = StandardMemoryAllocator::new_default(device.clone());
    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());
    let command_buffer_allocator =
        StandardCommandBufferAllocator::new(device.clone(), Default::default());

    let data_buffer = {
        let data_iter = gol.flatten();
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
        let data_iter = gol.flatten();
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

    let params = cs::ty::Params {
        h: gol.h as u32,
        w: gol.w as u32
    };

    let mut builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();
    builder
        .bind_pipeline_compute(pipeline.clone())
        .push_constants(pipeline.layout().clone(), 0, params);

    let run_count: usize = t / 2;
    let group_count_x = ceil_div(gol.h as u32, WORK_GROUP_SIZE);
    let group_count_y = ceil_div(gol.w as u32, WORK_GROUP_SIZE);
    
    for _i in 0..run_count {
        builder
            .bind_descriptor_sets( // default descriptors
                PipelineBindPoint::Compute,
                pipeline.layout().clone(),
                0,
                set.clone(),
            )
            .dispatch([group_count_x, group_count_y, 1])
            .unwrap()
            .bind_descriptor_sets( // reversed descriptors
                PipelineBindPoint::Compute,
                pipeline.layout().clone(),
                0,
                set_reverse.clone(),
            )
            .dispatch([group_count_x, group_count_y, 1])
            .unwrap();
    }

    // run one more if t is odd
    if t & 1 != 0 { 
        builder
            .bind_descriptor_sets( // default descriptors
                PipelineBindPoint::Compute,
                pipeline.layout().clone(),
                0,
                set.clone(),
            )
            .dispatch([group_count_x, group_count_y, 1])
            .unwrap();
    }
    
    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let data_buffer_content = {
        if t & 1 != 0 {
            data_buffer2.read().unwrap()
        } else {
            data_buffer.read().unwrap()
        }
    };
    let flat_vec = data_buffer_content.to_vec();

    println!("GOL GPU finished in {:?} / {} iterations", before.elapsed(), t);
    gol.update_flat(flat_vec);
}

#[allow(dead_code)]
pub fn iterate_gpu_debug(gol: &mut GolInstance, t: usize, p: bool) {
    for _i in 0..t {
        iterate_gpu(gol, 1);
        if p {
            println!("{}", gol.show());
        }
    }
}

#[allow(dead_code)]
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