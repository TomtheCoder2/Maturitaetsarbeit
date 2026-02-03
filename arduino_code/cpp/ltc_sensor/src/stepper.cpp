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
    // step();
    // if (last_loop_time_us + PID_LOOP_INTERVAL_US > micros()) {
    //     return;
    // }
    // last_loop_time_us = micros();
    // read analog pin A15 and print its value
    current_ltc_value = analogRead(ltc_analogPin); // read the input pin
    const float error = (float) (target_ltc_value - current_ltc_value);
    const float delta = error - last_error;
    integral += error;
    // Clamp the integral term
    if (integral > integral_max) {
        integral = integral_max;
    } else if (integral < -integral_max) {
        integral = -integral_max;
    }
    float p_gain, i_gain, d_gain;
    // adapted minPulseWidth and pid based on error
    if (abs(error) > 30) {
        // Large error: use aggressive gains
        p_gain = p * 10;
        i_gain = i;
        d_gain = d * 3;
        a_minPulseWidth = minPulseWidth;
    } else {
        // Small error: use conservative gains for stability
        p_gain = p;
        i_gain = 0;
        d_gain = d * 1.5;
        a_minPulseWidth = minSlowPulseWidth;
    }
    float control_signal = p_gain * error + i_gain * integral + d_gain * delta;
    last_error = error;
    // Serial.print("Control signal: ");Serial.println(control_signal);
    if (abs(control_signal) > 0.00 && abs(error) > minError) {
        // if signal is positive, move in LOW direction, else HIGH direction
        direction = (control_signal > 0) ? LOW : HIGH;
        target_speed = min(abs(control_signal) / 0.2, 1.0);
        for (int i = 0; i < 5; i++)
            step(true);
        step();
        arrived_counter = 0;
    } else if (abs(error) <= minError) {
        // turn off motor
        digitalWrite(enPin, HIGH);
        enabled = false;
        integral = 0;
        arrived_counter++;
        if (!reached_target && arrived_counter >= 5) {
            Serial.println(
                "t: " + String(millis() - move_start_time_ms) + " " + String(counter) + " " + String(pid_loop_counter));
            reached_target = true;
        }
    }
    pid_loop_counter++;
}

// lets define speed as a float between 0 and 1, 0 meaning largest pulseWidth and 1 meaning smallest pulseWidth
void stepper::step(bool do_delay_at_end) {
    if (current_ltc_value < MIN_LTC && direction == HIGH) {
        // Serial.println("Min LTC reached");
        return;
    }
    if (current_ltc_value > MAX_LTC && direction == LOW) {
        // Serial.println("Max LTC reached");
        return;
    }
    const int targetPulseWidth = map(target_speed * 1000, 0, 1000, maxPulseWidth, a_minPulseWidth);
    if (direction == current_direction) {
        if (pulseWidth < targetPulseWidth) {
            pulseWidth = min(pulseWidth + acceleration, targetPulseWidth);
        } else if (pulseWidth > targetPulseWidth) {
            pulseWidth = max(pulseWidth - acceleration, targetPulseWidth);
        }
    } else {
        current_direction = direction;
        pulseWidth = maxPulseWidth;
    }
    // only change direction if it is different from last time
    if (direction != last_direction) {
        digitalWrite(dirXPin, direction);
        last_direction = direction;
    }

    if (!enabled) {
        digitalWrite(enPin, LOW);
        enabled = true;
    }
    // Step the motor for each half-step
    digitalWrite(stepXPin, HIGH);
    delayMicroseconds(pulseWidth);
    digitalWrite(stepXPin, LOW);
    if (do_delay_at_end)
        delayMicroseconds(pulseWidth);
    counter++;
}


void stepper::set_target(int input_int) {
    target_ltc_value = input_int;
    integral = 0;
    reached_target = false;
    move_start_time_ms = millis();
    arrived_counter = 0;
    counter = 0;
    pid_loop_counter = 0;
}
