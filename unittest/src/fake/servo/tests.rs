use crate::fake;
use fake::servo::*;
use libtock_platform::CommandReturn;

// Tests the command implementation.
#[test]
fn command() {
    use fake::SyscallDriver;
    let servo = Servo::<1>::new();
    let value = servo.command(SERVO_EXISTS, 0, 0);
    assert_eq!(CommandReturn::is_success(&value), true);
    assert_eq!(
        CommandReturn::is_success(&servo.command(SET_ANGLE, 0, 90)),
        true
    );
    assert_eq!(
        CommandReturn::get_success_u32(&servo.command(GET_ANGLE, 0, 0)),
        Some(90)
    );
}

#[test]
fn kernel_integration() {
    use libtock_platform::Syscalls;
    let kernel = fake::Kernel::new();
    let servo = Servo::<1>::new();
    kernel.add_driver(&servo);
    let value = fake::Syscalls::command(DRIVER_NUM, SERVO_EXISTS, 0, 0);
    assert_eq!(CommandReturn::is_success(&value), true);
    assert_eq!(
        fake::Syscalls::command(DRIVER_NUM, SET_ANGLE, 2, 90).get_failure(),
        Some(ErrorCode::NoDevice)
    );
    assert_eq!(
        fake::Syscalls::command(DRIVER_NUM, SET_ANGLE, 0, 181).get_failure(),
        Some(ErrorCode::Fail)
    );
    assert!(fake::Syscalls::command(DRIVER_NUM, SET_ANGLE, 0, 90).is_success());
    assert_eq!(
        fake::Syscalls::command(DRIVER_NUM, GET_ANGLE, 0, 0).get_success_u32(),
        Some(90)
    );
    assert_eq!(
        fake::Syscalls::command(DRIVER_NUM, GET_ANGLE, 2, 0).get_failure(),
        Some(ErrorCode::NoDevice)
    );
}
