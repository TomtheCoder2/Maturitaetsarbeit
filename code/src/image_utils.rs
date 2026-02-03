use metal::*;
use objc::rc::autoreleasepool;
use std::mem;
use std::os::raw::c_void;
use std::sync::Arc;

// Repr for Metal struct alignment
#[repr(C, align(4))]
struct Globals {
    width: u32,
    height: u32,
}

/// Reusable Metal context (init once in thread to avoid Send/Sync issues).
pub struct MetalContext {
    device: Arc<Device>,
    command_queue: Arc<CommandQueue>,
    pipeline: Arc<ComputePipelineState>,
    _library: Arc<Library>,
}

impl MetalContext {
    /// Init once (e.g., in spawn closure).
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let device = Device::system_default().ok_or("No Metal device found")?;
        let device = Arc::new(device);

        let command_queue = device.new_command_queue();
        let command_queue = Arc::new(command_queue);

        let metal_lib_path = std::env::var("DEBAYER_METAL_LIB")?;
        let library = device.new_library_with_file(&metal_lib_path)?;
        let library = Arc::new(library);

        let desc = FunctionDescriptor::new();
        desc.set_name("debayer_kernel");
        let function = library.new_function_with_descriptor(&desc)?;

        let pipeline_descriptor = ComputePipelineDescriptor::new();
        pipeline_descriptor.set_compute_function(Some(&function));
        let pipeline = device.new_compute_pipeline_state(&pipeline_descriptor)?;
        let pipeline = Arc::new(pipeline);

        Ok(MetalContext {
            device,
            command_queue,
            pipeline,
            _library: library,
        })
    }

    /// Debayer per frame (reuse context; zero-copy).
    pub fn debayer(
        &self,
        raw_data: &[u8],
        width: usize,
        height: usize,
        reverse_bits: bool,
    ) -> Vec<u8> {
        let num_pixels = width * height;
        let expected_len = (num_pixels * 3) / 2;
        assert_eq!(raw_data.len(), expected_len);

        autoreleasepool(|| {
            // Step 1: Unpack (CPU, shared Vec)
            let bayer_length = (num_pixels * mem::size_of::<u16>()) as u64;
            let mut bayer_u16: Vec<u16> = vec![0u16; num_pixels];
            for y in 0..height {
                let row_bytes = (width * 3) / 2;
                let mut byte_idx = y * row_bytes;
                for x in (0..width).step_by(2) {
                    let b0 = raw_data[byte_idx] as u16;
                    let b1 = raw_data[byte_idx + 1] as u16;
                    let b2 = raw_data[byte_idx + 2] as u16;
                    let (pix1, pix2) = if reverse_bits {
                        (((b1 & 0x0F) << 8) | b0, (b2 << 4) | (b1 >> 4))
                    } else {
                        ((b0 << 4) | (b1 >> 4), ((b1 & 0x0F) << 8) | b2)
                    };
                    let idx1 = y * width + x;
                    bayer_u16[idx1] = pix1.min(4095);
                    bayer_u16[idx1 + 1] = pix2.min(4095);
                    byte_idx += 3;
                }
            }

            // Step 2: Output Vec (shared)
            let mut rgb_u8: Vec<u8> = vec![0u8; num_pixels * 3];
            let rgb_length = (num_pixels * 3) as u64;

            // Step 3: Zero-copy buffers (reuse context, recreate for size)
            let bayer_buffer = self.device.new_buffer_with_bytes_no_copy(
                bayer_u16.as_ptr() as *const c_void,
                bayer_length,
                MTLResourceOptions::StorageModeShared,
                None, // No dealloc
            );
            let rgb_buffer = self.device.new_buffer_with_bytes_no_copy(
                rgb_u8.as_mut_ptr() as *mut c_void,
                rgb_length,
                MTLResourceOptions::StorageModeShared,
                None,
            );

            // Globals buffer (recreate for dims)
            let globals_data = Globals {
                width: width as u32,
                height: height as u32,
            };
            let globals_buffer = self.device.new_buffer_with_data(
                &globals_data as *const _ as *const c_void,
                size_of::<Globals>() as u64,
                MTLResourceOptions::StorageModeShared,
            );

            // Step 4: Dispatch (reuse queue/pipeline)
            let command_buffer = self.command_queue.new_command_buffer();
            let encoder = command_buffer.new_compute_command_encoder();

            encoder.set_compute_pipeline_state(&self.pipeline);
            encoder.set_buffer(0, Some(&bayer_buffer), 0);
            encoder.set_buffer(1, Some(&rgb_buffer), 0);
            encoder.set_buffer(2, Some(&globals_buffer), 0);

            let threadgroup = MTLSize::new(16, 16, 1);
            let num_groups_x = (width as u64).div_ceil(16);
            let num_groups_y = (height as u64).div_ceil(16);
            let threadgroups =
                MTLSize::new(num_groups_x as NSUInteger, num_groups_y as NSUInteger, 1);
            encoder.dispatch_thread_groups(threadgroups, threadgroup);

            encoder.end_encoding();
            command_buffer.commit();
            command_buffer.wait_until_completed();

            rgb_u8
        })
    }
}

/// Wrapper for ease (init inside if needed, but prefer reuse).
pub fn gpu_debayer(raw_data: &[u8], width: usize, height: usize, reverse_bits: bool) -> Vec<u8> {
    let context = MetalContext::new().expect("Metal init failed");
    context.debayer(raw_data, width, height, reverse_bits)
}
pub fn save_rgb8_image(p0: &String, p1: &[u8], p2: usize, p3: usize) -> Result<(), std::io::Error> {
    use image::{ImageBuffer, Rgb};
    let img_buffer: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_raw(p2 as u32, p3 as u32, p1)
        .ok_or(std::io::Error::other("Failed to create image buffer"))?;
    img_buffer.save(p0).map_err(std::io::Error::other)?;
    Ok(())
}
