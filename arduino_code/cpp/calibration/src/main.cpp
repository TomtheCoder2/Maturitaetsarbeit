#include <Arduino.h>


// Ramp test: find max speed before skipping (1/8 microstep)
constexpr int STEP_PIN = 2, DIR_PIN = 5, EN_PIN = 8, LTC_PIN = A15;
constexpr int PULSE_US = 1, DIR_SETUP_US = 2, SETTLE_MS = 200;
constexpr int TEST_STEPS = 200; // Steps per rate test
constexpr float STEPS_PER_LTC = 1.8f;

void takeSteps(int dir, long steps) {
    digitalWrite(EN_PIN, LOW);
    delay(1);
    digitalWrite(DIR_PIN, dir);
    delayMicroseconds(DIR_SETUP_US);
    for (long i = 0; i < abs(steps); i++) {
        digitalWrite(STEP_PIN, HIGH);
        delayMicroseconds(1000);
        digitalWrite(STEP_PIN, LOW);
        delayMicroseconds(1000);
    }
    digitalWrite(EN_PIN, HIGH);
}

void setup() {
    pinMode(STEP_PIN, OUTPUT);
    pinMode(DIR_PIN, OUTPUT);
    pinMode(EN_PIN, OUTPUT);
    digitalWrite(EN_PIN, HIGH);
    Serial.begin(57600);
    // Wait for Serial to connect (especially for native USB boards)
    while (!Serial) {
        delay(10);
    }
    Serial.println("Ramp test starting. Watch Serial for max SPS.");
    // delay(2000);
}

void loop() {
    int start_ltc = analogRead(LTC_PIN);
    Serial.print("Start LTC: ");
    Serial.println(start_ltc);

    for (float sps = 1000.0f; sps <= 2000000.0f; sps += 100.0f) {
        unsigned long step_delay_us = 500000UL / sps; // Full cycle delay
        int ltc_before = analogRead(LTC_PIN);
        unsigned long t0 = millis();
        digitalWrite(DIR_PIN, LOW); // Move "forward"
        digitalWrite(EN_PIN, LOW);
        for (long i = 0; i < TEST_STEPS; i++) {
            digitalWrite(STEP_PIN, HIGH);
            delayMicroseconds(step_delay_us);
            digitalWrite(STEP_PIN, LOW);
            delayMicroseconds(step_delay_us);
        }
        digitalWrite(EN_PIN, HIGH);
        delay(SETTLE_MS);
        int ltc_after = analogRead(LTC_PIN);
        int delta_ltc = ltc_after - ltc_before;
        float observed_sps = (float) TEST_STEPS / ((millis() - t0) / 1000.0f);
        float ratio = (float) TEST_STEPS / abs(delta_ltc); // Should ~1.8
        Serial.print(sps, 0);
        Serial.print(" SPS cmd: ratio=");
        Serial.print(ratio, 2);
        Serial.print(" | deltaLTC=");
        Serial.print(delta_ltc);
        Serial.print(" | obs SPS=");
        Serial.println(observed_sps, 0);
        Serial.print(" | delay us=");
        Serial.println(step_delay_us);

        // Return
        takeSteps(HIGH, TEST_STEPS);
        delay(SETTLE_MS);

        if (abs(ratio - STEPS_PER_LTC) > 0.5f) {
            // Skipping detected
            Serial.print("Skipping starts at ca. ");
            Serial.print(sps - 200);
            Serial.println(" SPS");
            break;
        }
    }
    takeSteps(HIGH, TEST_STEPS); // Final return


    // Halt execution
    while (true) {
        delay(1000);
    }
}
