const int enPin = 8;
const int stepXPin = 2;
const int dirXPin = 5;
const int microStepsRes = 4;
const int stepsPerRev = 400;            // 200 steps per rev with half-stepping = 400 microsteps per rev
const int maxPulseWidthMicros = 1000;   // Initial delay between steps (slower)
const int minMinPulseWidthMicros = 50;  // Minimum delay between steps (faster)
int minPulseWidthMicros = 50;           // Minimum delay between steps (faster)
const int acc = 15;
const int dec = 7;
int targetPosition = 100;
int currentPosition = 0;
int pulseWidthMicros = maxPulseWidthMicros;

// pid stuff
const int p = 0.2;
const int i = 0;
const int d = 0.1;
float error = 0;
float last_error = 0;
float integral = 0;
float speed = 0;


// dc stuff
int dc_target = 0;
// modes:
// 0 - Nothing/default
// shooting modes: first turn to 25, then -50 and then back to 0
// 1 - shoot: turning to 25
// 2 - shoot: turning to -50
// 3 - not yet defined
int dc_mode = 0;
// unsigned long count_reset = 0;
// unsigned long count_shoot = 0;
unsigned long start_reset = -100000;
unsigned long start_shoot = -1000000;
unsigned long shoot_intervall = 150;
unsigned long reset_intervall = 300;

int apos = 0;
int avout = 319;
int bpos = 340;
int bvout = 445;

int count = 0;
int checkPos = 0;

int posToVout(int pos) {
  float m = ((float)(bvout - avout)) / ((float)(bpos - apos));
  float c = (float)avout - m * (float)apos;
  return (int)((float)pos * m + c);
}

int voutToPos(float vout) {
  // Calculate the slope (m) for vout-to-pos
  float m = ((float)(bpos - apos)) / ((float)(bvout - avout));

  // Calculate the y-intercept (c) for vout-to-pos
  float c = (float)apos - m * avout;

  // Return the position for a given vout
  // return max(0, min(bpos, (int)((float)vout * m + c)));
  return (int)(m * vout + c);
}

int calcSteps(int normalTarget) {
  return normalTarget / 2 / microStepsRes * microStepsRes * microStepsRes;  // Parse the target position and round to the nearest whole step
}

void normal_new(int input_int) {
  input_int = max(0, min(bpos, input_int));
  minPulseWidthMicros = minMinPulseWidthMicros;
  // Serial.println(posToVout(input_int));
  int last_target = targetPosition;
  targetPosition = input_int / 2 / microStepsRes * microStepsRes * microStepsRes;  // Parse the target position and round to the nearest whole step
  int direction = (targetPosition > currentPosition) ? 1 : 0;
  int last_direction = (last_target > currentPosition) ? 1 : 0;
  if (direction != last_direction) {
    // decelerate
    while (pulseWidthMicros != maxPulseWidthMicros) {
      int direction = last_direction ? HIGH : LOW;
      digitalWrite(dirXPin, direction);
      pulseWidthMicros = min(maxPulseWidthMicros, pulseWidthMicros + acc);
      digitalWrite(enPin, LOW);
      // Step the motor for each half-step
      digitalWrite(stepXPin, HIGH);
      delayMicroseconds(pulseWidthMicros);
      digitalWrite(stepXPin, LOW);
      delayMicroseconds(pulseWidthMicros);

      currentPosition += last_direction ? 1 : -1;
    }
  }
  // Serial.print(F("Target updated: "));
  // Serial.println(targetPosition);
  // set checkPos to 2, so that it checks the pos after going there, that makes it more accurate
  checkPos = max(checkPos, 5);
}

void normal() {
  // ir_loop();

  // if (count > 1000) {
  // currentPosition = calcSteps(voutToPos(voutFast()));
  // count = 0;
  // }
  count++;
  if (count % 20 == 0) {
    // currentPosition = calcSteps(voutToPos(vout_n(10)));
    // currentPosition = voutToPos(vout_n(10)) * microStepsRes / 2;
  }
  if (currentPosition != targetPosition) {
    int direction = (targetPosition > currentPosition) ? HIGH : LOW;
    digitalWrite(dirXPin, direction);

    int stepsToTarget = abs(targetPosition - currentPosition);

    if (stepsToTarget > (maxPulseWidthMicros - pulseWidthMicros) / dec) {  // Accelerate to max speed
      pulseWidthMicros = max(minPulseWidthMicros, pulseWidthMicros - acc);
    } else {  // Decelerate as we approach the target
      pulseWidthMicros = min(maxPulseWidthMicros, pulseWidthMicros + dec);
    }

    if (maxPulseWidthMicros < minPulseWidthMicros) {
      pulseWidthMicros = minPulseWidthMicros;
    }

    digitalWrite(enPin, LOW);
    // Step the motor for each half-step
    digitalWrite(stepXPin, HIGH);
    delayMicroseconds(pulseWidthMicros);
    digitalWrite(stepXPin, LOW);
    delayMicroseconds(pulseWidthMicros);

    currentPosition += (direction == HIGH) ? 1 : -1;
    // Serial.print("pulseWidthMicros: ");
    // Serial.println(pulseWidthMicros);
  } else {
    digitalWrite(enPin, HIGH);
    if (checkPos > 0) {
      // Serial.println("reached pos, checking!... error:");
      currentPosition = voutToPos(vout_n(300)) * microStepsRes / 2;
      // Serial.println(targetPosition - currentPosition);
      // minPulseWidthMicros = 700;
      checkPos--;
    } else {
      minPulseWidthMicros = minMinPulseWidthMicros;
    }
  }
}

void run_motors(int pulse) {
  if (pulse > 1000) {
    digitalWrite(enPin, HIGH);
  }
  pulseWidthMicros = max(pulseWidthMicros - 5, pulse);
  digitalWrite(enPin, LOW);
  // Step the motor for each half-step
  digitalWrite(stepXPin, HIGH);
  delayMicroseconds(pulseWidthMicros);
  digitalWrite(stepXPin, LOW);
  delayMicroseconds(pulseWidthMicros);
}

void pid() {
  count++;
  if (count % 100 == 0) {
    error = targetPosition - (float)voutToPos(vout_n(100)) * (float)microStepsRes / (float)2;
    float dev = last_error - error;
    integral += error;
    float correction = p * error + i * integral + d * dev;
    speed = ((float)1e4 - abs(correction) * 2) / 10;
    digitalWrite(dirXPin, correction > 0);
    // Serial.print("error:");
    // Serial.println(error);
    // Serial.print("correction:");
    // Serial.println(correction);
    // Serial.print("pulseWidthMicros:");
    // Serial.println(pulseWidthMicros);
    // Serial.print("speed:");
    // Serial.println(speed);
    // Serial.println();
    last_error = error;
  }
  run_motors(speed);
}

void setup() {
  Serial.begin(57600);
  pinMode(stepXPin, OUTPUT);
  pinMode(dirXPin, OUTPUT);
  pinMode(enPin, OUTPUT);
  digitalWrite(enPin, LOW);
  delayMicroseconds(500 * 1000);
  digitalWrite(enPin, HIGH);
  delayMicroseconds(500 * 1000);
  digitalWrite(enPin, HIGH);
  dc_setup();
  stop();
  ir_setup();
  Serial.println(F("Ready for position commands"));
}

void loop() {
  if (Serial.available()) {
    String input = Serial.readStringUntil('\n');
    if (input == "R") {
      // delayMicroseconds(2 * 1000 * 1000);
      currentPosition = 0;
      targetPosition = 0;
      reset_dc_pos();
      return;
    }
    if (input == "S") {
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
    if (input == "dc pos") {
      Serial.print("dc pos:");
      Serial.println(get_pos());
      return;
    }
    if (input.startsWith("dc")) {
      input.replace("dc ", "");
      int input_int = input.toInt();
      dc_target = input_int;
      Serial.print("Set dc_target to: ");
      Serial.println(dc_target);
      return;
    }
    if (input.startsWith("si")) {
      input.replace("si ", "");
      int input_int = input.toInt();
      shoot_intervall = input_int;
      Serial.print("Set shoot_intervall to: ");
      Serial.println(shoot_intervall);
      return;
    }
    if (input.startsWith("sr")) {
      input.replace("sr ", "");
      int input_int = input.toInt();
      reset_intervall = input_int;
      Serial.print("Set reset_intervall to: ");
      Serial.println(reset_intervall);
      return;
    }
    if (input == "sync") {
      Serial.println("");
      Serial.println("end");
      return;
    }
    if (input == "I") {
      // ir_loop();
      float v = vout();
      Serial.print("Pos: ");
      Serial.println(voutToPos(v));
      // Serial.println(v);
      return;
    }
    if (input == "i") {
      ir_loop();
      float v = vout();
      Serial.print("Pos: ");
      Serial.println(voutToPos(v));
      Serial.println(v);
      return;
    }
    if (input == "IR") {
      // Infrared reset
      float v = vout();
      currentPosition = calcSteps(voutToPos(v));
      Serial.print("VOUT: ");
      Serial.println(v);
      // targetPosition = currentPosition;
      Serial.print("Pos: ");
      Serial.println(voutToPos(v));
      Serial.print("Target Pos: ");
      Serial.println(targetPosition);
      return;
    }
    if (input.startsWith("check")) {
      input.replace("check ", "");
      // Serial.print("Input: ");
      // Serial.println(input);
      int input_int = input.toInt();
      // Serial.print("Input int: ");
      // Serial.println(input_int);
      checkPos = input_int;
      return;
    }
    if (input == "print_vout") {
      print_vout();
      return;
    }
    if (input == "full_reset") {
      Serial.println("Full reset!");
      reset_dc_pos();
      float v = vout();
      // avout = (int)v;
      currentPosition = 0;
      targetPosition = bpos / 2 / microStepsRes * microStepsRes * microStepsRes;  // Parse the target position and round to the nearest whole step
      digitalWrite(enPin, LOW);
      pulseWidthMicros = maxPulseWidthMicros;
      while (currentPosition != targetPosition) {
        int direction = (targetPosition > currentPosition) ? HIGH : LOW;
        digitalWrite(dirXPin, direction);

        int stepsToTarget = abs(targetPosition - currentPosition);

        const int accReset = 4;
        if (stepsToTarget > (maxPulseWidthMicros - pulseWidthMicros) / accReset) {  // Accelerate to max speed
          pulseWidthMicros = max(minPulseWidthMicros, pulseWidthMicros - accReset);
        } else {  // Decelerate as we approach the target
          pulseWidthMicros = min(maxPulseWidthMicros, pulseWidthMicros + accReset);
        }
        digitalWrite(enPin, LOW);

        // Step the motor for each half-step
        digitalWrite(stepXPin, HIGH);
        delayMicroseconds(pulseWidthMicros);
        digitalWrite(stepXPin, LOW);
        delayMicroseconds(pulseWidthMicros);

        currentPosition += (direction == HIGH) ? 1 : -1;
      }
      digitalWrite(enPin, HIGH);
      delay(500);
      // we reached bpos
      v = vout();
      // bvout = (int)v;
      Serial.print("New Avout: ");
      Serial.println(avout);
      Serial.print("New Bvout: ");
      Serial.println(bvout);
      targetPosition = 0;
      Serial.println("end");
      return;
    }
    int input_int = input.toInt();
    normal_new(input_int);

    // pid
    // targetPosition = input_int;
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
  normal();
  // pid();
}
