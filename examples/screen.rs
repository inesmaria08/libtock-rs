//! An extremely simple libtock-rs example. Just prints out a message
//! using the Console capsule, then terminates.

#![no_main]
#![no_std]
use core::fmt::Write;
use libtock::alarm::Alarm;
use libtock::alarm::Milliseconds;
use libtock::console::Console;
use libtock::display::Display;
use libtock::runtime::{set_main, stack_size};
// use libtock_display::Display;

set_main! {main}
stack_size! {800}

fn main() {
    writeln!(Console::writer(), "available resolutions\n");
    let resolutions = match Display::get_resolution_modes_count() {
        Ok(val) => val,
        Err(e) => {
            writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            0
        }
    };
    if resolutions == 0 {
        assert_eq!(0, 1);
    }
    writeln!(Console::writer(), "{resolutions}\n");
    let mut index = 0;
    while index < resolutions {
        let (width, height) = match Display::get_resolution_width_height(index as usize) {
            Ok((width, height)) => (width, height),
            Err(e) => {
                writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
                (0, 0)
            }
        };
        let width = (width, height).0;
        let height = (width, height).1;
        if width == 0 && height == 0 {
            assert_eq!(0, 1);
        }
        writeln!(Console::writer(), " {width} x {height} \n");
        index = index + 1;
    }

    writeln!(Console::writer(), "available colors depths\n");
    let pixel_format = match Display::get_pixel_format() {
        Ok(val) => val,
        Err(e) => {
            writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            0
        }
    };
    if pixel_format == 0 {
        assert_eq!(0, 1);
    }
    index = 0;
    while index < pixel_format {
        let format = match Display::pixel_format(index as usize) {
            Ok(val) => val,
            Err(e) => {
                writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
                0
            }
        };
        // let bits = screen_get_bits_per_pixel(format);
        writeln!(Console::writer(), "  {format} bbp\n");
        index = index + 1;
    }

    let mut buffer: Option<*mut u8> = None;
    let status = Display::screen_buffer_init(&mut buffer);
    writeln!(Console::writer(), "screen init\n");
    let error = match Display::set_brightness(100) {
        Ok(()) => (),
        Err(e) => {
            writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
        }
    };
    let (width, height) = match Display::get_resolution() {
        Ok((width, height)) => (width, height),
        Err(e) => {
            writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            (0, 0)
        }
    };
    let width = (width, height).0;
    let height = (width, height).1;
    if width == 0 && height == 0 {
        assert_eq!(0, 1);
    };
    let error = match Display::set_write_frame(0, 0, width, height) {
        Ok(()) => (),
        Err(e) => {
            writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
        }
    };
    let error = match Display::fill(status, 0x0) {
        Ok(()) => (),
        Err(e) => {
            writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
        }
    };
    let mut invert = false;
    for i in 0.. {
        if i % 4 == 3 {
            invert = !invert;
            if invert {
                let _ = Display::set_invert_on();
            } else {
                let _ = Display::set_invert_off();
            }
        }

        let error = match Display::set_rotation(i % 4) {
            Ok(()) => (),
            Err(e) => {
                writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            }
        };
        let error = match Display::set_write_frame(10, 20, 30, 30) {
            Ok(()) => (),
            Err(e) => {
                writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            }
        };
        let error = match Display::fill(status, 0xF800) {
            Ok(()) => (),
            Err(e) => {
                writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            }
        };
        let error = match Display::set_write_frame(88, 20, 30, 30) {
            Ok(()) => (),
            Err(e) => {
                writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            }
        };
        let error = match Display::fill(status, 0x0) {
            Ok(()) => (),
            Err(e) => {
                writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            }
        };

        Alarm::sleep_for(Milliseconds(1000)).unwrap();

        let error = match Display::set_write_frame(10, 20, 30, 30) {
            Ok(()) => (),
            Err(e) => {
                writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            }
        };
        let error = match Display::fill(status, 0x0) {
            Ok(()) => (),
            Err(e) => {
                writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            }
        };
        let error = match Display::set_write_frame(88, 20, 30, 30) {
            Ok(()) => (),
            Err(e) => {
                writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            }
        };
        let error = match Display::fill(status, 0x07F0) {
            Ok(()) => (),
            Err(e) => {
                writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            }
        };

        Alarm::sleep_for(Milliseconds(1000)).unwrap();

        let error = match Display::set_write_frame(0, 0, width, height) {
            Ok(()) => (),
            Err(e) => {
                writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            }
        };
        let error = match Display::fill(status, 0x0) {
            Ok(()) => (),
            Err(e) => {
                writeln!(Console::writer(), "\n\n\n   {:?}\n\n\n", e);
            }
        };
    }
}
