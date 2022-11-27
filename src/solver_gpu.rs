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

use crate::{gpu_utils::{init, get_compute_pipeline}, gol_instance::{self, GolInstance}};

// const VALUE_MUL: u32 = 14;

const RUN_COUNT: u32 = 10000;

// fn gol_to_buff(gol: &GolInstance) -> Vec<i32> {
//     let cells = gol.cells;
// }

pub fn run_gpu() {
    let (device, queue) = init();
    mod cs {
        vulkano_shaders::shader! {
            ty: "compute",
            path: "./src/shaders/test_shader.glsl",
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

    // We start by creating the buffer that will store the data.
    let data_buffer = {
        // Iterator that produces the data.
        let data_iter = 0..65536u32;
        // Builds the buffer and fills it with this iterator.
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
        ],
    )
    .unwrap();

    // let params = cs::ty::Params {
    //     val: VALUE_MUL
    // };

    // In order to execute our operation, we have to build a command buffer.
    let mut builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();
    builder
        .bind_pipeline_compute(pipeline.clone())
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            pipeline.layout().clone(),
            0,
            set.clone(),
        );
    
    for _i in 0..RUN_COUNT {
        builder
            .dispatch([1024, 1, 1])
            .unwrap();
    }
        
    // Finish building the command buffer by calling `build`.
    let command_buffer = builder.build().unwrap();

    // Let's execute this command buffer now.
    // To do so, we TODO: this is a bit clumsy, probably needs a shortcut
    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        // This line instructs the GPU to signal a *fence* once the command buffer has finished
        // execution. A fence is a Vulkan object that allows the CPU to know when the GPU has
        // reached a certain point.
        // We need to signal a fence here because below we want to block the CPU until the GPU has
        // reached that point in the execution.
        .then_signal_fence_and_flush()
        .unwrap();

    // Blocks execution until the GPU has finished the operation. This method only exists on the
    // future that corresponds to a signalled fence. In other words, this method wouldn't be
    // available if we didn't call `.then_signal_fence_and_flush()` earlier.
    // The `None` parameter is an optional timeout.
    //
    // Note however that dropping the `future` variable (with `drop(future)` for example) would
    // block execution as well, and this would be the case even if we didn't call
    // `.then_signal_fence_and_flush()`.
    // Therefore the actual point of calling `.then_signal_fence_and_flush()` and `.wait()` is to
    // make things more explicit. In the future, if the Rust language gets linear types vulkano may
    // get modified so that only fence-signalled futures can get destroyed like this.
    future.wait(None).unwrap();

    // Now that the GPU is done, the content of the buffer should have been modified. Let's
    // check it out.
    // The call to `read()` would return an error if the buffer was still in use by the GPU.
    let data_buffer_content = data_buffer.read().unwrap();
    for n in 0..65536u32 {
        assert_eq!(data_buffer_content[n as usize], n + RUN_COUNT);
    }

    println!("Success 1");
}