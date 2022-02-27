use std::{io::Error, mem, thread::sleep, time::Duration};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::*,
        System::Diagnostics::ToolHelp,
        UI::{Shell::*, WindowsAndMessaging::*},
    },
};

fn test() -> Result<i32, Error> {
    unsafe {
        // ShellExecuteA(
        //     HWND::default(),
        //     PCSTR(b"open\0".as_ptr()),
        //     PCSTR(b"BlackDesert64.exe\0".as_ptr()),
        //     PCSTR(b"1, 1\0".as_ptr()),
        //     PCSTR(b"E:\\Black Desert\\Client\\bin64\\\0".as_ptr()),
        //     SHOW_WINDOW_CMD as i32,
        // );
        ShellExecuteA(
            HWND::default(),
            PCSTR(b"open\0".as_ptr()),
            PCSTR(b"Notepad.exe\0".as_ptr()),
            PCSTR::default(),
            PCSTR(b"C:\\Program Files\\WindowsApps\\Microsoft.WindowsNotepad_11.2112.32.0_x64__8wekyb3d8bbwe\\Notepad\0".as_ptr()),
            SW_SHOWNORMAL.0.try_into().unwrap(),
        );
    }

    sleep(Duration::from_secs(3));

    let hsnapshot = unsafe { ToolHelp::CreateToolhelp32Snapshot(ToolHelp::TH32CS_SNAPPROCESS, 0) };

    if hsnapshot != INVALID_HANDLE_VALUE {
        let mut process: ToolHelp::PROCESSENTRY32 = ToolHelp::PROCESSENTRY32::default();
        process.dwSize = mem::size_of::<ToolHelp::PROCESSENTRY32>().count_ones();

        let mut ret = unsafe { ToolHelp::Process32First(hsnapshot, &mut process) };

        while ret.as_bool() {
            println!(
                "sanpshot: {:?} ret: {:?} process: {:?}",
                hsnapshot,
                ret.as_bool(),
                process.th32ProcessID
            );

            ret = unsafe { ToolHelp::Process32Next(hsnapshot, &mut process) };
        }
    }

    Ok(0)
}

fn main() {
    test().unwrap();
}
