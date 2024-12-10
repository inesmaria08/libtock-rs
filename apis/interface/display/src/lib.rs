#![no_std]
use core::cell::Cell;
// use core::fmt;
use core::marker::PhantomData;
// use core::ptr::addr_of_mut;
use libtock_platform::allow_ro::AllowRo;
use libtock_platform::share;
use libtock_platform::subscribe::Subscribe;
use libtock_platform::{self as platform, DefaultConfig};
use libtock_platform::{ErrorCode, Syscalls};
const BUFFER_SIZE: usize = 10 * 1024;
static mut STATIC_BUFFER: Option<[u8; BUFFER_SIZE]> = None;
pub struct Display<S: Syscalls, C: Config = DefaultConfig>(S, C);

impl<S: Syscalls, C: Config> Display<S, C> {
    #[inline(always)]
    pub fn screen_buffer_init(buffer: &mut Option<*mut u8>) -> &'static mut [u8] {
        if buffer.is_some() {
            panic!("Buffer is already initialized!");
        }
        unsafe {
            if STATIC_BUFFER.is_some() {
                panic!("Buffer is already initialized!");
            }
            STATIC_BUFFER = Some([0; BUFFER_SIZE]);
            if let Some(ref mut initialized_buffer) = STATIC_BUFFER {
                *buffer = Some(initialized_buffer.as_mut_ptr());
                &mut *initialized_buffer
            } else {
                panic!("Buffer initialization failed!");
            }
        }
    }
    pub fn exists() -> bool {
        S::command(DRIVER_NUM, command::EXISTS, 0, 0).is_success()
    }
    pub fn screen_setup() -> Result<u32, ErrorCode> {
        S::command(DRIVER_NUM, command::SCREEN_SETUP, 1, 0).to_result()
    }
    pub fn set_power(value: usize) -> Result<(), ErrorCode> {
        if value != 0 {
            S::command(DRIVER_NUM, command::SET_POWER, value as u32, 0).to_result()
        } else {
            Err(ErrorCode::Invalid)
        }
    }
    pub fn set_brightness(value: usize) -> Result<(), ErrorCode> {
        let called: Cell<Option<(u32,)>> = Cell::new(None);
        share::scope(|subscribe| {
            S::subscribe::<_, _, C, DRIVER_NUM, { subscribe::WRITE }>(subscribe, &called)?;

            S::command(DRIVER_NUM, command::SET_BRIGHTNESS, value as u32, 0).to_result()?;

            loop {
                S::yield_wait();
                if let Some((_,)) = called.get() {
                    return Ok(());
                }
            }
        })
    }
    pub fn set_invert_on() -> Result<(), ErrorCode> {
        let called: Cell<Option<(u32,)>> = Cell::new(None);
        share::scope(|subscribe| {
            S::subscribe::<_, _, C, DRIVER_NUM, { subscribe::WRITE }>(subscribe, &called)?;

            S::command(DRIVER_NUM, command::SET_INVERT_ON, 0, 0).to_result()?;

            loop {
                S::yield_wait();
                if let Some((_,)) = called.get() {
                    return Ok(());
                }
            }
        })
    }
    pub fn set_invert_off() -> Result<(), ErrorCode> {
        let called: Cell<Option<(u32,)>> = Cell::new(None);
        share::scope(|subscribe| {
            S::subscribe::<_, _, C, DRIVER_NUM, { subscribe::WRITE }>(subscribe, &called)?;

            S::command(DRIVER_NUM, command::SET_INVERT_OFF, 0, 0).to_result()?;

            loop {
                S::yield_wait();
                if let Some((_,)) = called.get() {
                    return Ok(());
                }
            }
        })
    }
    pub fn set_invert(value: usize) -> Result<(), ErrorCode> {
        if value != 0 {
            S::command(DRIVER_NUM, command::SET_INVERT, value as u32, 0).to_result()
        } else {
            Err(ErrorCode::Invalid)
        }
    }
    pub fn get_resolution_modes_count() -> Result<u32, ErrorCode> {
        S::command(DRIVER_NUM, command::GET_RESOLUTION_MODES_COUNT, 0, 0).to_result()
    }
    pub fn get_resolution_width_height(index: usize) -> Result<(u32, u32), ErrorCode> {
        S::command(
            DRIVER_NUM,
            command::GET_RESOLUTION_WIDTH_HEIGHT,
            index as u32,
            0,
        )
        .to_result()
    }
    pub fn pixel_modes_count() -> Result<u32, ErrorCode> {
        S::command(DRIVER_NUM, command::PIXEL_MODES_COUNT, 0, 0).to_result()
    }
    pub fn pixel_format(index: usize) -> Result<u32, ErrorCode> {
        S::command(DRIVER_NUM, command::PIXEL_FORMAT, index as u32, 0).to_result()
    }
    pub fn get_rotation() -> Result<u32, ErrorCode> {
        S::command(DRIVER_NUM, command::GET_ROTATION, 0, 0).to_result()
    }
    pub fn set_rotation(rotation: usize) -> Result<(), ErrorCode> {
        let called: Cell<Option<(u32,)>> = Cell::new(None);
        share::scope(|subscribe| {
            S::subscribe::<_, _, C, DRIVER_NUM, { subscribe::WRITE }>(subscribe, &called)?;

            S::command(DRIVER_NUM, command::SET_ROTATION, rotation as u32, 0).to_result()?;
            loop {
                S::yield_wait();
                if let Some((_,)) = called.get() {
                    return Ok(());
                }
            }
        })
    }
    pub fn get_resolution() -> Result<(u32, u32), ErrorCode> {
        S::command(DRIVER_NUM, command::GET_RESOLUTION, 0, 0).to_result()
    }
    pub fn set_resolution(resolution: usize) -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, command::SET_RESOLUTION, resolution as u32, 0).to_result()
    }
    pub fn get_pixel_format() -> Result<u32, ErrorCode> {
        S::command(DRIVER_NUM, command::GET_PIXEL_FORMAT, 0, 0).to_result()
    }
    pub fn set_pixel_format(format: usize) -> Result<(), ErrorCode> {
        S::command(DRIVER_NUM, command::SET_PIXEL_FORMAT, format as u32, 0).to_result()
    }
    pub fn set_write_frame(x: u32, y: u32, width: u32, height: u32) -> Result<(), ErrorCode> {
        let data1: u32 = (((x & 0xFFFF) << (16 as u8)) | (y & 0xFFFF)) as u32;
        let data2: u32 = (((width & 0xFFFF) << (16 as u8)) | (height & 0xFFFF)) as u32;
        let called: Cell<Option<(u32,)>> = Cell::new(None);
        share::scope(|subscribe| {
            S::subscribe::<_, _, C, DRIVER_NUM, { subscribe::WRITE }>(subscribe, &called)?;

            S::command(
                DRIVER_NUM,
                command::SET_WRITE_FRAME,
                data1 as u32,
                data2 as u32,
            )
            .to_result()?;
            loop {
                S::yield_wait();
                if let Some((_,)) = called.get() {
                    return Ok(());
                }
            }
        })
    }
    pub fn write(s: &[u8]) -> Result<(), ErrorCode> {
        let called: Cell<Option<(u32,)>> = Cell::new(None);
        share::scope::<
            (
                AllowRo<_, DRIVER_NUM, { allow_ro::SHARED }>,
                Subscribe<_, DRIVER_NUM, { subscribe::WRITE }>,
            ),
            _,
            _,
        >(|handle| {
            let (allow_ro, subscribe) = handle.split();

            S::allow_ro::<C, DRIVER_NUM, { allow_ro::SHARED }>(allow_ro, s)?;

            S::subscribe::<_, _, C, DRIVER_NUM, { subscribe::WRITE }>(subscribe, &called)?;

            S::command(DRIVER_NUM, command::WRITE, s.len() as u32, 0).to_result()?;

            loop {
                S::yield_wait();
                if let Some((_,)) = called.get() {
                    return Ok(());
                }
            }
        })
    }
    pub fn fill(s: &mut [u8], color: u16) -> Result<(), ErrorCode> {
        if s.len() - 2 > 0 {
            s[0] = ((color >> 8) & 0xFF) as u8;
            s[1] = (color & 0xFF) as u8;
        }
        let called: Cell<Option<(u32,)>> = Cell::new(None);
        share::scope::<
            (
                AllowRo<_, DRIVER_NUM, { allow_ro::SHARED }>,
                Subscribe<_, DRIVER_NUM, { subscribe::WRITE }>,
            ),
            _,
            _,
        >(|handle| {
            let (allow_ro, subscribe) = handle.split();

            S::allow_ro::<C, DRIVER_NUM, { allow_ro::SHARED }>(allow_ro, s)?;

            S::subscribe::<_, _, C, DRIVER_NUM, { subscribe::WRITE }>(subscribe, &called)?;

            S::command(DRIVER_NUM, command::FILL, 0, 0).to_result()?;

            loop {
                S::yield_wait();
                if let Some((_,)) = called.get() {
                    return Ok(());
                }
            }
        })
    }
}
pub trait Config:
    platform::allow_ro::Config + platform::allow_rw::Config + platform::subscribe::Config
{
}
impl<T: platform::allow_ro::Config + platform::allow_rw::Config + platform::subscribe::Config>
    Config for T
{
}
pub struct Screen<S: Syscalls> {
    syscalls: PhantomData<S>,
}
#[cfg(test)]
mod tests;

// -----------------------------------------------------------------------------
// Driver number and command IDs
// -----------------------------------------------------------------------------

const DRIVER_NUM: u32 = 0x90001;

// Command IDs
#[allow(unused)]
mod command {
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
}
mod subscribe {
    pub const WRITE: u32 = 0;
}
mod allow_ro {
    pub const SHARED: u32 = 0;
}
