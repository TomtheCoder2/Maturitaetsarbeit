use com::commands::Command;

fn test(command: Command) {
    let encoded = command.encode();
    println!("Encoded: {:?}, len: {}", encoded, encoded.len());
    let decoded = Command::decode(&encoded).unwrap();

    match decoded {
        Command::Pos(val) => println!("Decoded Pos with value: {}", val),
        Command::SetPID(p, i, d) => println!("Decoded SetPID with values: {}, {}, {}", p, i, d),
        Command::SendPID(p, i, d) => println!("Decoded SendPID with values: {}, {}, {}", p, i, d),
        Command::Data(arr) => println!("Decoded Data with array: {:?}", arr),
        Command::Start => println!("Decoded Start"),
        Command::Stop => todo!(),
    }
    assert_eq!(command, decoded);
}

fn main() {
    test(Command::Pos(42));
    test(Command::SetPID(1, 2, 3));
    test(Command::SendPID(1.0, 2.0, 3.0));
    test(Command::Data([420; 1000]));
}
