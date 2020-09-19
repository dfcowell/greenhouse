const int wet = 338;
const int dry = 591;
const int relayPin = 7;

const int humidityThreshold = 30;

void setup() {
  Serial.begin(9600);
  pinMode(relayPin, OUTPUT);
}

void loop() {
  int pumpStatus = LOW;
  char outputBuf[36];
  int sensorValue0 = analogRead(A0);

  int humidityPercentage0 = map(sensorValue0, wet, dry, 100, 0);

  if (humidityPercentage0 < humidityThreshold) {
    digitalWrite(relayPin, HIGH);
    // Run pump for 1s then re-evaluate
    delay(1000);
  } else {
    digitalWrite(relayPin, LOW);
    delay(1000);
  }
  
  sprintf(outputBuf, "%d:%s", humidityPercentage0, pumpStatus == HIGH ? "ON" : "OFF");

  Serial.println(outputBuf);
}