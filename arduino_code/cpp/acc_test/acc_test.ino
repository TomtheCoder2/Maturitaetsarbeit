#include "./AccelStepper.h"
const byte dirPin = 5;
const byte stepPin = 2;
const int enPin = 8;
const byte motorInterfaceType = AccelStepper::DRIVER;  // A4988
AccelStepper stepper = AccelStepper(motorInterfaceType, stepPin, dirPin);

const byte leftLimitPin = A0;

long targetPosition = 0;
const long maxPosition = 400;

// random movement from the current position within [0, maxPosition] range
void defineTarget() {
  long deltaPos = random(300, 1001);
  if (random(0, 2) == 0) targetPosition = min(maxPosition, targetPosition + deltaPos);  // forward
  else targetPosition = max(0, targetPosition - deltaPos);                              // backward
  Serial.print(F("Target: "));
  Serial.println(targetPosition);
  stepper.moveTo(targetPosition);
}

void setup() {
  Serial.begin(57600);
  pinMode(enPin, OUTPUT);
  digitalWrite(enPin, LOW);
  // pinMode(leftLimitPin, INPUT_PULLUP);
  // randomSeed(analogRead(A1));
  stepper.setMinPulseWidth(100);
  stepper.setCurrentPosition(0);

  stepper.setMaxSpeed(5000);
  stepper.setAcceleration(100);
  // home();   // homing
  defineTarget();
}

void loop() {
  while (stepper.distanceToGo() != 0) stepper.run();
  Serial.println(F("pause"));
  delay(500);
  defineTarget();
}