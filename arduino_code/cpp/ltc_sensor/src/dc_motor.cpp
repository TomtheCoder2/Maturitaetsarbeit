#include <Arduino.h>
#include <util/atomic.h>
#include "dc_motor.h"
#define D_ENCB 26  // WHITE
#define D_ENCA 19  // YELLOW

// must be here, because idk how attach a member function as interrupt
volatile int position;
int lastEncA = 0;

DcMotor::DcMotor(const int enca, const int encb, const int pwm, const int in1, const int in2)
    : ENCA(enca), ENCB(encb), PWM(pwm), IN1(in1), IN2(in2),
      prevTime(0), prevError(0), integral(0),
      lastEncB(0), timeSinceReachedPos(0) {
}

void DcMotor::resetPosition() {
    ATOMIC_BLOCK(ATOMIC_RESTORESTATE) {
        position = 0;
    }
}

void readEncoder() {
    const int a = digitalRead(D_ENCA);
    const int b = digitalRead(D_ENCB);
    if (a == lastEncA) return; // Ignore if no change in ENCA
    lastEncA = a;

    if (a == b) {
        position++;
    } else {
        position--;
    }
}

void DcMotor::setup() {
    pinMode(ENCA, INPUT);
    pinMode(ENCB, INPUT);
    attachInterrupt(digitalPinToInterrupt(ENCA), readEncoder, CHANGE);

    pinMode(PWM, OUTPUT);
    pinMode(IN1, OUTPUT);
    pinMode(IN2, OUTPUT);
}

int DcMotor::getPosition() {
    int pos = 0;
    ATOMIC_BLOCK(ATOMIC_RESTORESTATE) {
        pos = position;
    }
    return pos;
}

bool DcMotor::loop() {
    constexpr float kp = 35.0;
    constexpr float ki = 0;
    constexpr float kd = 1.5;

    const long currTime = micros();
    const float deltaTime = ((float) (currTime - prevTime)) / 1.0e6;
    prevTime = currTime;

    const int pos = getPosition();
    const int error = pos - target;

    const float derivative = (error - prevError) / deltaTime;
    integral += error * deltaTime;

    const float controlSignal = kp * error + kd * derivative + ki * integral;

    float power = fabs(controlSignal);
    if (power > FULL_POWER) power = FULL_POWER;

    const int direction = (controlSignal > 0) ? -1 : 1;

    prevError = error;

    if (abs(error) < 2) {
        timeSinceReachedPos++;
    } else {
        timeSinceReachedPos = 0;
    }

    if (timeSinceReachedPos > 5) {
        power = 0;
        if (shoot_index != -1) {
            target = shoot_positions[shoot_index];
            shoot_index++;
            if (shoot_index >= sizeof(shoot_positions) / sizeof(shoot_positions[0])) {
                shoot_index = -1;
            }
        }
    }

    if (shoot_index == 1) {
        power = FULL_POWER;
    }
    // debug measurement
    if (((last_pos > 0 && pos < 0) || pos == 0) && amount_of_prints > 0) {
        Serial.print("Dc Motor took ");
        Serial.print(millis() - time_shoot_start_ms);
        Serial.println("ms from start to back to pos 0");
        amount_of_prints--;
    }
    last_pos = pos;

    setMotor(direction, power);
    return timeSinceReachedPos > 30;
}

void DcMotor::setMotor(const int dir, const int pwmVal) {
    analogWrite(PWM, pwmVal);
    if (dir == 1) {
        digitalWrite(IN1, HIGH);
        digitalWrite(IN2, LOW);
    } else if (dir == -1) {
        digitalWrite(IN1, LOW);
        digitalWrite(IN2, HIGH);
    } else {
        digitalWrite(IN1, LOW);
        digitalWrite(IN2, LOW);
    }
}

void DcMotor::turn(const int dir) {
    setMotor(dir, 255);
}

void DcMotor::stop() {
    setMotor(0, 0);
}

void DcMotor::shoot() {
    shoot_index = 0;
    target = shoot_positions[shoot_index];
    time_shoot_start_ms = millis();
#ifdef DEBUG
    amount_of_prints = 5;
#endif
}
