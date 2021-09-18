fn main() {
    windows::build! {
        Windows::Win32::Graphics::Gdi::EnumDisplayDevicesA,
        Windows::Win32::Graphics::Gdi::EnumDisplaySettingsA,
        Windows::Win32::UI::DisplayDevices::DEVMODEA,
    };
}
