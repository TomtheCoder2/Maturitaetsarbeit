#include <Arduino.h>
#include "stepper.h"
#include "dc_motor.h"

#define ENCA 19  // YELLOW
#define ENCB 26  // WHITE
#define PWM 44
#define IN2 50
#define IN1 51

#define DEBUG false

// outside leads to ground and VCC
int val = 0; // variable to store the value read

stepper stepper1;
DcMotor dc_motor(ENCA, ENCB, PWM, IN1, IN2);

#ifdef DEBUG
// debug stuff
int counter = 0;
#endif

void setup() {
    // put your setup code here, to run once:
    Serial.begin(57600);
    Serial.println("Startng Arduino controller...");
    // init
    stepper1.setup();
    dc_motor.setup();
}

void loop() {
    if (Serial.available()) {
        String input = Serial.readStringUntil('\n');
        // replace '\r' with ""
        input.replace("\r", "");
        if (input == "sync") {
            Serial.println("");
            Serial.println("end");
            return;
        }
        if (input == "I") {
            // ir_loop();
            float v = analogRead(ltc_analogPin);
            Serial.print("I: ");
            Serial.println(v);
            // Serial.println(v);
            return;
        }
        // get pulseWidth
        if (input == "pw") {
            Serial.print("pw: ");
            Serial.println(stepper1.pulseWidth);
            return;
        }
        // shoot
        if (input == "S") {
            dc_motor.shoot();
            return;
        }
        // dc pos
        if (input.startsWith("dc")) {
            int input_int = input.substring(3).toInt();
            dc_motor.target = input_int;
            return;
        }
        // reset_dc
        if (input == "reset_dc") {
            DcMotor::resetPosition();
            // Serial.println("Reset dc motor position to 0");
            return;
        }
        // pid set:
        // eg. pid 4 3 2
        if (input.startsWith("pid")) {
            input.replace("pid ", "");
            int firstSpace = input.indexOf(' ');
            int secondSpace = input.indexOf(' ', firstSpace + 1);
            float p = input.substring(0, firstSpace).toFloat();
            float i = input.substring(firstSpace + 1, secondSpace).toFloat();
            float d = input.substring(secondSpace + 1).toFloat();
            stepper1.p = p;
            stepper1.i = i;
            stepper1.d = d;
            return;
        }
        // check if number and then set stepper::target to that number
        if (input.toInt() != 0 || input == "0") {
            int input_int = input.toInt();
            stepper1.set_target(input_int);
            // Serial.print("Set target_ltc_value to: ");
            // Serial.println(stepper1.target_ltc_value);
            return;
        }
    }
    stepper1.loop();
    dc_motor.loop();

    if (DEBUG) {
        if (counter % 1000 == 0) {
            Serial.print("Stepper pos: ");
            Serial.print(stepper1.current_ltc_value);
            Serial.print(" | DcMotor pos: ");
            Serial.println(dc_motor.getPosition());
        }
    }

    counter++;
}
