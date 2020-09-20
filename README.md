# Greenhouse

Greenhouse is an open-source tool for monitoring and automating the watering of plants. It consists of an Arduino-based data collector/ hardware controller and a metrics collector implemented in Rust to plot environmental data over time.

## Hardware Dependencies

Greenhouse has been confirmed to run out-of-the-box using the following hardawre:

- Arduino Uno (or compatible microcontroller)
- Raspberry Pi 4
- Capacitive Soil Moisture Sensor

There is some setup and configuration required to callibrate the Arduino for your specific sensor. See the [Arduino Readme](arduino/README.md) for more information.