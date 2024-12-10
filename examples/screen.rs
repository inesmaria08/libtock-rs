#![no_main]
#![no_std]
use core::fmt::Write;
use libtock::alarm::Alarm;
use libtock::alarm::Milliseconds;
use libtock::console::Console;
use libtock::display::Display;
use libtock::runtime::{set_main, stack_size};

set_main! {main}
stack_size! {800}

fn main() {
    let _ = writeln!(Console::writer(), "available resolutions\n");
    let resolutions = match Display::get_resolution_modes_count() {
        Ok(val) => val,
        Err(e) => {
            let _ = writeln!(Console::writer(), "{:?}\n", e);
            0
        }
    };
    if resolutions == 0 {
        assert_eq!(0, 1);
    }
    let _ = writeln!(Console::writer(), "{resolutions}\n");
    for index in 0..resolutions {
        let (width, height) = match Display::get_resolution_width_height(index as usize) {
            Ok((width, height)) => (width, height),
            Err(e) => {
                let _ = writeln!(Console::writer(), "{:?}\n", e);
                (0, 0)
            }
        };
        let width = (width, height).0;
        let height = (width, height).1;
        if width == 0 && height == 0 {
            assert_eq!(0, 1);
        }
        let _ = writeln!(Console::writer(), " {width} x {height} \n");
    }

    let _ = writeln!(Console::writer(), "available colors depths\n");
    let pixel_format = match Display::get_pixel_format() {
        Ok(val) => val,
        Err(e) => {
            let _ = writeln!(Console::writer(), "{:?}\n", e);
            0
        }
    };
    if pixel_format == 0 {
        assert_eq!(0, 1);
    }
    for index in 0..pixel_format {
        let format = match Display::pixel_format(index as usize) {
            Ok(val) => val,
            Err(e) => {
                let _ = writeln!(Console::writer(), "{:?}\n", e);
                0
            }
        };
        let _ = writeln!(Console::writer(), "  {format} bbp\n");
    }

    let mut buffer: Option<*mut u8> = None;
    let status = Display::screen_buffer_init(&mut buffer);
    let _ = writeln!(Console::writer(), "screen init\n");
    let _ = match Display::set_brightness(100) {
        Ok(()) => (),
        Err(e) => {
            let _ = writeln!(Console::writer(), "{:?}\n", e);
        }
    };
    let (width, height) = match Display::get_resolution() {
        Ok((width, height)) => (width, height),
        Err(e) => {
            let _ = writeln!(Console::writer(), "{:?}\n", e);
            (0, 0)
        }
    };
    let width = (width, height).0;
    let height = (width, height).1;
    if width == 0 && height == 0 {
        assert_eq!(0, 1);
    };
    let _ = match Display::set_write_frame(0, 0, width, height) {
        Ok(()) => (),
        Err(e) => {
            let _ = writeln!(Console::writer(), "{:?}\n", e);
        }
    };
    let _ = match Display::fill(status, 0x0) {
        Ok(()) => (),
        Err(e) => {
            let _ = writeln!(Console::writer(), "{:?}\n", e);
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

        let _ = match Display::set_rotation(i % 4) {
            Ok(()) => (),
            Err(e) => {
                let _ = writeln!(Console::writer(), "{:?}\n", e);
            }
        };
        let _ = match Display::set_write_frame(10, 20, 30, 30) {
            Ok(()) => (),
            Err(e) => {
                let _ = writeln!(Console::writer(), "{:?}\n", e);
            }
        };
        let _ = match Display::fill(status, 0xF800) {
            Ok(()) => (),
            Err(e) => {
                let _ = writeln!(Console::writer(), "{:?}\n", e);
            }
        };
        let _ = match Display::set_write_frame(88, 20, 30, 30) {
            Ok(()) => (),
            Err(e) => {
                let _ = writeln!(Console::writer(), "{:?}\n", e);
            }
        };
        let _ = match Display::fill(status, 0x0) {
            Ok(()) => (),
            Err(e) => {
                let _ = writeln!(Console::writer(), "{:?}\n", e);
            }
        };

        Alarm::sleep_for(Milliseconds(1000)).unwrap();

        let _ = match Display::set_write_frame(10, 20, 30, 30) {
            Ok(()) => (),
            Err(e) => {
                let _ = writeln!(Console::writer(), "{:?}\n", e);
            }
        };
        let _ = match Display::fill(status, 0x0) {
            Ok(()) => (),
            Err(e) => {
                let _ = writeln!(Console::writer(), "{:?}\n", e);
            }
        };
        let _ = match Display::set_write_frame(88, 20, 30, 30) {
            Ok(()) => (),
            Err(e) => {
                let _ = writeln!(Console::writer(), "{:?}\n", e);
            }
        };
        let _ = match Display::fill(status, 0x07F0) {
            Ok(()) => (),
            Err(e) => {
                let _ = writeln!(Console::writer(), "{:?}\n", e);
            }
        };

        Alarm::sleep_for(Milliseconds(1000)).unwrap();

        let _ = match Display::set_write_frame(0, 0, width, height) {
            Ok(()) => (),
            Err(e) => {
                let _ = writeln!(Console::writer(), "{:?}\n", e);
            }
        };
        let _ = match Display::fill(status, 0x0) {
            Ok(()) => (),
            Err(e) => {
                let _ = writeln!(Console::writer(), "{:?}\n", e);
            }
        };
    }
}
