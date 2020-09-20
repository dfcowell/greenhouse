const int wet = 338;
const int dry = 591;
const int relayPin = 7;

const int humidityThreshold = 40;

void setup()
{
  Serial.begin(9600);
  pinMode(relayPin, OUTPUT);
}

void loop()
{
  int pumpStatus = LOW;
  char outputBuf[36];
  int sensorValue0 = analogRead(A0);

  int humidityPercentage0 = map(sensorValue0, wet, dry, 100, 0);

  if (humidityPercentage0 < humidityThreshold)
  {
    pumpStatus = HIGH;
  }
  else
  {
    pumpStatus = LOW;
  }

  sprintf(outputBuf, "A0:%d:%d", humidityPercentage0, pumpStatus);
  Serial.println(outputBuf);

  digitalWrite(relayPin, pumpStatus);
  delay(1000);
}