use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Duration;

use com::commands::Command;
use serialport::prelude::*;
use std::fmt::{Debug, Formatter};
use std::thread::sleep;

pub struct ArduinoCom {
    serial: Box<dyn SerialPort>,
    last_command: String
}

impl Debug for ArduinoCom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArduinoCom")
    }
}

impl ArduinoCom {
    pub fn new() -> Self {
        // list all ports and their names
        let ports = serialport::available_ports().expect("Failed to list ports");
        let mut port_name = "COM4".to_string();
        for port in ports {
            println!("{:?}", port);
            if format!("{:?}", port.port_type).contains("Arduino") {
                println!("Arduino found on port: {:?}", port.port_name);
                port_name = port.port_name;
            }
        }

        // Configure the serial port
        let baud_rate = 57600;

        let serial_settings = SerialPortSettings {
            baud_rate,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::Hardware,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_micros(10),
            ..Default::default()
        };

        let mut port = serialport::open_with_settings(&port_name, &serial_settings)
            .expect("Failed to open port");
        // send start command and wait for response
        let mut arduino_com = ArduinoCom { serial: port, last_command: "".to_string() };
        // arduino_com.send_command(Command::Start);
        // println!("start command sent");
        // let c = arduino_com.receive_command();
        // println!("Received command: {:?}", c);
        // arduino_com.send_string("R");
        arduino_com
    }

    pub fn send_command(&mut self, command: Command) {
        let encoded = command.encode();
        let length = Command::length_c(&command);
        // filter out all 0s
        // let encoded = encoded
        // .iter()
        // .filter(|x| **x != 0)
        // .map(|x| *x)
        // .collect::<Vec<u8>>();
        let encoded = encoded.to_vec()[0..length].to_vec();
        // println!("Encoded: {:?}", encoded);
        // println!("length: {}, len: {}", length, encoded.len());
        self.serial
            .write_all(&encoded)
            .expect("Failed to write to port");
    }

    pub fn send_stepper_motor_pos(&mut self, pos: i32) {
        let command = Command::Pos(pos);
        self.send_command(command);
    }

    pub fn send_stepper_motor_speed(&mut self, speed: i32) {
        let command = Command::Speed(speed);
        self.send_command(command);
    }

    pub fn receive_command(&mut self) -> Command {
        let mut buffer = [0; 1];
        self.serial
            .read_exact(&mut buffer)
            .expect("Failed to read from port");
        let ty = buffer[0];
        println!("ty: {:?}", ty);
        let total_length = Command::length(ty);
        println!("Total length: {}", total_length);
        let mut buffer = vec![0; total_length - 1];
        self.serial
            .read_exact(&mut buffer)
            .expect("Failed to read from port");
        let mut data = &mut [ty].to_vec();
        data.extend(&buffer);
        // println!("Data: {:?}", data.iter().map(|x| format!("{:02x}", x)).collect::<Vec<String>>());
        Command::decode(&data).unwrap()
    }

    pub fn try_receive_command(&mut self) -> Option<Command> {
        let mut buffer = [0; 1];
        let res = self.serial.read(&mut buffer);
        if res.is_err() {
            return None;
        }
        let ty = buffer[0];
        let total_length = Command::length(ty);
        let mut buffer = vec![0; total_length - 1];
        let res = self.serial.read(&mut buffer);
        if res.is_err() {
            return None;
        }
        let mut data = &mut [ty].to_vec();
        data.extend(&buffer);
        Command::decode(&data)
    }

    pub fn send_string(&mut self, s: &str) {
        if s == self.last_command {
            return;
        }
        writeln!(self.serial, "{}", s).expect("Failed to write to port");
        self.serial.flush().expect("Failed to flush port");
        // println!("Sent: {}", s);
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
            self.serial.read_exact(&mut buf).expect("Failed to read from port");
            let c = buf[0] as char;
            if c == '\n' {
                break;
            }
            buffer.push(c);
        }
        buffer
    }

    pub fn read_everything(&mut self) -> String {
        let mut buffer = String::new();
        loop {
            let mut buf = [0; 1];
            let res = self.serial.read(&mut buf);
            if res.is_err() {
                break;
            }
            let c = buf[0] as char;
            buffer.push(c);
        }
        buffer
    }

    pub fn sync(&mut self) {
        self.send_string("sync");
        let mut output = "".to_string();
        while !output.starts_with("end") {
            output = self.read_line();
            // println!("f{:?}f", output.chars//().collect::<Vec<char>>());
            println!("sync o: {}", output);
        }
        println!("Finished syncing");
    }
}
