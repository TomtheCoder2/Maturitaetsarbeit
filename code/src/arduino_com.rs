use std::io::Write;
use std::time::Duration;

// use com::commands::Command;
use serialport::prelude::*;
use std::fmt::{Debug, Formatter};
use std::thread::sleep;
// Make sure this is imported

pub struct ArduinoCom {
    serial: Box<dyn SerialPort>,
    last_command: String,
}

impl Debug for ArduinoCom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArduinoCom")
    }
}

impl Default for ArduinoCom {
    fn default() -> Self {
        Self::new()
    }
}

impl ArduinoCom {
    pub fn new() -> Self {
        // list all ports and their names
        let ports = serialport::available_ports().expect("Failed to list ports");
        let mut port_name = "/dev/tty.usbmodem1401".to_string(); // Default, will be overwritten if Arduino found
        for port in ports {
            println!("{:?}", port);
            if format!("{:?}", port.port_type).contains("Arduino")
                || port.port_name.contains("usbmodem")
            {
                println!("Arduino found on port: {:?}", port.port_name);
                port_name = port.port_name;
            }
        }

        // Configure the serial port
        let baud_rate = 57600;

        let serial_settings = SerialPortSettings {
            baud_rate,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None, // Keep this as None
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(1000), // Keep this generous
        };

        let port = serialport::open_with_settings(&port_name, &serial_settings)
            .unwrap_or_else(|_| panic!("Failed to open port: {}", port_name));

        let arduino_com = ArduinoCom {
            serial: port,
            last_command: "".to_string(),
        };

        // *** ADD THIS CRUCIAL DELAY ***
        // Give the Arduino time to reset and boot up after the serial port is opened.
        // 2-3 seconds is a good starting point. You can reduce it if it becomes stable.
        println!("Waiting for Arduino to boot after connection...");
        sleep(Duration::from_millis(2000)); // Wait for 3 seconds

        // Optional: Clear any junk that might have been buffered during Arduino boot.
        // This consumes any bytes sent before your main read loop starts.
        // let _ = arduino_com.read_everything();
        println!("Arduino communication initialized and buffer cleared.");

        arduino_com
    }

    pub fn send_string(&mut self, s: &str) {
        if s == self.last_command && s != "sync" {
            return;
        }
        writeln!(self.serial, "{}", s).expect("Failed to write to port");
        self.serial.flush().expect("Failed to flush port");
        // println!("Sent: {}", s);
        self.last_command = s.to_string(); // Update last_command
    }

    pub fn force_send_string(&mut self, s: &str) {
        writeln!(self.serial, "{}", s).expect("Failed to write to port");
        self.serial.flush().expect("Failed to flush port");
        // println!("Sent: {}", s);
    }

    pub fn read_line(&mut self) -> String {
        // read until \n
        let mut buffer = String::new();
        loop {
            let mut buf = [0; 1];
            self.serial
                .read_exact(&mut buf)
                .expect("Failed to read from port");
            let c = buf[0] as char;
            if c == '\n' {
                break;
            }
            buffer.push(c);
        }
        buffer
    }

    pub fn read_line_option(&mut self) -> Option<String> {
        // read until \n
        let mut buffer = String::new();
        loop {
            let mut buf = [0; 1];
            self.serial.read_exact(&mut buf).ok()?;
            let c = buf[0] as char;
            if c == '\n' {
                break;
            }
            buffer.push(c);
        }
        Some(buffer)
    }

    pub fn read_everything(&mut self) -> String {
        let mut buffer = String::new();
        loop {
            let mut buf = [0; 1];
            // Use a non-blocking read if possible, or expect timeout after some data.
            // For clearing buffer, read_exact will timeout if no more data.
            // A common pattern here is to use read() instead of read_exact() if you want to
            // read whatever is available and then break if nothing is left.
            // The current read_exact will wait for at least one byte within timeout,
            // so if nothing is there, it will timeout. This is fine for clearing initial junk.
            let res = self.serial.read_exact(&mut buf);
            if res.is_err() {
                // This means either timeout or actual read error, assuming it's timeout
                break;
            }
            let c = buf[0] as char;
            buffer.push(c);
        }
        buffer
    }

    pub fn sync(&mut self) {
        self.send_string("sync");
        // println!("Sent 'sync', waiting for response...");

        // Give Arduino a moment to process the command and send its response
        sleep(Duration::from_millis(10));

        let mut output = "".to_string();
        while !output.starts_with("end") {
            match self.read_line() {
                line if !line.is_empty() => {
                    output = line.trim().to_string(); // Trim whitespace just in case
                                                      // println!("SYNC DEBUG: Received: '{}'", output);
                }
                _ => {
                    // println!("SYNC DEBUG: Received an empty line (likely just \\r\\n)");
                    // If you continuously get empty lines and "end" isn't showing,
                    // there might be a problem with the Arduino's output.
                    // Consider adding a safety counter to prevent infinite loops.
                }
            }
        }
        // println!("Finished syncing");
    }

    pub fn get_pos(&mut self) -> f32 {
        self.get_pos_sync(true)
    }

    pub fn get_pos_sync(&mut self, sync: bool) -> f32 {
        self.get_value(sync, "I")
    }

    pub fn get_pulse_width(&mut self) -> f32 {
        self.get_value(true, "pw")
    }

    fn get_value(&mut self, sync: bool, command: &str) -> f32 {
        if sync {
            self.sync();
        }
        self.send_string(command);
        sleep(Duration::from_millis(5));
        let output = self.read_line();
        let prefix = format!("{}:", command);
        assert!(output.starts_with(&prefix));
        let value = output
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<f32>()
            .unwrap();
        value
    }

    pub fn set_pid(&mut self, p: f32, i: f32, d: f32) {
        self.send_string(&format!("pid {} {} {}", p, i, d));
    }
}
