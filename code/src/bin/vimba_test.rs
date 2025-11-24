use vimba_sys::{
    VmbAccessModeType, VmbCameraInfo_t, VmbErrorType, VmbFrameStatusType, VmbFrame_t, VmbHandle_t,
    VmbVersionInfo_t,
};

fn err_str(err: i32) -> &'static str {
    use VmbErrorType::*;
    #[allow(non_upper_case_globals)]
    match err {
        VmbErrorSuccess => "VmbErrorSuccess",
        VmbErrorInternalFault => "VmbErrorInternalFault",
        VmbErrorApiNotStarted => "VmbErrorApiNotStarted",
        VmbErrorNotFound => "VmbErrorNotFound",
        VmbErrorBadHandle => "VmbErrorBadHandle",
        VmbErrorDeviceNotOpen => "VmbErrorDeviceNotOpen",
        VmbErrorInvalidAccess => "VmbErrorInvalidAccess",
        VmbErrorBadParameter => "VmbErrorBadParameter",
        VmbErrorStructSize => "VmbErrorStructSize",
        VmbErrorMoreData => "VmbErrorMoreData",
        VmbErrorWrongType => "VmbErrorWrongType",
        VmbErrorInvalidValue => "VmbErrorInvalidValue",
        VmbErrorTimeout => "VmbErrorTimeout",
        VmbErrorOther => "VmbErrorOther",
        VmbErrorResources => "VmbErrorResources",
        VmbErrorInvalidCall => "VmbErrorInvalidCall",
        VmbErrorNoTL => "VmbErrorNoTL",
        VmbErrorNotImplemented => "VmbErrorNotImplemented",
        VmbErrorNotSupported => "VmbErrorNotSupported",
        VmbErrorIncomplete => "VmbErrorIncomplete",
        VmbErrorIO => "VmbErrorIO",
        _ => "unknown error",
    }
}

fn vimba_err(err: i32) -> anyhow::Result<()> {
    if err == VmbErrorType::VmbErrorSuccess {
        Ok(())
    } else {
        Err(anyhow::anyhow!("vimba error {}: {}", err, err_str(err)))
    }
}

macro_rules! vimba_call {
    ($expr: expr) => {{
        vimba_err(unsafe { $expr })
    }};
}

/// Automatically shutdown Vimba when library reference goes out of scope.
struct VimbaLib {
    lib: vimba_sys::VimbaC,
    started: bool,
}

impl VimbaLib {
    fn new(lib: vimba_sys::VimbaC) -> anyhow::Result<Self> {
        // start vimba
        vimba_call!(lib.VmbStartup())?;
        Ok(Self { lib, started: true })
    }
}

impl Drop for VimbaLib {
    fn drop(&mut self) {
        if self.started {
            // shutdown
            unsafe { self.lib.VmbShutdown() };
            self.started = false;
        }
    }
}

fn main() -> anyhow::Result<()> {
    let vimba_handle: VmbHandle_t = 1 as _;

    #[cfg(target_os = "windows")]
    let vimbac_path = r#"C:\Program Files\Allied Vision\Vimba_6.0\VimbaC\Lib\Win64\VimbaC.dll"#;

    #[cfg(not(target_os = "windows"))]
    let vimbac_path = "/opt/Vimba_6_1/VimbaC/DynamicLib/arm_64bit/libVimbaC.so";

    let vimba_lib = unsafe { vimba_sys::VimbaC::new(vimbac_path) }?;

    // start vimba
    let vmb = VimbaLib::new(vimba_lib)?;

    // print version
    let mut version_info = VmbVersionInfo_t {
        major: 0,
        minor: 0,
        patch: 0,
    };
    vimba_call!(vmb.lib.VmbVersionQuery(
        &mut version_info,
        std::mem::size_of::<VmbVersionInfo_t>() as u32
    ))?;
    println!(
        "Vimba API Version: {}.{}.{}",
        version_info.major, version_info.minor, version_info.patch
    );
    // check if GigE is available
    let mut is_gige_avail = 0;
    let data = std::ffi::CString::new("GeVTLIsPresent")?;
    vimba_call!(vmb
        .lib
        .VmbFeatureBoolGet(vimba_handle, data.as_ptr(), &mut is_gige_avail))?;
    println!("GigE is available: {}", is_gige_avail);

    // get camera list
    let mut n_count = 0;
    vimba_call!(vmb
        .lib
        .VmbCamerasList(std::ptr::null_mut(), 0, &mut n_count, 0))?;
    println!("{} cameras found", n_count);

    let mut cameras: Vec<VmbCameraInfo_t> = vec![
        VmbCameraInfo_t {
            cameraIdString: std::ptr::null_mut(),
            cameraName: std::ptr::null_mut(),
            modelName: std::ptr::null_mut(),
            serialString: std::ptr::null_mut(),
            permittedAccess: 0,
            interfaceIdString: std::ptr::null_mut(),
        };
        n_count as usize
    ];

    let mut n_found_count = 0;
    vimba_call!(vmb.lib.VmbCamerasList(
        cameras[..].as_mut_ptr(),
        n_count,
        &mut n_found_count,
        std::mem::size_of::<VmbCameraInfo_t>().try_into()?
    ))?;

    assert_eq!(n_count, n_found_count);

    for i in 0..n_found_count as usize {
        println!("Camera {}:", i);
        println!("  cameraIdString: {}", unsafe {
            std::ffi::CStr::from_ptr(cameras[i].cameraIdString).to_str()
        }?);
        println!("  cameraName: {}", unsafe {
            std::ffi::CStr::from_ptr(cameras[i].cameraName).to_str()
        }?);
        println!("  modelName: {}", unsafe {
            std::ffi::CStr::from_ptr(cameras[i].modelName).to_str()
        }?);
        println!("  serialString: {}", unsafe {
            std::ffi::CStr::from_ptr(cameras[i].serialString).to_str()
        }?);
        println!("  permittedAccess: {}", cameras[i].permittedAccess);
        println!("  interfaceIdString: {}", unsafe {
            std::ffi::CStr::from_ptr(cameras[i].interfaceIdString).to_str()
        }?);
    }

    let mut camera_handle = std::ptr::null_mut(); // VmbHandle_t         cameraHandle        = NULL;
    let camera_access_mode = VmbAccessModeType::VmbAccessModeFull; // We open the camera with full access

    if cameras.len() > 0 {
        println!("Opening camera 0...");
        vimba_call!(vmb.lib.VmbCameraOpen(
            cameras[0].cameraIdString,
            camera_access_mode as u32, // API bug? Should not require cast.
            &mut camera_handle,
        ))?;
        println!("...OK");

        {
            // Get the firmware version
            let mut buf = [0u8; 100];
            let buf_ptr = (&mut buf[..]).as_mut_ptr() as *mut i8;
            let mut filled: u32 = 0;
            let data = std::ffi::CString::new("DeviceFirmwareVersion")?;
            vimba_call!(vmb.lib.VmbFeatureStringGet(
                camera_handle,
                data.as_ptr(),
                buf_ptr,
                100,
                &mut filled,
            ))?;

            let firmware_version = unsafe { std::ffi::CStr::from_ptr(buf_ptr) }.to_str()?;
            println!("DeviceFirmwareVersion: {:?}", firmware_version);
        }

        {
            // Get the pixel format
            let mut pixel_format: *const std::os::raw::c_char = std::ptr::null();
            let data = std::ffi::CString::new("PixelFormat")?;
            vimba_call!(vmb.lib.VmbFeatureEnumGet(
                camera_handle,
                data.as_ptr(),
                &mut pixel_format
            ))?;
            println!("PixelFormat: {}", unsafe {
                std::ffi::CStr::from_ptr(pixel_format).to_str()
            }?);
        }

        // {
        //     // Get the sensor bit depth
        //     let mut sensor_bit_depth: *const std::os::raw::c_char = std::ptr::null();
        //     let data = std::ffi::CString::new("SensorBitDepth")?;
        //     vimba_call!(vmb.lib.VmbFeatureEnumGet(
        //         camera_handle,
        //         data.as_ptr(),
        //         &mut sensor_bit_depth
        //     ))?;
        //     println!("SensorBitDepth: {}", unsafe {
        //         std::ffi::CStr::from_ptr(sensor_bit_depth).to_str()
        //     }?);
        // }

        // Get the payload size
        let mut payload_size = 0;
        let data = std::ffi::CString::new("PayloadSize")?;
        vimba_call!(vmb
            .lib
            .VmbFeatureIntGet(camera_handle, data.as_ptr(), &mut payload_size,))?;
        println!("PayloadSize: {}", payload_size);

        let mut frame = std::mem::MaybeUninit::<VmbFrame_t>::uninit();

        let mut payload: Vec<u8> = vec![0; payload_size.try_into().unwrap()];

        unsafe {
            // This truly is unsafe - we must manually ensure `payload` has a
            // longer lifetime than `frame`.
            (*frame.as_mut_ptr()).buffer = payload.as_mut_ptr() as _;
            (*frame.as_mut_ptr()).bufferSize = payload.len().try_into().unwrap();
        }

        vimba_call!(vmb.lib.VmbFrameAnnounce(
            camera_handle,
            frame.as_mut_ptr(),
            std::mem::size_of::<VmbFrame_t>().try_into().unwrap()
        ))?;

        vimba_call!(vmb.lib.VmbCaptureStart(camera_handle))?;

        vimba_call!(vmb
            .lib
            .VmbCaptureFrameQueue(camera_handle, frame.as_mut_ptr(), None))?;

        let data = std::ffi::CString::new("AcquisitionStart")?;
        vimba_call!(vmb.lib.VmbFeatureCommandRun(camera_handle, data.as_ptr(),))?;

        let n_timeout = 2000;
        vimba_call!(vmb
            .lib
            .VmbCaptureFrameWait(camera_handle, frame.as_mut_ptr(), n_timeout))?;

        let frame = unsafe { frame.assume_init() };

        if frame.receiveStatus == VmbFrameStatusType::VmbFrameStatusComplete {
            println!("frame complete");
            println!("imageSize {}", frame.imageSize);
            println!("{}x{}", frame.width, frame.height);
        } else {
            println!("frame not complete. status: {}", frame.receiveStatus);
        }
    }

    Ok(())
}