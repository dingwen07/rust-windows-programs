use windows::{
    Win32::Foundation::*, Win32::System::Threading::*,
    Win32::System::ProcessStatus::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() {
    let mut window_text: [u16; 512] = [0; 512];

    let hwnd = unsafe { GetForegroundWindow() };
    unsafe { 
        let len = GetWindowTextW(hwnd, PWSTR(window_text.as_mut_ptr()), 512);
        let window_text = String::from_utf16_lossy(&window_text[..len as usize]);
        println!("{}", window_text);
    }
}
