use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

pub struct PythonScript {
    process: std::process::Child,
}

impl PythonScript {
    pub fn new() -> Self {
        let python_process = Command::new("python")
            .arg("./python_code_ma/tf_detect_player.py") // Python script to read from memory-mapped region
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
            // println!("Got stdin.");
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
                                .map(|s| s.parse::<i32>().unwrap())
                                .collect::<Vec<i32>>();
                            // println!("Received coordinates: {:?}", cs);
                            coords = (cs[0].max(0) as u32, cs[1].max(0) as u32);
                            brake = true;
                            break;
                        }

                        // Check if the line contains the "Done" message
                        if line.trim().contains("Done") {
                            // println!("Received 'Done' from Python. Continuing in Rust...");
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
