use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Duration;

use serialport::prelude::*;

fn main() -> std::io::Result<()> {
    // encoding tests
    let mut text = "text";
    let mut encoded = b"text";
    let mut encoded = encoded.to_vec();
    encoded.push(0xff);
    println!("Encoded: {:?}", encoded);
    println!("Decoded: {:?}", String::from_utf8_lossy(&encoded.to_vec()));


    // Configure the serial port
    let port_name = "COM5"; // Replace with the correct port for your Arduino
    let baud_rate = 9600;

    let serial_settings = SerialPortSettings {
        baud_rate,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_secs(1),
        ..Default::default()
    };

    let mut port = serialport::open_with_settings(port_name, &serial_settings)
        .expect("Failed to open port");
    let command = b'a';  // Example command
    port.write_all(&[command]).expect("Failed to write to port");
    drop(port);
    println!("Command sent");
    let mut port = serialport::open_with_settings(port_name, &serial_settings)
        .expect("Failed to open port");

    let file = File::create("output.txt")?;
    let mut writer = BufWriter::new(file);

    let mut buffer: Vec<u8> = vec![0; 1024];
    let mut received_data: Vec<u8> = Vec::new();
    let mut opos = 0;
    let mut received_data = Vec::new();

    loop {
        match port.read(buffer.as_mut_slice()) {
            Ok(bytes_read) => {
                received_data.extend_from_slice(&buffer[..bytes_read]);
                println!("Received: {:?}", String::from_utf8(received_data.clone()));
                if let Some(pos) = received_data.windows(3).position(|window| window == b"END") {
                    writer.write_all(&received_data[..pos])?;
                    println!("Writing {} to file", String::from_utf8_lossy(&received_data[..pos]));
                    opos = pos;
                    break;
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => break,
            Err(e) => {
                eprintln!("Error reading from serial port: {:?}", e);
                break;
            }
        }
    }
    let mut final_data = "".to_string();
    let mut acccumaltor = vec![];
    for data in received_data {
        if data != 0xff {
            acccumaltor.push(data);
        } else {
            final_data.push_str(&*String::from_utf8_lossy(&acccumaltor));
            acccumaltor = vec![];
        }
    }
    // received_data = received_data.into_iter().filter(|&x| x != 0xff).collect::<Vec<u8>>();
    println!("result: {}", final_data);

    Ok(())
}
