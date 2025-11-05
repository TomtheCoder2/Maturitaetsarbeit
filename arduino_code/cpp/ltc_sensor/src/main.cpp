#include <Arduino.h>
#include "stepper.h"

// outside leads to ground and VCC
int val = 0; // variable to store the value read

stepper stepper1;

void setup() {
    // put your setup code here, to run once:
    Serial.begin(57600);
    Serial.println("Hello, Arduino!");
    // init
    stepper1.setup();
}

void loop() {
    if (Serial.available()) {
        String input = Serial.readStringUntil('\n');
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
        // check if number and then set stepper::target to that number
        if (input.toInt() != 0 || input == "0") {
            int input_int = input.toInt();
            stepper1.target_ltc_value = input_int;
            stepper1.integral = 0;
            // Serial.print("Set target_ltc_value to: ");
            // Serial.println(stepper1.target_ltc_value);
            return;
        }
    }
    stepper1.loop();
}
