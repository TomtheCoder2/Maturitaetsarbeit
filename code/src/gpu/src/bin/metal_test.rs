use metal::*;

fn main() {
    // Get the default Metal device
    let device = Device::system_default().expect("No Metal device found");
    let command_queue = device.new_command_queue();

    // Create two input vectors (example with 1024 floats)
    let size = 530_816;
    let input1: Vec<f32> = (0..size).map(|i| i as f32).collect();
    let input2: Vec<f32> = (0..size).map(|i| (i + 1) as f32).collect();
    let mut output = vec![0.0f32; size];

    let t0 = std::time::Instant::now();
    // Create Metal buffers
    let buffer1 = device.new_buffer_with_data(
        input1.as_ptr() as *const _,
        (size * std::mem::size_of::<f32>()) as u64,
        MTLResourceOptions::CPUCacheModeWriteCombined,
    );
    let buffer2 = device.new_buffer_with_data(
        input2.as_ptr() as *const _,
        (size * std::mem::size_of::<f32>()) as u64,
        MTLResourceOptions::CPUCacheModeWriteCombined,
    );
    let output_buffer = device.new_buffer(
        (size * std::mem::size_of::<f32>()) as u64,
        MTLResourceOptions::CPUCacheModeWriteCombined,
    );
    println!(
        "Buffer creation time: {:.3} ms",
        t0.elapsed().as_micros() as f32 / 1000f32
    );

    // Metal Shading Language (MSL) compute shader source
    let shader_source = include_str!("../../shaders/add.metal");

    // Compile the shader into a library
    let library = device
        .new_library_with_source(shader_source, &CompileOptions::new())
        .expect("Failed to compile shader");

    let function = library
        .get_function("add_arrays", None)
        .expect("Function not found");
    let pipeline = device
        .new_compute_pipeline_state_with_function(&function)
        .expect("Pipeline creation failed");

    // Create command buffer and encoder
    let command_buffer = command_queue.new_command_buffer();
    let encoder = command_buffer.new_compute_command_encoder();

    encoder.set_compute_pipeline_state(&pipeline);
    encoder.set_buffer(0, Some(&buffer1), 0);
    encoder.set_buffer(1, Some(&buffer2), 0);
    encoder.set_buffer(2, Some(&output_buffer), 0);

    // Dispatch threads (one thread per element, in groups of 64 for efficiency)
    let threads_per_group = MTLSize::new(64, 1, 1);
    let num_groups = MTLSize::new(((size + 63) / 64) as NSUInteger, 1, 1);
    let thread_group = MTLSize::new(size as u64, 1, 1);
    encoder.dispatch_thread_groups(num_groups, threads_per_group);
    // Correction: dispatch_threads for total threads
    // encoder.dispatch_threads(thread_group, threads_per_group);

    // Note: For simple dispatch, use dispatch_threads
    let threads_per_threadgroup = MTLSize {
        width: 64,
        height: 1,
        depth: 1,
    };
    let num_threadgroups = MTLSize {
        width: ((size + 63) / 64) as NSUInteger,
        height: 1,
        depth: 1,
    };
    encoder.dispatch_thread_groups(num_threadgroups, threads_per_threadgroup);

    encoder.end_encoding();

    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Copy output back to host
    let output_ptr = output_buffer.contents() as *mut f32;
    unsafe {
        output.copy_from_slice(std::slice::from_raw_parts(output_ptr, size));
    }

    // Print first few results for verification
    println!("First 5 results:");
    for i in 0..5 {
        println!("{} + {} = {}", input1[i], input2[i], output[i]);
    }
}
