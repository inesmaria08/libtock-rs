#![no_std]

use libtock_platform::{ErrorCode, Syscalls};

pub struct Servo<S: Syscalls>(S);

impl<S: Syscalls> Servo<S> {
    pub fn servo_exists() -> Result<(), ErrorCode> {
        let val = S::command(DRIVER_NUM, SERVO_EXISTS, 0, 0).is_success();
        if val == true {
            Ok(())
        } else {
            //println!("the driver could not be found");
            Err(ErrorCode::Fail)
        }
    }
    pub fn set_angle(index: u32, angle: u32) -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, SET_ANGLE, index, angle).to_result()
    }
    pub fn get_angle(index: u32) -> Result<u32, ErrorCode> {
        S::command(DRIVER_NUM, GET_ANGLE, index, 0).to_result()
    }
}

#[cfg(test)]
mod tests;

// -----------------------------------------------------------------------------
// Driver number and command IDs
// -----------------------------------------------------------------------------

const DRIVER_NUM: u32 = 0x90009;

// Command IDs
const SERVO_EXISTS: u32 = 0;
const SET_ANGLE: u32 = 1;
const GET_ANGLE: u32 = 2;
