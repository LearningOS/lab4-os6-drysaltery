use super::File;
use crate::fs::StatMode;
use crate::mm::{UserBuffer};
use crate::sbi::console_getchar;
use crate::task::suspend_current_and_run_next;

/// The standard input
pub struct Stdin;
/// The standard output
pub struct Stdout;

impl File for Stdin {
    fn fstat(&self) -> (u64, super::StatMode, u32) {
        println!("[ERRPR]: using fstat in Stdin!!!");
        (0, StatMode::NULL, 0)
    }
    // fn fstat(&self) -> u64 {
    //     321
    // }
    fn readable(&self) -> bool { true }
    fn writable(&self) -> bool { false }
    fn read(&self, mut user_buf: UserBuffer) -> usize {
        assert_eq!(user_buf.len(), 1);
        // busy loop
        let mut c: usize;
        loop {
            c = console_getchar();
            if c == 0 {
                suspend_current_and_run_next();
                continue;
            } else {
                break;
            }
        }
        let ch = c as u8;
        unsafe { user_buf.buffers[0].as_mut_ptr().write_volatile(ch); }
        1
    }
    fn write(&self, _user_buf: UserBuffer) -> usize {
        panic!("Cannot write to stdin!");
    }
}

impl File for Stdout {
    // fn fstat(&self) -> u64 {
    //     123
    // }
    fn fstat(&self) -> (u64, super::StatMode, u32) {
        println!("[ERRPR]: using fstat in Stdout!!!");
        (0, StatMode::NULL, 0)
    }
    fn readable(&self) -> bool { false }
    fn writable(&self) -> bool { true }
    fn read(&self, _user_buf: UserBuffer) -> usize{
        panic!("Cannot read from stdout!");
    }
    fn write(&self, user_buf: UserBuffer) -> usize {
        for buffer in user_buf.buffers.iter() {
            print!("{}", core::str::from_utf8(*buffer).unwrap());
        }
        user_buf.len()
    }
}
