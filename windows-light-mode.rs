// From: Canvas02
// Depends on: windows["Win32_System_Registry", "Win32_Foundation"]

use windows::{
    w,
    Win32::System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_DWORD},
};

fn main() {
    let is_light_theme = is_light_theme();
    dbg!(is_light_theme);
}

fn is_light_theme() -> [u8; 4] {
    let mut buffer = [0u8; 4];
    let mut cb_buffer = buffer.len() as u32;
    unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            w!(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"),
            w!("AppsUseLightTheme"),
            RRF_RT_REG_DWORD,
            None,
            Some(buffer.as_mut_ptr() as _),
            Some(&mut cb_buffer),
        )
    }
    .to_hresult()
    .unwrap();

    buffer
}
