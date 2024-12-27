use crate::DriverInfo;
use core::cell::Cell;
use libtock_platform::{CommandReturn, ErrorCode};

pub struct Screen {
    screen_setup: Option<u16>,
    resolution_modes: Option<u16>,
    invert: [Cell<bool>; 1],
    screen_resolution_width_height: Option<(u16, u16)>,
    resolution_width_height: [Cell<u16>; 2],
    pixel_modes: Option<u16>,
    screen_pixel_format: Option<u16>,
    pixel_format: [Cell<u32>; 1],
    brightness: [Cell<u16>; 1],
    rotation: [Cell<u16>; 1],
    write_frame: [Cell<u16>; 2],
    power: [Cell<u16>; 1],
}

impl Screen {
    pub fn new() -> std::rc::Rc<Screen> {
        #[allow(clippy::declare_interior_mutable_const)]
        const VALUE_U16: Cell<u16> = Cell::new(0);
        #[allow(clippy::declare_interior_mutable_const)]
        const VALUE_U32: Cell<u32> = Cell::new(0);
        const VALUE_BOOL: Cell<bool> = Cell::new(false);
        std::rc::Rc::new(Screen {
            screen_setup: std::option::Option::Some(3),
            screen_pixel_format: std::option::Option::Some(332),
            pixel_format: [VALUE_U32],
            resolution_modes: std::option::Option::Some(2),
            screen_resolution_width_height: std::option::Option::Some((1920, 1080)),
            resolution_width_height: [VALUE_U16, VALUE_U16],
            invert: [VALUE_BOOL],
            brightness: [VALUE_U16],
            pixel_modes: std::option::Option::Some(5),
            rotation: [VALUE_U16],
            write_frame: [VALUE_U16, VALUE_U16],
            power: [VALUE_U16],
        })
    }
}

impl crate::fake::SyscallDriver for Screen {
    fn info(&self) -> DriverInfo {
        DriverInfo::new(DRIVER_NUM)
    }

    fn command(&self, command_num: u32, argument0: u32, argument1: u32) -> CommandReturn {
        match command_num {
            EXISTS => crate::command_return::success(),
            SCREEN_SETUP => crate::command_return::success_u32(self.screen_setup.unwrap() as u32),

            SET_POWER => {
                self.power[0].set(1);
                crate::command_return::success()
            }
            GET_POWER => crate::command_return::success_u32(self.power[0].get() as u32),

            SET_BRIGHTNESS => {
                self.brightness[0].set(argument0 as u16);
                crate::command_return::success()
            }
            GET_BRIGHTNESS => crate::command_return::success_u32(self.brightness[0].get() as u32),

            SET_INVERT_ON => {
                self.invert[0].set(argument0 != 0);
                crate::command_return::success()
            }

            SET_INVERT_OFF => {
                self.invert[0].set(argument0 != 0);
                crate::command_return::success()
            }

            SET_INVERT => {
                self.invert[0].set(argument0 != 0);
                crate::command_return::success()
            }

            GET_INVERT => crate::command_return::success_u32(self.invert[0].get() as u32),

            GET_RESOLUTION_MODES_COUNT => {
                if self.screen_setup != None {
                    crate::command_return::success_u32(self.resolution_modes.unwrap() as u32)
                } else {
                    crate::command_return::failure(ErrorCode::NoSupport)
                }
            }

            GET_RESOLUTION_WIDTH_HEIGHT => {
                if self.screen_setup != None {
                    crate::command_return::success_2_u32(
                        self.screen_resolution_width_height.unwrap().0 as u32,
                        self.screen_resolution_width_height.unwrap().1 as u32,
                    )
                } else {
                    crate::command_return::failure(ErrorCode::NoSupport)
                }
            }

            PIXEL_MODES_COUNT => {
                if self.screen_setup != None {
                    crate::command_return::success_u32(self.pixel_modes.unwrap() as u32)
                } else {
                    crate::command_return::failure(ErrorCode::NoSupport)
                }
            }

            PIXEL_FORMAT => {
                if self.screen_setup != None {
                    crate::command_return::success_u32(self.screen_pixel_format.unwrap() as u32)
                } else {
                    crate::command_return::failure(ErrorCode::NoSupport)
                }
            }

            GET_ROTATION => crate::command_return::success_u32(self.rotation[0].get() as u32),

            SET_ROTATION => {
                self.rotation[0].set(argument0 as u16);
                crate::command_return::success()
            }

            GET_RESOLUTION => crate::command_return::success_2_u32(
                self.resolution_width_height[0].get() as u32,
                self.resolution_width_height[1].get() as u32,
            ),

            SET_RESOLUTION => {
                self.resolution_width_height[0].set(argument0 as u16);
                self.resolution_width_height[1].set(argument1 as u16);
                crate::command_return::success()
            }

            GET_PIXEL_FORMAT => crate::command_return::success_2_u32(
                self.pixel_format[0].get() as u32,
                self.pixel_format[1].get() as u32,
            ),

            SET_PIXEL_FORMAT => {
                if Some(argument0) != None {
                    self.pixel_format[0].set(argument0);
                    crate::command_return::success()
                } else {
                    crate::command_return::failure(ErrorCode::Invalid)
                }
            }

            SET_WRITE_FRAME => {
                self.write_frame[0].set(argument0 as u16);
                self.write_frame[1].set(argument1 as u16);
                crate::command_return::success()
            }

            GET_WRITE_FRAME => crate::command_return::success_2_u32(
                self.write_frame[0].get() as u32,
                self.write_frame[1].get() as u32,
            ),

            // to do
            WRITE => crate::command_return::success(),

            FILL => crate::command_return::success(),
            _ => return crate::command_return::failure(ErrorCode::NoSupport),
        }
    }
}

// -----------------------------------------------------------------------------
// Implementation details below
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests;

const DRIVER_NUM: u32 = 0x90001;

// Command IDs
#[allow(unused)]

pub const EXISTS: u32 = 0;
pub const SCREEN_SETUP: u32 = 1;
pub const SET_POWER: u32 = 2;
pub const SET_BRIGHTNESS: u32 = 3;
pub const SET_INVERT_ON: u32 = 4;
pub const SET_INVERT_OFF: u32 = 5;
pub const SET_INVERT: u32 = 6;
pub const GET_RESOLUTION_MODES_COUNT: u32 = 11;
pub const GET_RESOLUTION_WIDTH_HEIGHT: u32 = 12;
pub const PIXEL_MODES_COUNT: u32 = 13;
pub const PIXEL_FORMAT: u32 = 14;
pub const GET_ROTATION: u32 = 21;
pub const SET_ROTATION: u32 = 22;
pub const GET_RESOLUTION: u32 = 23;
pub const SET_RESOLUTION: u32 = 24;
pub const GET_PIXEL_FORMAT: u32 = 25;
pub const SET_PIXEL_FORMAT: u32 = 26;
pub const SET_WRITE_FRAME: u32 = 100;
pub const WRITE: u32 = 200;
pub const FILL: u32 = 300;
pub const GET_POWER: u32 = 400;
pub const GET_BRIGHTNESS: u32 = 401;
pub const GET_INVERT: u32 = 402;
pub const GET_WRITE_FRAME: u32 = 403;
