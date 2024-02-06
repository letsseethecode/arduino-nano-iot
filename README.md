# Arduino Nano 33 IoT

This project is a starting platform to build applications on the
Arduino Nano 33 IoT. It comes preconfigured with WiFi, Gyroscope, and
serial via USB.

## Building

### Install dependencies:

The pre-requisites have all been added to the `Makefile`

```sh
make install
```

### Build & flash:

* press the reset button on the Arduino Nano 33 IoT twice
* observe the LED start to pulse slowly

1. Build the firmware
    ```sh
    make build
    ```
1. Pack the firmware ready for flashing
    ```sh
    make pack
    ```
1. Flash the firmware onto the Arduino Nano 33 IoT
    ```sh
      make deploy
    ```

Note: Each step depends on the previous one, so `make deploy` will call `make pack`, which will call `make build` where needed.
