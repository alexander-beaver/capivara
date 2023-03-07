use winapi::shared::windef::RECT;
use winapi::um::winuser;
use winapi::um::wingdi;
use crate::caviomorph::RawImage;

#[cfg(windows)]
pub fn capture(display_id: u32, x: i32, y: i32, w: i32, h:i32) -> RawImage{
    println!("Capturing screen");
let mut rect = RECT{
        left: x,
        top: y,
        right: x + w,
        bottom: y + h
    };

    let mut dc = unsafe { winuser::GetDC(winuser::GetDesktopWindow()) };
    let mut memdc = unsafe { wingdi::CreateCompatibleDC(dc) };
    let mut bmp = unsafe { wingdi::CreateCompatibleBitmap(dc, w, h) };
    let mut old_bmp = unsafe { wingdi::SelectObject(memdc, bmp as *mut _) };
    unsafe { wingdi::BitBlt(memdc, 0, 0, w, h, dc, x, y, wingdi::SRCCOPY) };
    unsafe { winuser::GetWindowRect(winuser::GetDesktopWindow(), &mut rect) };
    println!("{} {} {} {}", rect.left, rect.top, rect.right, rect.bottom);
    let mut image = RawImage{
        width: w as u16,
        height: h as u16,
        pixels: Vec::new()
    };
    println!("Starting pixel loop");
    for i in 0..h{
        for j in 0..w{
            let mut pixel:u32 = 0;
            unsafe { pixel = wingdi::GetPixel(memdc, j, i) };

            image.pixels.push(pixel);
        }
    }
    println!("Finished pixel loop");

    unsafe { wingdi::SelectObject(memdc, old_bmp) };
    unsafe { wingdi::DeleteObject(bmp as *mut _) };
    unsafe { wingdi::DeleteDC(memdc) };
    unsafe { winuser::ReleaseDC(winuser::GetDesktopWindow(), dc) };
    image
}