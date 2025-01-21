// RS485 control pins
#define RE_PIN 12
#define DE_PIN 13

// Serial connection for RS485
#define RS485_SERIAL Serial2


// dc stuff
int dc_target = 0;
// modes:
// 0 - Nothing/default
// shooting modes: first turn to 25, then -50 and then back to 0
// 1 - shoot: turning to 25
// 2 - shoot: turning to -50
// 3 - not yet defined
int dc_mode = 0;
unsigned long start_reset = -100000;
unsigned long start_shoot = -1000000;
unsigned long shoot_intervall = 150;
unsigned long reset_intervall = 300;

int motor_speed = 0;

const long int max_pos = 20000;
int pos = 0;
int ir_target = 0;
int corrections = 0;
int reset_counter = 0;

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
    if (inputString[i] == '-') {
      continue;
    }
    if (!isDigit(inputString[i])) {
      return false;  // Return false if a non-digit character is found
    }
  }
  return true;  // All characters are digits
}

// ir to pos (0-330)
int ir_to_pos(int ir_pos) {
  float ir_pos_f = (float)ir_pos;
  // Serial.println(ir_pos_f);
  // Serial.println(a * ir_pos_f);
  return (int)(2.0447363529 * ir_pos_f + -590.1313588202);
}

// pos (0-330) to ir
int pos_to_ir(int pos) {
  float pos_f = (float)pos;
  // Serial.println(pos_f);
  // Serial.println(0283.5737179f * pos_f);
  return (int)((245.6969697 * pos_f) / 1000.0f + 363.99f);
}

void set_speed(int speed) {
  if (speed != motor_speed) {
    motor_speed = speed;
    sendCommand(0x01, 5, 0x04, speed);  // Maximum positioning speed
    delay(30);
  }
}


void setup() {
  Serial.begin(57600);
  Serial.println("Begin...");
  ir_setup();
  // Configure control pins
  pinMode(RE_PIN, OUTPUT);
  pinMode(DE_PIN, OUTPUT);

  // Start RS485 in receive mode
  digitalWrite(RE_PIN, LOW);  // Receiver enabled
  digitalWrite(DE_PIN, LOW);  // Driver disabled

  // Initialize serial communication
  RS485_SERIAL.begin(9600);  // Adjust baud rate if needed for the motor
  dc_setup();

  // Test command to rotate motor right
  // sendCommand(0x01, 0x01, 0x00, 1000); // ROR (Rotate Right) with velocity 1000
  sendCommand(0x01, 6, 208, 0x0);
  delay(1);
  read_output();


  sendCommand(0x01, 5, 0x04, 2047);  // Maximum positioning speed
  delay(100);
  sendCommand(0x01, 5, 0x05, 2047);  // Maximum acceleration
  delay(100);
  sendCommand(0x01, 5, 0x8c, 7);  // microstep res
  delay(100);
  sendCommand(0x01, 5, 153, 6);  // ramp divisor
  delay(100);
  sendCommand(0x01, 5, 154, 3);  // pulse divisor
  delay(100);
  sendCommand(0x01, 6, 208, 0x0);
  delay(100);
}

void loop() {
  if (Serial.available()) {
    String input = Serial.readStringUntil('\n');
    while (Serial.available()) {
      input = Serial.readStringUntil('\n');
    }
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
    if (input == "S") {
      if (dc_mode != 0) {
        return;
      }
      // shoot
      dc_target = 20;
      dc_mode = 1;
      return;
    }
    if (input == "reset_dc") {
      unsigned long int start = millis();
      while (millis() - start < 2000) {
        turn(1);
      }
      stop();
      reset_dc_pos();
      while (!dc_loop(-35)) {
      }
      reset_dc_pos();
      Serial.println("Reset dc motor");
      return;
    }
    if (input == "I") {
      Serial.print("Pos: ");
      Serial.println(ir_to_pos(vout()));
      // Serial.println("end");
      return;
    }
    if (input == "ir") {
      Serial.print("IR: ");
      // print_vout();
      Serial.print(vout());
      Serial.print(" Pos: ");
      Serial.println(ir_to_pos(vout()));
      return;
    }
    if (input == "vi") {
      print_vout();
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
      // pos = clamp(input.toInt(), 0, 330);
      // int ir_pos = pos_to_ir(pos);
      // ir_target = ir_pos;
      // Serial.print("Ir Pos: ");
      // Serial.print(ir_pos);
      // pos = ir_to_pos(ir_pos);
      // Serial.print(" Normal Pos: ");
      // Serial.println(pos);

      // Serial.print("IR Pos: "); Serial.println(current_pos);
      // pos = 100, current_pos = 120, correction = -20
      // int correction = pos - current_pos;
      // int drive_pos = pos + correction;
      // pos = current_pos;
      // long long int input_int = clamp(convert(drive_pos), 0, max_pos);
      reset_counter--;
      int ir = vout_int();
      int current_pos = ir_to_pos(ir);
      if (reset_counter < 0) {
        // sendCommand(0x01, 5, 1, -convert(current_pos));  // set actual position
        // sendCommand(0x01, 5, 1, -convert(pos));  // set actual position
        // delay(50);
        reset_counter = 1;
      }
      pos = input.toInt();
      set_speed(2047);  // Maximum positioning speed
      

      // long long int input_int = clamp(convert(input.toInt()), 0, max_pos);
      long long int input_int = convert((int)(0.8 * (float)input.toInt()));
      // Serial.print("pos: "); Serial.println((long)input_int); 
      // sendCommand(0x01, 0x04, 0x00, -input_int);
      // delay(100);
      corrections = 50;
    }
  }
  if (corrections > 0) {
    int ir = vout_100();
    int current_pos = ir_to_pos(ir);
    // pos = 100, current_pos = 120, correction = -20
    int correction = (pos - current_pos) * 3 / 4;
    if (abs(correction) > 1) {
      // int drive_pos = pos + correction;
      // pos = drive_pos;
      long long int input_int = convert(pos);
      // Serial.print("pos: ");
      // Serial.println((long)current_pos);
      // sendCommand(0x01, 0x04, 0x00, -input_int);
      // sendCommand(0x01, 5, 1, -convert(current_pos));  // set actual position
      // delay(100);
      // sendCommand(0x01, 0x04, 0x00, -input_int);
      // delay(20);
      if (abs(correction) < 30) {
        set_speed(200);
        correction = (pos - current_pos) * 3 / 10;
      }
      // sendCommand(0x01, 5, 0x8c, 10);  // microstep res
      // delay(20);
      sendCommand(0x01, 0x04, 0x01, -convert(correction));
      // delay(20);
      corrections--;
    }
  }

  int dc_pos = get_pos();
  // if (count_shoot > 1 && dc_pos > -50) {
  if (millis() - start_shoot < shoot_intervall && dc_pos > -50) {
    // Serial.print("dc pos:");Serial.println(dc_pos);
    // count_shoot--;
    turn(-1);
    dc_target = 0;
    dc_mode = 2;
    // count_reset = 10000;
    start_reset = millis();
    // } else if (count_reset > 1) {
  } else if (millis() - start_reset < reset_intervall) {
    // Serial.println("resetting");
    turn(1);
    dc_mode = 3;
    dc_target = -35;
    reset_dc_pos();
    // count_reset--;
  } else {
    if (dc_loop(dc_target)) {
      if (dc_mode == 1) {
        // count_shoot = 10000;
        start_shoot = millis();
      } else if (dc_mode == 3) {
        // Serial.println("reseting dc pos");
        reset_dc_pos();
        dc_mode = 0;
        dc_target = 0;
      }
    }
  }


  // read_output();




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