//
// Created by Jan Wilhelm on 02.11.2025.
//

#include "stepper.h"

#include <Arduino.h>

void stepper::setup() {
    pinMode(stepXPin, OUTPUT);
    pinMode(dirXPin, OUTPUT);
    pinMode(enPin, OUTPUT);
    current_ltc_value = analogRead(ltc_analogPin);
}

void stepper::loop() {
    // read analog pin A15 and print its value
    current_ltc_value = analogRead(ltc_analogPin);  // read the input pin
    float error = (float)(target_ltc_value - current_ltc_value);
    // lets print the error if its less than 20
    if (abs(error) < 20) {
        // Serial.print("ltc: ");
        // Serial.println(current_ltc_value);
    }
    float delta = error - last_error;
    integral += error;
    float control_signal = p * error + i * integral + d * delta;
    last_error = error;
    // Serial.print("Control signal: ");Serial.println(control_signal);
    if (abs(control_signal) > 0.00 && abs(error) > minError) {
        int direction = (control_signal < 0) ? HIGH : LOW;
        step(direction, min(abs(control_signal) / 0.2, 1.0));
        // step(direction, 0.5);
    } else if (abs(error) <= minError) {
        // turn off motor
        digitalWrite(enPin, HIGH);
        integral = 0;
    }
}

// lets define speed as a float between 0 and 1, 0 meaning largest pulseWidth and 1 meaning smallest pulseWidth
void stepper::step(int direction, float speed) {
    int targetPulseWidth = map(speed * 1000, 0, 1000, maxPulseWidth, minPulseWidth);
    if (direction == current_direction) {
        if (pulseWidth < targetPulseWidth) {
            pulseWidth = min(pulseWidth + 10, targetPulseWidth);
        } else if (pulseWidth > targetPulseWidth) {
            pulseWidth = max(pulseWidth - 10, targetPulseWidth);
        }
    } else {
        current_direction = direction;
        pulseWidth = maxPulseWidth;
    }
    digitalWrite(dirXPin, direction);

    digitalWrite(enPin, LOW);
    // Step the motor for each half-step
    digitalWrite(stepXPin, HIGH);
    delayMicroseconds(pulseWidth);
    digitalWrite(stepXPin, LOW);
    delayMicroseconds(pulseWidth);
}
