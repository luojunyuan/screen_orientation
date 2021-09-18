
// use bindings::Windows::Win32::Graphics::Gdi::ChangeDisplaySettingsA;
use bindings::Windows::Win32::Graphics::Gdi::EnumDisplayDevicesA;
use bindings::Windows::Win32::Graphics::Gdi::EnumDisplaySettingsA;
use bindings::Windows::Win32::Graphics::Gdi::DISPLAY_DEVICEA;
use bindings::Windows::Win32::UI::DisplayDevices::DEVMODEA;
use bindings::Windows::Win32::Graphics::Gdi::ENUM_CURRENT_SETTINGS;
use std::mem::size_of;
use bindings::Windows::Win32::Foundation::PSTR;
use bindings::Windows::Win32::System::SystemServices::CHAR;

fn main() -> windows::Result<()> {
    // rotate(1, Orientations::DegreeCW180);
    // let s: String = Orientations::DegreeCW0.to_string();
    // println!(s);

    let mut d = DISPLAY_DEVICEA::default();
    let d_ptr: *mut DISPLAY_DEVICEA = &mut d;
    d.cb = size_of::<DISPLAY_DEVICEA>() as u32;

    unsafe {
        if !EnumDisplayDevicesA(None, 0, d_ptr, 0).as_bool() {
            panic!("Display index(idevnum) is greater than connected displays.");
        } 
    }

    let mut dm = DEVMODEA::default();
    let dm_ptr: *mut DEVMODEA = &mut dm;

    unsafe {
        let this_is_char: CHAR = d.DeviceName[0];

        if false != EnumDisplaySettingsA(this_is_char, ENUM_CURRENT_SETTINGS, dm_ptr).as_bool()
        {

        }
    }

    Ok(())
}

// fn rotate(display_num: u32, orientation: Orientations) {
//     let pending_to_print: String = orientation;
//     println!(pending_to_print);
// }

// // Why
// enum Orientations {
//     DegreeCW0 = 0,
//     DegreeCW90 = 3,
//     DegreeCW180 = 2,
//     DegreeCW270 = 1
// }


// function parameter *mut -> std::ptr::null_mut()
// anthoer situation is None
