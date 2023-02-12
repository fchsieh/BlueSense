# BlueSense - Presence Detection

BlueSense is a simple Rust application that uses the [btleplug](https://github.com/deviceplug/btleplug) to connect to a Bluetooth Low Energy (BLE) device and detect the number of nearby devices.
The application is designed to be run on a Raspberry Pi Zero and should work on any system that supports Bluetooth and Rust.

## Start API Server

```Rust
cargo run
```

## Start Frontend

```bash
cd frontend && npm install
npm start
```
