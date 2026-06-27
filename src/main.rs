#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;

const DOT_MS: u32 = 200;
const DASH_MS: u32 = DOT_MS * 3;

fn flash_letter(letter: char, led: &mut Pin<Output>) {
    let morse_code = match letter.to_ascii_uppercase() {
        'A' => ".-",
        'B' => "-...",
        'C' => "-.-.",
        'D' => "-..",
        'E' => ".",
        'F' => "..-.",
        'G' => "--.",
        'H' => "....",
        'I' => "..",
        'J' => ".---",
        'K' => "-.-",
        'L' => ".-..",
        'M' => "--",
        'N' => "-.",
        'O' => "---",
        'P' => ".--.",
        'Q' => "--.-",
        'R' => ".-.",
        'S' => "...",
        'T' => "-",
        'U' => "..-",
        'V' => "...-",
        'W' => ".--",
        'X' => "-..-",
        'Y' => "-.--",
        'Z' => "--..",
        ' ' => " ",
        _ => "",
    };

    for signal in morse_code.chars() {
        match signal {
            '.' => {
                let _ = led.set_high();
                arduino_hal::delay_ms(DOT_MS);
            },
            '-' => {
                let _ = led.set_high();
                arduino_hal::delay_ms(DASH_MS);
            },
            ' ' => {
                arduino_hal::delay_ms(DASH_MS * 2);
            },
            _ => (),
        }
        let _ = led.set_low();
        arduino_hal::delay_ms(DOT_MS);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d8.into_output().downgrade();

    loop {
        flash_letter('Y', &mut led);
        flash_letter('a', &mut led);
        flash_letter('K', &mut led);

        arduino_hal::delay_ms(1000);
    }
}
