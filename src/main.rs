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
use morse::append_buffer;
use panic_halt as _;
use usb::usb_log;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led: bsp::Led = pins.led_sck.into();

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
    let mut count: usize = 0;

    let message = "LETS SEE THE CODE  ";

    append_buffer(&mut buffer, &mut count, message);

    const DURATION: u64 = 100;
    const DIT_DURATION: u64 = DURATION;
    const DAH_DURATION: u64 = DURATION * 3;
    const SPC_DURATION: u64 = DURATION;

    let mut index: usize = 0;
    let mut next_on: u64 = 0;
    let mut next_off: u64 = 0;

    loop {
        timer.tick();
        let now = timer.millis();

        if next_on <= now {
            if index > count {
                index = 0;
            }
            let c = morse::get_signal(&buffer, index);
            index += 1;
            match c {
                morse::DIT => {
                    usb_log(".");
                    next_off = now + DIT_DURATION;
                    led.set_high().unwrap();
                }
                morse::DAH => {
                    usb_log("-");
                    next_off = now + DAH_DURATION * 5;
                    led.set_high().unwrap();
                }
                _ => {
                    usb_log(" ");
                    next_off = now + SPC_DURATION;
                }
            }
            next_on = next_off + SPC_DURATION;
        } else if next_off <= now {
            led.set_low().unwrap();
        }
        unsafe {
            // NB Must be called at least once every 10ms to stay USB-compliant.
            poll_usb(&timer, &message, &buffer, count);
        }
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
            usb_log("buffer: '");
            for b in *buffer {
                lexical_core::write(b, &mut buf);
                usb_log(core::str::from_utf8(&buf).unwrap());
                usb_log(", ");
            }
            usb_log("'\r\n");
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
