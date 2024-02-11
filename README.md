# Arduino Nano 33 IoT

This project is a starting platform to build applications on the
Arduino Nano 33 IoT. It comes preconfigured with WiFi, Gyroscope, and
serial via USB.

# Acknowledgements

This project was forked from https://github.com/sulami/arduino-nano-33 and then extended to meet our requirements.

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

## Connecting to USB

Use `screen` to connect to the USB port.
```sh
screen /dev/cu.usbmodemLSTC1
```

Send messages by:
1. `Ctrl-A`
1. `: <enter>`
1. `exec !! echo "<message>"`

Quite by:
1. `Ctrl-A`
1. `Ctrl-K`
1. `y`
