#ifndef DC_MOTOR_H
#define DC_MOTOR_H

#include <Arduino.h>

// consts
static constexpr int shoot_positions[] = {50, -100, 0};
constexpr int FULL_POWER = 255;

class DcMotor {
public:
    DcMotor(int enca, int encb, int pwm, int in1, int in2);

    static void resetPosition();
    void setup();
    int getPosition();
    bool loop();
    void turn(int dir);
    void stop();

    void shoot();

    int target = 0;

private:
    void setMotor(int dir, int pwmVal);

    int ENCA, ENCB, PWM, IN1, IN2;
    long prevTime;
    float prevError;
    float integral;
    int lastEncB;
    int timeSinceReachedPos;

    // shooting stuff
    int shoot_index = -1;
    // when the shot started, for debugging and measurement purposes
    int time_shoot_start_ms = 0;
    int last_pos = 0;
    int amount_of_prints = 0;
};

#endif // DC_MOTOR_H