use libtock_platform::ErrorCode;
use libtock_unittest::fake;

type Screen = super::Display<fake::Syscalls>;

#[test]
fn no_driver() {
    let _kernel = fake::Kernel::new();
    assert_eq!(Screen::exists(), Err(ErrorCode::Fail))
}
#[test]
fn exists() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::exists(), Ok(()));
}

#[test]
fn screen_setup() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::screen_setup(), Ok(3));
}

#[test]
fn set_power() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::set_power(1), Ok(()));
}

#[test]
fn set_brightness() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::set_brightness(90), Ok(()));
}

#[test]
fn set_invert_on() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::set_invert_on(), Ok(()));
}

#[test]
fn set_invert_off() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::set_invert_off(), Ok(()));
}

#[test]
fn set_invert() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::set_invert(2), Ok(()));
}

#[test]
fn get_resolution_modes_count() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::get_resolution_modes_count(), Ok(2));
}

#[test]
fn get_resolution_width_height() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::get_resolution_width_height(1), Ok((1920, 1080)));
}

#[test]
fn pixel_modes_count() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::pixel_modes_count(), Ok(5));
}

#[test]
fn pixel_format() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::pixel_format(1), Ok(332));
}

#[test]
fn set_rotation() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::set_rotation(30), Ok(()));
}

#[test]
fn get_rotation() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::set_rotation(30), Ok(()));
    assert_eq!(Screen::get_rotation(), Ok(30));
}

#[test]
fn set_resolution() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::set_resolution(360, 720), Ok(()));
}

#[test]
fn get_resolution() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::set_resolution(360, 720), Ok(()));
    assert_eq!(Screen::get_resolution(), Ok((360, 720)));
}

#[test]
fn set_pixel_format() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::set_pixel_format(565), Ok(()));
}

#[test]
fn get_pixel_format() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::set_pixel_format(565), Ok(()));
    assert_eq!(Screen::get_pixel_format(), Ok(565));
}

#[test]
fn set_write_frame() {
    let kernel = fake::Kernel::new();
    let driver = fake::Screen::new();
    kernel.add_driver(&driver);
    assert_eq!(Screen::set_write_frame(4660, 22136, 39612, 57072), Ok(()));
}
