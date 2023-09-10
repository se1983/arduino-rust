#![no_std]
#![no_main]

use arduino_hal::hal::port::{PD3, PD5, PD6};
use arduino_hal::pac::TC0;
use arduino_hal::port::mode::{Analog, Input, Output, PwmOutput};
use arduino_hal::port::{mode, Pin, PinOps};
use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::{IntoPwmPin, PwmPinOps, Timer0Pwm, Timer2Pwm};
use arduino_hal::{DefaultClock, Peripherals, Pins, Usart};
use embedded_hal::PwmPin;
use panic_halt as _;

// https://docs.arduino.cc/tutorials/generic/secrets-of-arduino-pwm

static sequence: [[u8; 3]; 64] = [
    [17, 130, 75],
    [33, 13, 11],
    [136, 170, 199],
    [28, 250, 113],
    [176, 213, 48],
    [109, 246, 149],
    [19, 238, 223],
    [239, 235, 80],
    [27, 173, 158],
    [241, 120, 127],
    [157, 39, 101],
    [39, 86, 211],
    [176, 122, 209],
    [95, 17, 25],
    [244, 220, 56],
    [129, 61, 225],
    [234, 87, 236],
    [116, 168, 24],
    [114, 242, 97],
    [136, 81, 209],
    [78, 32, 79],
    [97, 3, 188],
    [188, 194, 134],
    [229, 242, 166],
    [135, 171, 50],
    [200, 247, 52],
    [168, 29, 214],
    [241, 237, 246],
    [85, 248, 146],
    [84, 118, 37],
    [252, 101, 224],
    [160, 169, 166],
    [197, 59, 14],
    [34, 215, 211],
    [115, 65, 29],
    [230, 240, 246],
    [186, 194, 102],
    [87, 195, 49],
    [18, 9, 223],
    [27, 156, 110],
    [88, 40, 69],
    [189, 54, 98],
    [45, 214, 56],
    [72, 163, 143],
    [65, 235, 187],
    [185, 84, 22],
    [24, 247, 235],
    [184, 63, 199],
    [242, 71, 21],
    [94, 250, 166],
    [9, 190, 83],
    [190, 212, 82],
    [174, 227, 98],
    [12, 198, 5],
    [60, 253, 14],
    [10, 191, 221],
    [17, 85, 73],
    [187, 193, 25],
    [9, 124, 42],
    [234, 203, 39],
    [94, 123, 246],
    [86, 132, 125],
    [236, 37, 219],
    [158, 60, 149],
];

trait Slider {
    fn slide(&mut self, duty: u8);
}

impl<TC, PIN: PwmPinOps<TC, Duty = u8>> Slider for Pin<mode::PwmOutput<TC>, PIN> {
    fn slide(&mut self, duty: u8) {
        let current_duty = self.get_duty();
        if current_duty < duty {
            for d in (current_duty..duty) {
                self.set_duty(d);
                arduino_hal::delay_ms(10);
            }
        } else {
            for d in (duty..current_duty).rev() {
                self.set_duty(duty);
                arduino_hal::delay_ms(10);
            }
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!").void_unwrap();

    let mut pwm_timer0 = arduino_hal::simple_pwm::Timer0Pwm::new(
        dp.TC0,
        arduino_hal::simple_pwm::Prescaler::Prescale64,
    );
    let mut pwm_timer2 = arduino_hal::simple_pwm::Timer2Pwm::new(
        dp.TC2,
        arduino_hal::simple_pwm::Prescaler::Prescale64,
    );

    let mut red = pins.d6.into_output().into_pwm(&mut pwm_timer0);
    let mut green = pins.d5.into_output().into_pwm(&pwm_timer0);
    let mut blue = pins.d3.into_output().into_pwm(&mut pwm_timer2);

    red.enable();
    green.enable();
    blue.enable();

    loop {
        for [r, g, b] in sequence {
            red.slide(r);
            green.slide(g);
            blue.slide(b);
        }
    }
}
