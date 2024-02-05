.PHONY = build pack deploy
.DEFAULT_GOAL = build
SRC_FILES = $(shell find . -name "*.rs")

target/thumbv6m-none-eabi/release/arduino-nano-33-iot: $(SRC_FILES)
	cargo build --release

target/arduino.bin: target/thumbv6m-none-eabi/release/arduino-nano-33-iot
	rust-objcopy -O binary target/thumbv6m-none-eabi/release/arduino-nano-33-iot target/arduino.bin

build: target/thumbv6m-none-eabi/release/arduino-nano-33-iot

pack: target/arduino.bin

deploy: pack
	arduino-cli upload -i target/arduino.bin -b arduino:samd:nano_33_iot -p /dev/cu.usbmodem1101
