//
// Created by Jan Wilhelm on 02.11.2025.
//

#ifndef STEPPER_H
#define STEPPER_H
#include <Arduino.h>
#include <pins_arduino.h>

constexpr int enPin = 8;
constexpr int stepXPin = 2;
constexpr int dirXPin = 5;
constexpr int microStepsRes = 4;
constexpr int ltc_analogPin = A15; // potentiometer wiper (middle terminal) connected to analog pin 15
constexpr int minPulseWidth = 10;
constexpr int maxPulseWidth = 250;
constexpr int minError = 3;


// pid logs:
// first stable but slow (after getting rid of the oscillations)
// float p = 0.01;
// float i = 0;
// float d = 0.5;

// latest stable:
// float p = 0.035;
// float i = 0.001;
// float d = 0.85;


class stepper {
public:
    int target_ltc_value = 500;
    int current_ltc_value = 500;
    int current_direction = HIGH;
    float p = 0.004;
    float i = 0.00015;
    float d = 0.7;
    float last_error = 0;
    float integral = 0;

    int pulseWidth = maxPulseWidth;
    void setup();
    void loop();
private:
    void step(int direction, float speed);
};




#endif //STEPPER_H
