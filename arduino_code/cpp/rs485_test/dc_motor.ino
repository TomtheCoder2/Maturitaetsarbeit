#include <util/atomic.h>  // For the ATOMIC_BLOCK macro

#define ENCA 19  // YELLOW
#define ENCB 26  // WHITE
#define PWM 44
#define IN2 50
#define IN1 51

volatile int posi = 0;  // specify posi as volatile: https://www.arduino.cc/reference/en/language/variables/variable-scope-qualifiers/volatile/
long prevT = 0;
float eprev = 0;
float eintegral = 0;
int last_encb = 0;
int t_since_reached_pos = 0;

void reset_dc_pos() {
  // ATOMIC_BLOCK(ATOMIC_RESTORESTATE) {
    posi = 0;
  // }
}

void dc_setup() {
  Serial.begin(57600);
  pinMode(ENCA, INPUT);
  pinMode(ENCB, INPUT);
  attachInterrupt(digitalPinToInterrupt(ENCA), readEncoder, RISING);
  // delay(1000);

  // pinMode(PWM,OUTPUT);
  pinMode(IN1, OUTPUT);
  pinMode(IN2, OUTPUT);

  Serial.println("target pos");
}

int get_pos() {
  int pos = 0;
  ATOMIC_BLOCK(ATOMIC_RESTORESTATE) {
    pos = posi;
  }
  return pos;
}

// return true when target is reached
bool dc_loop(int target) {
  // set target position
  // int target = -50;
  // int target = 45*sin(prevT/1e6);

  // PID constants
  float kp = 35.0;
  float kd = 1.5;
  float ki = 0;

  // time difference
  long currT = micros();
  float deltaT = ((float)(currT - prevT)) / (1.0e6);
  prevT = currT;

  // Read the position in an atomic block to avoid a potential
  // misread if the interrupt coincides with this code running
  // see: https://www.arduino.cc/reference/en/language/variables/variable-scope-qualifiers/volatile/
  int pos = 0;
  ATOMIC_BLOCK(ATOMIC_RESTORESTATE) {
    pos = posi;
  }

  // error
  int e = pos - target;

  // derivative
  float dedt = (e - eprev) / (deltaT);

  // integral
  eintegral = eintegral + e * deltaT;

  // control signal
  float u = kp * e + kd * dedt + ki * eintegral;

  // motor power
  float pwr = fabs(u);
  if (pwr > 255) {
    pwr = 255;
  }

  // motor direction
  int dir = -1;
  if (u < 0) {
    dir *= -1;
  }


  // store previous error
  eprev = e;

  if (abs(e) < 5) {
    t_since_reached_pos++;
  } else {
    t_since_reached_pos = 0;
  }

  if (t_since_reached_pos > 30) {
    pwr = 0;
    // Serial.println("reached target");
  }

  // signal the motor
  setMotor(dir, pwr, PWM, IN1, IN2);

  // Serial.print(target);Serial.print(" ");Serial.print(pos);Serial.print(" ");Serial.print(pwr);Serial.print(" ");Serial.print(dir); Serial.println();
  return t_since_reached_pos > 30;
}

void turn(int dir) {
  // Serial.println("turn dir");
  setMotor(dir, 255, PWM, IN1, IN2);
}

void stop() {
  setMotor(1, 0, PWM, IN1, IN2);
}

void setMotor(int dir, int pwmVal, int pwm, int in1, int in2) {
  analogWrite(pwm, pwmVal);
  if (dir == 1) {
    digitalWrite(in1, HIGH);
    digitalWrite(in2, LOW);
  } else if (dir == -1) {
    digitalWrite(in1, LOW);
    digitalWrite(in2, HIGH);
  } else {
    digitalWrite(in1, LOW);
    digitalWrite(in2, LOW);
  }
}

void readEncoder() {
  int b = digitalRead(ENCB);
  if (b > 0) {
    posi++;
  } else {
    posi--;
  }
}