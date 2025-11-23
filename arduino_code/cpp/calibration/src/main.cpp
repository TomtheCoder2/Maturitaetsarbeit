#include <Arduino.h>

// CONFIGURATION: PLEASE ADJUST THESE VALUES
// ----------------------------------------------------------------
// Pin definitions for your stepper motor driver
constexpr int STEP_PIN = 2;
constexpr int DIR_PIN = 5;
constexpr int EN_PIN = 8;

// Analog pin connected to your Linear Transducer (LTC)
constexpr int LTC_PIN = A15;

// The number of discrete measurements to take in each direction.
constexpr int NUM_MEASUREMENTS = 2;

// The number of steps the motor will take for each measurement.
// Ensure that (NUM_MEASUREMENTS * STEPS_PER_MEASUREMENT) does not exceed
// the physical travel limits of your system.
constexpr int STEPS_PER_MEASUREMENT = 300;

// The duration of the step pulse in microseconds
constexpr int PULSE_WIDTHS_US[] = {1000, 500, 200, 100, 50, 20, 1};
constexpr int DEFAULT_PULSE_WIDTH_US = PULSE_WIDTHS_US[0];
const long ACC = 10;

// A brief delay in milliseconds after each move to let the system settle.
constexpr int SETTLE_DELAY_MS = 500;

// Serial port speed for displaying results.
constexpr long SERIAL_BAUD_RATE = 57600;
// ----------------------------------------------------------------


/**
 * @brief Moves the stepper motor a specified number of steps.
 * @param steps The number of steps to move. A positive value moves in one
 *              direction (e.g., FORWARD), and a negative value moves in reverse.
 */
void takeSteps(long steps, long delayUS) {
    if (steps == 0) return;

    digitalWrite(EN_PIN, LOW); // Enable motor
    delay(2); // Allow driver to power up

    // Set the direction based on the sign of the 'steps' parameter
    digitalWrite(DIR_PIN, (steps > 0) ? HIGH : LOW);

    // accelerate
    long current_delay = DEFAULT_PULSE_WIDTH_US;

    for (long i = 0; i < abs(steps); ++i) {
        if ((DEFAULT_PULSE_WIDTH_US - current_delay) / ACC > abs(steps) - i) {
            current_delay = min(current_delay + ACC, DEFAULT_PULSE_WIDTH_US);
        } else if (delayUS < current_delay) {
            current_delay = max(current_delay - ACC, delayUS);
        }
        digitalWrite(STEP_PIN, HIGH);
        delayMicroseconds(current_delay);
        digitalWrite(STEP_PIN, LOW);
        delayMicroseconds(current_delay);
    }

    digitalWrite(EN_PIN, HIGH); // Disable motor to save power and reduce heat
}

/**
 * @brief Runs a measurement sequence in a given direction.
 * @param steps_per_move The number of steps for each individual move.
 *                       Sign determines direction.
 * @param out_scale A reference to a float where the calculated scale
 *                  for this direction will be stored.
 */
void runMeasurementSequence(long steps_per_move, float& out_scale, long delayUS = DEFAULT_PULSE_WIDTH_US) {
    const long start_ltc_value = analogRead(LTC_PIN);
    Serial.println("Move #: \tTotal Steps: \tLTC Reading:");
    Serial.print("0\t0\t\t");
    Serial.println(start_ltc_value);

    long total_ltc_change = 0;

    for (int i = 1; i <= NUM_MEASUREMENTS; ++i) {
        takeSteps(steps_per_move, delayUS);
        delay(SETTLE_DELAY_MS);
        const long current_ltc_value = analogRead(LTC_PIN);
        // go back
        takeSteps(-steps_per_move, delayUS);
        Serial.print(i);
        Serial.print("\t");
        Serial.print(i * steps_per_move);
        Serial.print("\t\t");
        Serial.println(current_ltc_value);
        total_ltc_change += abs(current_ltc_value - start_ltc_value);
    }

    const long total_steps_taken = NUM_MEASUREMENTS * steps_per_move;

    if (total_ltc_change > 0) {
        out_scale += (float)abs(total_steps_taken) / (float)total_ltc_change;
        Serial.print("\nDirectional Scale: ");
        Serial.print(out_scale);
        Serial.println(" steps per LTC unit.");
    } else {
        out_scale += 0.0f;
        Serial.println("\nWarning: No change in LTC value detected.");
    }
}


void setup() {
    pinMode(STEP_PIN, OUTPUT);
    pinMode(DIR_PIN, OUTPUT);
    pinMode(EN_PIN, OUTPUT);
    digitalWrite(EN_PIN, HIGH); // Start with motor disabled

    Serial.begin(SERIAL_BAUD_RATE);
    // Wait for Serial to connect (especially for native USB boards)
    while (!Serial) {
        delay(10);
    }
    Serial.println("\n--- Stepper to LTC Calibration Script ---");
    Serial.println("Ensure the mechanism has a clear path for movement.");
    // Serial.println("Starting in 5 seconds...");
    // delay(5000);
}

void loop() {
    float scale_forward = 0.0f;
    float scale_reverse = 0.0f;

    for (const int delay_us : PULSE_WIDTHS_US) {
        // --- First Direction ---
        Serial.println("\n--- Measuring in FORWARD direction with pulse width " + String(delay_us) + "us ---");
        runMeasurementSequence(STEPS_PER_MEASUREMENT, scale_forward, delay_us);
    }
    // because we are always adding to the variable, so we have to divide by the length of the list of all pulsewidths to get the average
    scale_forward = scale_forward / ((float)sizeof(PULSE_WIDTHS_US) / (float)sizeof(PULSE_WIDTHS_US[0]));

    for (const int delay_us : PULSE_WIDTHS_US) {
        // --- Reverse Direction ---
        Serial.println("\n--- Measuring in REVERSE direction with pulse width " + String(delay_us) + "us ---");
        runMeasurementSequence(-STEPS_PER_MEASUREMENT, scale_reverse, delay_us);
    }
    scale_reverse = scale_reverse / ((float)sizeof(PULSE_WIDTHS_US) / (float)sizeof(PULSE_WIDTHS_US[0]));

    // --- Final Calculation ---
    Serial.println("\n\n--- CALIBRATION COMPLETE ---");
    if (scale_forward > 0.0f && scale_reverse > 0.0f) {
        const float average_scale = (scale_forward + scale_reverse) / 2.0f;
        Serial.print("Average scaling factor: ");
        Serial.println(average_scale, 4); // Print with 4 decimal places
        Serial.println("This is your 'steps_per_ltc_unit' value.");
        Serial.println("You can now use this constant in your motion planning code.");
    } else {
        Serial.println("Could not calculate a valid average scale.");
        Serial.println("Please check your wiring and ensure the motor is moving.");
    }

    // Halt execution
    while (true) {
        delay(1000);
    }
}