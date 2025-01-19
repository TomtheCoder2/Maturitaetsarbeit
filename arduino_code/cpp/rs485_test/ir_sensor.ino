#include "GP2Y0E03.h"

GP2Y0E03 sensor = GP2Y0E03();

void ir_setup() {
  Serial.begin(57600);
  sensor.init(A8);
}

// 0 - 23
// 320 - 7

// motor pos, vout
// 0

void ir_loop() {
  Serial.print("digital:");
  Serial.println(sensor.distDigital());

  Serial.print("analog:");
  Serial.println(sensor.distAnalog());

  Serial.print("VOUT:");
  Serial.println((float)sensor.vout() / 10.0);

  Serial.print("max:");
  Serial.println(50);

  Serial.print("min:");
  Serial.println(-1);

  float v = vout();
  Serial.print("pos:");
  // Serial.println((float)voutToPos(v)/(float)10);
  // Serial.println(v);
  // delay(100);

  Serial.println();
}

float vout() {
  long v = 0;
  const int n = 1000;
  for (int i = 0; i < n; i++) {
    int current_v = sensor.vout();
    // Serial.print(current_v);Serial.print(" ");
    v += current_v;
  }
  // Serial.println();
  // Serial.println(v);
  return (float)v / (float)n;
}


int vout_int() {
  long v = 0;
  const int n = 1000;
  for (int i = 0; i < n; i++) {
    int current_v = sensor.vout();
    // Serial.print(current_v);Serial.print(" ");
    v += current_v;
  }
  // Serial.println();
  // Serial.println(v);
  return (int)((float)v / (float)n);
}

float vout_100() {
  long v = 0;
  const int n = 100;
  for (int i = 0; i < n; i++) {
    int current_v = sensor.vout();
    // Serial.print(current_v);Serial.print(" ");
    v += current_v;
  }
  // Serial.println();
  // Serial.println(v);
  return (float)v / (float)n;
}

float vout_n(int n) {
  long v = 0;
  // const int n = 100;
  for (int i = 0; i < n; i++) {
    int current_v = sensor.vout();
    // Serial.print(current_v);Serial.print(" ");
    v += current_v;
  }
  // Serial.println();
  // Serial.println(v);
  return (float)v / (float)n;
}

float print_vout() {
  long v = 0;
  const int n = 100;
  for (int i = 0; i < n; i++) {
    int current_v = sensor.vout();
    Serial.print(current_v);
    Serial.print(", ");
    v += current_v;
  }
  Serial.println();
  Serial.println(v);
  return (float)v / (float)n;
}

float voutFast() {
  return (float)sensor.vout();
}