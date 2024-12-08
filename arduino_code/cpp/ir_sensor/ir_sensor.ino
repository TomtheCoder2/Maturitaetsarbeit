#include "GP2Y0E03.h"

GP2Y0E03 sensor = GP2Y0E03();

void setup() {
  Serial.begin(57600);
  sensor.init(A8);
}

// 0 - 23
// 320 - 7

// motor pos, vout
// 0

void loop() {
  Serial.print("digital:");
  Serial.println(sensor.distDigital());

  Serial.print("analog:");
  Serial.println(sensor.distAnalog());

  Serial.print("VOUT:");
  Serial.println(sensor.vout()/10);

  Serial.print("max:");
  Serial.println(50);

  Serial.print("min:");
  Serial.println(-1);

  Serial.println();
  // delay(100);
}