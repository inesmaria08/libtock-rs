#![no_main]
#![no_std]
use libtock::alarm::{Alarm, Milliseconds};
use libtock::runtime::{set_main, stack_size};
use libtock::servo::Servo;
use libtock_platform::yield_id;

set_main! {main}
stack_size! {0x200}

fn main() {
    for i in 0..180 {
        let _ = Servo::set_angle(1, i);
    }
}
