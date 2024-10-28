use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Duration;

use com::commands::Command;
use serialport::prelude::*;
use std::thread::sleep;
fn send_command(serial: &mut Box<dyn SerialPort>, command: Command) {
    let encoded = command.encode();
    let length = Command::length_c(&command);
    // filter out all 0s
    // let encoded = encoded
    // .iter()
    // .filter(|x| **x != 0)
    // .map(|x| *x)
    // .collect::<Vec<u8>>();
    let encoded = encoded.to_vec()[0..length].to_vec();
    println!("Encoded: {:?}", encoded);
    println!("length: {}, len: {}", length, encoded.len());
    serial.write_all(&encoded).expect("Failed to write to port");
}

fn receive_command(serial: &mut Box<dyn SerialPort>) -> Command {
    let mut buffer = [0; 1];
    serial
        .read_exact(&mut buffer)
        .expect("Failed to read from port");
    let ty = buffer[0];
    println!("ty: {:?}", ty);
    let total_length = Command::length(ty);
    println!("Total length: {}", total_length);
    let mut buffer = vec![0; total_length - 1];
    serial
        .read_exact(&mut buffer)
        .expect("Failed to read from port");
    let mut data = &mut [ty].to_vec();
    data.extend(&buffer);
    // println!("Data: {:?}", data.iter().map(|x| format!("{:02x}", x)).collect::<Vec<String>>());
    Command::decode(&data).unwrap()
}

fn main() -> std::io::Result<()> {
    // encoding tests
    let mut text = "text";
    let mut encoded = b"text";
    let mut encoded = encoded.to_vec();
    encoded.push(0xff);
    println!("Encoded: {:?}", encoded);
    println!("Decoded: {:?}", String::from_utf8_lossy(&encoded.to_vec()));

    // list all ports and their names
    let ports = serialport::available_ports()?;
    for port in ports {
        println!("{:?}", port);
    }

    // Configure the serial port
    let port_name = "COM4"; // Replace with the correct port for your Arduino
    let baud_rate = 57600;

    let serial_settings = SerialPortSettings {
        baud_rate,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_secs(100),
        ..Default::default()
    };

    let mut port =
        serialport::open_with_settings(port_name, &serial_settings).expect("Failed to open port");
    // let command = b'a';  // Example command
    // port.write_all(&[command]).expect("Failed to write to port");
    // drop(port);
    send_command(&mut port, Command::Start);
    println!("start command sent");
    // wait for the arduino to start
    let c = receive_command(&mut port);
    println!("Received command: {:?}", c);

    sleep(Duration::from_secs(1));
    loop {
        // send_command(&mut port, Command::Start);
        send_command(&mut port, Command::Pos(50));
        println!("pos 50 command sent");
        sleep(Duration::from_secs(3));
        // let c = receive_command(&mut port);
        // println!("Received command: {:?}", c);
        send_command(&mut port, Command::Pos(0));
        println!("pos -50 command sent");
        sleep(Duration::from_secs(3));
        // let c = receive_command(&mut port);
        // println!("Received command: {:?}", c);
    }

    // let mut port = serialport::open_with_settings(port_name, &serial_settings)
    //     .expect("Failed to open port");

    // let mut file = File::create("./output.csv")?;
    // file.write_all(b"Pos\n")?;

    // let mut buffer = [0; 1];
    // let mut data = "".to_string();

    // loop {
    //     let command = receive_command(&mut port);
    //     match command {
    //         Command::Data(arr) => {
    //             println!("Received data command: {:?}", arr);
    //             for val in arr.iter() {
    //                 file.write_all(format!("{}\n", val).as_bytes())?;
    //             }
    //             // file.write_all(b"\n")?;
    //             file.flush()?;
    //             // println!("Data written to file: path: {}", file.path().display());
    //         }
    //         Command::Stop => {
    //             println!("Received stop command");
    //             break;
    //         }
    //         _ => {
    //             println!("Received command: {:?}", command);
    //         }
    //     }
    //     data.push_str(&String::from_utf8((&buffer).to_vec()).unwrap());
    //     // println!("Data: {}", data);
    // }

    // // port.write_all(&[b's']).expect("Failed to write to port");

    Ok(())
}
