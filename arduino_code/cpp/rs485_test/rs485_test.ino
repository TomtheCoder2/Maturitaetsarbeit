// RS485 control pins
#define RE_PIN 12
#define DE_PIN 13

// Serial connection for RS485
#define RS485_SERIAL Serial2


const long int max_pos = 20000;
int pos = 0;

int clamp(long int value, long int min, long int max) {
  return min(max, max(min, value));
}

int convert(long int o_pos) {
  return o_pos * max_pos / 330;
}

bool isNumber(String inputString) {
  // Check if the string is empty
  if (inputString.length() == 0) {
    return false;
  }
  // Check each character to ensure it's a digit
  for (unsigned int i = 0; i < inputString.length(); i++) {
    if (!isDigit(inputString[i])) {
      return false;  // Return false if a non-digit character is found
    }
  }
  return true;  // All characters are digits
}


void setup() {
  Serial.begin(57600);
  Serial.println("Begin...");
  // Configure control pins
  pinMode(RE_PIN, OUTPUT);
  pinMode(DE_PIN, OUTPUT);

  // Start RS485 in receive mode
  digitalWrite(RE_PIN, LOW);  // Receiver enabled
  digitalWrite(DE_PIN, LOW);  // Driver disabled

  // Initialize serial communication
  RS485_SERIAL.begin(9600);  // Adjust baud rate if needed for the motor


  // Test command to rotate motor right
  // sendCommand(0x01, 0x01, 0x00, 1000); // ROR (Rotate Right) with velocity 1000
  sendCommand(0x01, 6, 208, 0x0);
  delay(1);
  read_output();


  sendCommand(0x01, 5, 0x04, 2047);  // abs  target acc set
  delay(100);
  sendCommand(0x01, 5, 0x05, 2047);  // abs  acc acc set
  delay(100);
  sendCommand(0x01, 5, 0x8c, 7);     // microstep res
  delay(100);
  sendCommand(0x01, 5, 153, 6);      // ramp divisor
  delay(100);
  sendCommand(0x01, 5, 154, 3);      // pulse divisor
  delay(100);
  sendCommand(0x01, 6, 208, 0x0);
}

void loop() {
  if (Serial.available()) {
    String input = Serial.readStringUntil('\n');
    if (input == "e") {
      sendCommand(0x01, 5, 0x04, 2047);  // abs  target speed set
      delay(100);
      return;
    }
    if (input.startsWith("p")) {
      input.replace("p ", "");
      long long int input_int = input.toInt();
      sendCommand(0x01, 0x04, 0x00, -input_int);
      return;
    }
    if (input == "I") {
      Serial.print("Pos: ");
      Serial.println(pos);
      // Serial.println("end");
      return;
    }
    if (input == "sync") {
      Serial.println("");
      Serial.println("end");
      return;
    }
    if (input == "R" || input == "full_reset" || input == "r") {
      Serial.println("Reset!...");
      sendCommand(0x01, 0x04, 0x01, 1000);
      delay(1000);
      Serial.println("Reset...");
      sendCommand(0x01, 5, 0x01, 0);
      Serial.println("Reset done");
      delay(2000);
      sendCommand(0x01, 0x04, 0x00, -10000);
      Serial.println("end");
    }
    if (isNumber(input)) {
      // Serial.println("number!");
      pos = input.toInt();
      long long int input_int = clamp(convert(input.toInt()), 0, max_pos);
      // Serial.print("pos: "); Serial.println((long)input_int);
      sendCommand(0x01, 0x04, 0x00, -input_int);
    }
  }

  read_output();




  // Example: Move motor to position 10000 (absolute)
  // delay(2000);  // Wait 2 seconds
  // sendCommand(0x01, 0x04, 0x01, 10000); // MVP (Move to Position) absolute position 10000

  // delay(2000);  // Wait 2 seconds
  // sendCommand(0x01, 0x04, 0x01, -10000); // MVP (Move to Position) absolute position 10000

  // sendCommand(0x01, 208, 0x00, 0x0);

  // 01 04 01 00 00 00 27 10 3D
  //  1  4  1  0  0  0 27 10 3D

  // Example: Stop motor
  // delay(2000); // Wait 2 seconds
  // sendCommand(0x01, 0x03, 0x00, 0); // MST (Motor Stop)
}

void sendCommand(byte address, byte command, byte type, int32_t value) {
  // Enable transmission mode
  digitalWrite(RE_PIN, HIGH);
  digitalWrite(DE_PIN, HIGH);

  // Build command frame
  byte frame[9];
  frame[0] = address;  // Module address
  frame[1] = command;  // Command number
  frame[2] = type;     // Type (e.g., motor number or parameter type)
  frame[3] = 0x0;
  frame[4] = (value >> 24) & 0xFF;  // Value MSB
  frame[5] = (value >> 16) & 0xFF;
  frame[6] = (value >> 8) & 0xFF;
  frame[7] = value & 0xFF;                 // Value LSB
  frame[8] = calculateChecksum(frame, 8);  // Checksum

  // Debug: Print the command frame
  // Serial.print("Sending Command: ");
  // for (int i = 0; i < 9; i++) {
  //   Serial.print(frame[i], HEX);
  //   Serial.print(" ");
  // }
  // Serial.println();

  // Send command frame
  RS485_SERIAL.write(frame, 9);

  // Wait for data to be sent
  RS485_SERIAL.flush();
  delay(1);

  // Switch back to receive mode
  digitalWrite(RE_PIN, LOW);
  digitalWrite(DE_PIN, LOW);

  // // Optional: Read response (if needed)
  // delay(100);  // Allow some time for a response
  // while (RS485_SERIAL.available()) {
  //   char c = RS485_SERIAL.read();
  //   Serial.print(c, HEX);
  //   Serial.print(" ");
  // }
  // Serial.println();
}

void read_output() {
  // RS485_SERIAL.end();
  bool data = false;
  while (RS485_SERIAL.available()) {
    char c = RS485_SERIAL.read();
    Serial.print(c, HEX);
    Serial.print(" ");
    data = true;
  }
  if (data) {
    Serial.println();
  }
}

byte calculateChecksum(byte* frame, int length) {
  byte checksum = 0;
  for (int i = 0; i < length; i++) {
    checksum += frame[i];
  }
  return checksum;
}