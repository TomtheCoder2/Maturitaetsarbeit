#ifndef STEPPER_H
#define STEPPER_H
#include <Arduino.h>
#include <pins_arduino.h>

constexpr int enPin = 8;
constexpr int stepXPin = 2;
constexpr int dirXPin = 5;
constexpr int microStepsRes = 8;
constexpr int ltc_analogPin = A15; // potentiometer wiper (middle terminal) connected to analog pin 15
constexpr int minPulseWidth = 1;
constexpr int minSlowPulseWidth = 5;
constexpr int maxPulseWidth = 250;
constexpr int minError = 3;
constexpr int acceleration = 40;
constexpr int integral_max = 25000;

constexpr int MIN_LTC = 120;
constexpr int MAX_LTC = 980;


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
    float p = 0.14;
    float i = 0.21;
    float d = 2.2;
    float last_error = 0;
    float integral = 0;

    int pulseWidth = maxPulseWidth;
    void setup();
    void loop();

    void set_target(int input_int);

private:
    void step(int direction, float speed, int a_minPulseWidth);
    bool reached_target = false;
    unsigned long move_start_time_ms = 0;
    // counter since last target setting
    unsigned long counter = 0;
    // counts how many times we are within the error margin
    int arrived_counter = 0;

    // optimization
    int last_direction = -1;
    bool enabled = false;
};




#endif //STEPPER_H
