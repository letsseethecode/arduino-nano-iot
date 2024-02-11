.PHONY = build pack deploy clean
.DEFAULT_GOAL = build
SRC_FILES = $(shell find . -name "*.rs")

install:
	brew update
	brew install arduino-cli
	arduino-cli core install arduino:samd
	cargo install cargo-binutils
	rustup component add llvm-tools-preview

target/thumbv6m-none-eabi/release/arduino-nano-33-iot: $(SRC_FILES)
	cargo build --release

target/arduino.bin: target/thumbv6m-none-eabi/release/arduino-nano-33-iot
	rust-objcopy -O binary target/thumbv6m-none-eabi/release/arduino-nano-33-iot target/arduino.bin

clean:
	rm -rf target/

build: target/thumbv6m-none-eabi/release/arduino-nano-33-iot

pack: target/arduino.bin

deploy: pack
	$(eval USB := $(shell ls /dev/cu.usbmodem*01 | head -n 1))
	arduino-cli upload -i target/arduino.bin -b arduino:samd:nano_33_iot -p ${USB}
