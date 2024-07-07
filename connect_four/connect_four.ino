#include <Servo.h>

Servo myservo;

void step(int steps) {
  if (steps < 0) {
    digitalWrite(5, HIGH);
    steps = -steps;
  } else {
    digitalWrite(5, LOW);
  }

  for (int i = 0; i < steps; i++) {
    digitalWrite(2, HIGH);
    delayMicroseconds(45);
    digitalWrite(2, LOW);
    delayMicroseconds(45);
  }
}

int toNum(bool a, bool b, bool c) {
  // 000 is reserved
  if (!a &&  !b && c) { // 001
    return 0;
  } else if (!a &&  b && !c) { // 010
    return 1;
  } else if (!a &&  b && c) { // 011
    return 2;
  } else if (a &&  !b && !c) { // 100
    return 3;
  } else if (a &&  !b && c) { // 101
    return 4;
  } else if (a &&  b && !c) { // 110
    return 5;
  } else if (a &&  b && c) { // 111
    return 6;
  }

  return -1;
}

void activateServo() {
  myservo.write(0);
  delay(500);
  myservo.write(130);
}

bool analogToDigitalRead(int pin) {
  int value = analogRead(pin);
  return value > 512;
}

int rowPositions[] = {8400, 11300, 14000, 16900, 19600, 22400, 25500};

void setup() {
  Serial.begin(9600);
  pinMode(8, OUTPUT); // enabled low
  pinMode(5, OUTPUT); // direction
  pinMode(2, OUTPUT); // step
  myservo.attach(3);
  myservo.write(130);
  // pinMode(12, INPUT_PULLUP);
  // randomSeed(analogRead(A5));
  Serial.println("Started");


  digitalWrite(8, HIGH);
}

void loop() {
  while (toNum(analogToDigitalRead(A0), analogToDigitalRead(A1), analogToDigitalRead(A2)) == -1);
  int num = toNum(analogToDigitalRead(A0), analogToDigitalRead(A1), analogToDigitalRead(A2));

  digitalWrite(8, LOW);

  step(rowPositions[num]);
  delay(500);
  activateServo();
  delay(1500);

  step(-rowPositions[num]);

  digitalWrite(8, HIGH);
}

// void loop() {
//   // while (digitalRead(12));

//   // int inputChar = random(48, 55);

//   while (Serial.available() <= 0);

//   int inputChar = Serial.read();

//   if (inputChar == 'p') {
//     Serial.print("A0: ");
//     Serial.print(analogToDigital(analogRead(A0)));
//     Serial.print(" A1: ");
//     Serial.print(analogToDigital(analogRead(A1)));
//     Serial.print(" A2: ");
//     Serial.println(analogToDigital(analogRead(A2)));
//   } else {
//     digitalWrite(8, LOW);
//     Serial.write(inputChar);

//     step(rowPositions[inputChar - 48]);
//     delay(500);
//     myservo.write(0);
//     delay(500);
//     myservo.write(130);
//     delay(1500);

//     step(-rowPositions[inputChar - 48]);

//     digitalWrite(8, HIGH);
//   }
// }
