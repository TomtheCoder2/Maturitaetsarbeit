use image::GenericImage;
use image::GenericImageView;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

pub struct PythonScript {
    process: std::process::Child,
}

impl PythonScript {
    pub fn new() -> Self {
        let python_process = Command::new("python")
            .arg("../../python_code_ma/tf_detect_player.py") // Python script to read from memory-mapped region
            .stdin(Stdio::piped()) // Open stdin to send data to Python
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start Python program");

        PythonScript {
            process: python_process,
        }
    }

    pub fn detect_player(&mut self, img: &[u8]) -> (u32, u32) {
        // Get the stdin handle of the Python process
        if let Some(ref mut stdin) = self.process.stdin {
            println!("Got stdin.");
            // Send the image data to Python over stdin
            stdin.write_all(&img).expect("Failed to write to stdin");
            stdin.flush().expect("Failed to flush stdin");
        } else {
            panic!("Failed to open stdin");
        }
        // println!("Sent image data to Python.");
        let stdout = self.process.stdout.as_mut().expect("Failed to open stdout");
        let mut stdout = BufReader::new(stdout);
        let mut coords = (0, 0);
        loop {
            let len = match stdout.fill_buf() {
                Ok(stdout) => {
                    // ik its missspelled but i cant use break
                    let mut brake = false;
                    if stdout.is_empty() {
                        continue;
                    }
                    let len = stdout.len();
                    for line in stdout.lines() {
                        // println!("Got line.");
                        let line = line.expect("Failed to read line");
                        // println!("Python output: {}", line); // Optional: print the Python output
                        // the programm sends the coords like this: "c:35, 31"
                        if line.starts_with("c:") {
                            let cs = line
                                .trim_start_matches("c:")
                                .split(", ")
                                .map(|s| s.parse::<u32>().unwrap())
                                .collect::<Vec<u32>>();
                            println!("Received coordinates: {:?}", cs);
                            coords = (cs[0], cs[1]);
                            brake = true;
                            break;
                        }

                        // Check if the line contains the "Done" message
                        if line.trim().contains("Done") {
                            println!("Received 'Done' from Python. Continuing in Rust...");
                            brake = true;
                            break;
                        }
                    }
                    if brake {
                        break;
                    }
                    len
                }
                other => panic!("Some better error handling here... {:?}", other),
            };
            stdout.consume(len);
        }
        coords
    }

    pub fn finish(&mut self) {
        self.process.kill().expect("Failed to kill Python process");
    }
}

fn main() -> io::Result<()> {
    // Load and compress the image in PNG format
    let img = image::open("../../player_training_images/34_32_2_2_0010.png").unwrap();
    let img = img.resize_exact(128, 128, image::imageops::FilterType::Nearest);
    println!(
        "length: {}, dimensions: {:?}",
        img.to_rgb8().into_raw().len(),
        img.dimensions()
    );
    let values = img
        .to_rgb8()
        .into_raw()
        .iter()
        .map(|&v| v as u8)
        .collect::<Vec<u8>>();
    let width = img.width() as usize;
    let height = img.height() as usize;
    let image = values.clone();
    let mut img = image::DynamicImage::ImageRgb8(
        image::ImageBuffer::from_raw(width as u32, height as u32, image.clone()).unwrap(),
    );
    for i in 0..width * height {
        let r = image[i * 3];
        let g = image[i * 3 + 1];
        let b = image[i * 3 + 2];
        img.put_pixel(
            i as u32 % width as u32,
            i as u32 / width as u32,
            image::Rgba([r, g, b, 100]),
        );
    }

    let mut python_script = PythonScript::new();
    let coords = python_script.detect_player(&values);
    println!("Coords: {:?}", coords);
    let t0 = std::time::Instant::now();
    let coords = python_script.detect_player(&values);
    println!("Coords: {:?}, time: {:?}", coords, t0.elapsed());
    python_script.finish();

    // // Start the Python process and pass the image data via stdin (pipe)
    // let mut python_process = Command::new("python")
    //     // .arg("../../python_code_ma/read_shared_memory.py")
    //     .arg("../../python_code_ma/tf_detect_player.py") // Python script to read from memory-mapped region
    //     .stdin(Stdio::piped()) // Open stdin to send data to Python
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::inherit())
    //     .spawn()
    //     .expect("Failed to start Python program");

    // // Get the stdin handle of the Python process
    // if let Some(ref mut stdin) = python_process.stdin {
    //     println!("Got stdin.");
    //     // Send the image data to Python over stdin
    //     stdin.write_all(&values)?;
    //     stdin.flush()?;
    // } else {
    //     panic!("Failed to open stdin");
    // }
    // println!("Sent image data to Python.");

    // // while until stdout is valid
    // let mut stdout = python_process.stdout.take();
    // while stdout.is_none() {
    //     stdout = python_process.stdout.take();
    // }
    // let stdout = stdout.expect("Wasn't stdout");
    // println!("Got stdout.");
    // // let stderr = python_process.stderr.as_mut().expect("Wasn't stderr");

    // let mut stdout = BufReader::new(stdout);
    // // let mut stderr = BufReader::new(stderr);

    // loop {
    //     let len = match stdout.fill_buf() {
    //         Ok(stdout) => {
    //             // ik its missspelled but i cant use break
    //             let mut brake = false;
    //             if stdout.is_empty() {
    //                 continue;
    //             }
    //             let len = stdout.len();
    //             for line in stdout.lines() {
    //                 // println!("Got line.");
    //                 let line = line?;
    //                 // println!("Python output: {}", line); // Optional: print the Python output
    //                 // the programm sends the coords like this: "c:35, 31"
    //                 if line.starts_with("c:") {
    //                     let coords = line
    //                         .trim_start_matches("c:")
    //                         .split(", ")
    //                         .map(|s| s.parse::<u32>().unwrap())
    //                         .collect::<Vec<u32>>();
    //                     println!("Received coordinates: {:?}", coords);
    //                     brake = true;
    //                     break;
    //                 }

    //                 // Check if the line contains the "Done" message
    //                 if line.trim().contains("Done") {
    //                     println!("Received 'Done' from Python. Continuing in Rust...");
    //                     brake = true;
    //                     break;
    //                 }
    //             }
    //             if brake {
    //                 break;
    //             }
    //             len
    //         }
    //         other => panic!("Some better error handling here... {:?}", other),
    //     };
    //     stdout.consume(len);
    // }
    // println!("Finished reading Python output.");

    // // Wait for the Python process to finish
    // let _ = python_process.wait()?;
    // println!("Python script completed.");

    Ok(())
}
