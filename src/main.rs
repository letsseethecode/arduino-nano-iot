#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_variables)]

mod morse;
mod orientation;
mod time;
mod usb;

use arduino_nano33iot as bsp;
use bsp::entry;
use bsp::hal;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use morse::convert_message_into_morse_buffer;
use panic_halt as _;
use usb::usb_log;

const DURATION: u64 = 100;
const DIT_DURATION: u64 = DURATION;
const DAH_DURATION: u64 = DURATION * 3;
const SPC_DURATION: u64 = DURATION;

#[entry]
fn main() -> ! {
    // We need this to be able to access all the capabilities of the board
    let mut peripherals = Peripherals::take().unwrap();
    // We need the pins to access the LED and setup the USB
    let pins = bsp::Pins::new(peripherals.PORT);
    // We need a reference to the LED that we're going to blink
    let mut led: bsp::Led = pins.led_sck.into();

    // Timer is needed to be able to blink for the correct duration
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let core = CorePeripherals::take().unwrap();
    let delay = Delay::new(core.SYST, &mut clocks);
    let gclk0 = clocks.gclk0();
    let mut timer = time::Timer::new(peripherals.TC5, &mut clocks, &gclk0, &mut peripherals.PM);

    unsafe {
        usb::setup_usb(
            &mut clocks,
            peripherals.USB,
            &mut peripherals.PM,
            pins.usb_dm,
            pins.usb_dp,
        );
    }

    let mut buffer: [u8; 256] = [0; 256];
    let mut buffer_length: usize = 0;

    let message = "SOS  ";

    convert_message_into_morse_buffer(&mut buffer, &mut buffer_length, message);

    let mut led_index: usize = 0;
    let mut led_on_time: u64 = 0;
    let mut led_off_time: u64 = 0;

    loop {
        timer.tick();
        let now = timer.millis();

        set_led(
            &mut led,
            now,
            &mut led_on_time,
            &mut led_off_time,
            &buffer,
            buffer_length,
            &mut led_index,
        );

        unsafe {
            // NB Must be called at least once every 10ms to stay USB-compliant.
            poll_usb(&timer, message, &buffer, buffer_length);
        }
    }
}

fn set_led(
    led: &mut bsp::Led,
    time_now: u64,
    led_on_time: &mut u64,
    led_off_time: &mut u64,
    buffer: &[u8; 256],
    buffer_length: usize,
    index: &mut usize,
) {
    if time_now >= *led_on_time {
        if *index > buffer_length {
            *index = 0;
        }
        let c = morse::get_signal_from_buffer(buffer, *index);
        *index += 1;
        match c {
            morse::DIT => {
                usb_log(".");
                *led_off_time = time_now + DIT_DURATION;
                led.set_high().unwrap();
            }
            morse::DAH => {
                usb_log("-");
                *led_off_time = time_now + DAH_DURATION * 3;
                led.set_high().unwrap();
            }
            _ => {
                usb_log(" ");
                *led_off_time = time_now + SPC_DURATION;
            }
        }
        *led_on_time = *led_off_time + SPC_DURATION;
    } else if time_now >= *led_off_time {
        led.set_low().unwrap();
    }
}

unsafe fn poll_usb(timer: &time::Timer, message: &str, buffer: &[u8; 256], length: usize) {
    if let Some(usb_dev) = usb::USB_BUS.as_mut() {
        if let Some(serial) = usb::USB_SERIAL.as_mut() {
            usb_dev.poll(&mut [serial]);

            let mut buf = [0u8; 127];
            if let Ok(count) = serial.read(&mut buf) {
                handle_serial(
                    core::str::from_utf8(&buf[..count - 1]).unwrap(),
                    timer,
                    message,
                    buffer,
                    length,
                );
            }
        }
    };
}

fn handle_serial(s: &str, timer: &time::Timer, message: &str, buffer: &[u8; 256], length: usize) {
    let mut buf = [0u8; 32];
    match s {
        "b" | "buffer" => {
            usb_log("buffer: [");
            for b in *buffer {
                lexical_core::write(b, &mut buf);
                usb_log(core::str::from_utf8(&buf).unwrap());
                usb_log(", ");
            }
            usb_log("]\r\n");
        }
        "m" | "message" => {
            usb_log("message: '");
            usb_log(message);
            usb_log("'\r\n");
        }
        "p" | "ping" => usb_log("pong\r\n"),
        "t" | "tick" => {
            usb_log("rtc: ");
            lexical_core::write(timer.millis(), &mut buf);
            usb_log(core::str::from_utf8(&buf).unwrap());
            usb_log("\r\n");
        }
        _ => {
            usb_log("unknown command: '");
            usb_log(s);
            usb_log("'\r\n");
        }
    };
}
