/*
use std::{ffi::c_void, mem, thread, time::Duration};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{self, HANDLE, HWND},
        System::{
            Diagnostics::{Debug, ToolHelp},
            Threading,
        },
        UI::{Shell::*, WindowsAndMessaging::*},
    },
};
*/

use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HWND,
        UI::{Shell::*, WindowsAndMessaging::*},
    },
};

/*
fn read_patch(h_handle: HANDLE, address: *mut u8, buffer: *mut c_void) -> bool {
    let mut size = 0;
    let address_ptr = address as *mut _ as *mut c_void;

    let result = unsafe { Debug::ReadProcessMemory(h_handle, address_ptr, buffer, 64, &mut size) };

    if !result.as_bool() {
        unsafe { Foundation::CloseHandle(h_handle) };
        return false;
    }

    return true;
}

fn write_patch(h_handle: HANDLE, address: *mut u8, buffer: *mut c_void) -> bool {
    let mut size = 0;
    let address_ptr = address as *mut _ as *mut c_void;

    let result = unsafe { Debug::WriteProcessMemory(h_handle, address_ptr, buffer, 4, &mut size) };

    if !result.as_bool() {
        unsafe { Foundation::CloseHandle(h_handle) };
        return false;
    }

    return true;
}
 */

fn run_game(path: &str, username: &str, password: &str) -> Result<bool, String> {
    let result = false;

    let s = path.to_string();
    let len_end = s.rfind("/").unwrap();
    let dir = s.get(0..len_end + 1).unwrap();

    unsafe {
        ShellExecuteA(
            HWND::default(),
            PCSTR("open\0".as_ptr()),
            PCSTR(format!("{}\0", path).as_ptr()),
            PCSTR(format!("{},{}\0", username, password).as_ptr()),
            PCSTR(format!("{}\0", dir).as_ptr()),
            SW_SHOWNORMAL.0.try_into().unwrap(),
        );
    }

    /*
       thread::sleep(Duration::from_secs(5));

       let mut h_snapshot =
           unsafe { ToolHelp::CreateToolhelp32Snapshot(ToolHelp::TH32CS_SNAPPROCESS, 0) };

       if h_snapshot != Foundation::INVALID_HANDLE_VALUE {
           let mut process: ToolHelp::PROCESSENTRY32 = ToolHelp::PROCESSENTRY32::default();
           process.dwSize = mem::size_of::<ToolHelp::PROCESSENTRY32>() as u32;

           let mut ret = unsafe { ToolHelp::Process32First(h_snapshot, &mut process) };

           while ret.as_bool() {
               let mut v = vec![0];
               for ch in process.szExeFile {
                   if ch.0 != 0 {
                       v.push(ch.0);
                   }
               }

               let s = String::from_utf8(v).unwrap();
               if s.contains("BlackDesert64.exe") {
                   unsafe { Foundation::CloseHandle(h_snapshot) };

                   h_snapshot = unsafe {
                       ToolHelp::CreateToolhelp32Snapshot(
                           ToolHelp::TH32CS_SNAPMODULE,
                           process.th32ProcessID,
                       )
                   };

                   if h_snapshot != Foundation::INVALID_HANDLE_VALUE {
                       let mut module: ToolHelp::MODULEENTRY32 = ToolHelp::MODULEENTRY32::default();
                       module.dwSize = mem::size_of::<ToolHelp::MODULEENTRY32>() as u32;

                       ret = unsafe { ToolHelp::Module32First(h_snapshot, &mut module) };
                       while ret.as_bool() {
                           v = vec![];
                           for ch in process.szExeFile {
                               if ch.0 != 0 {
                                   v.push(ch.0);
                               }
                           }

                           let s1 = String::from_utf8(v).unwrap();
                           if s1.contains("BlackDesert64.exe") {
                               unsafe { Foundation::CloseHandle(h_snapshot) };

                               let h_process = unsafe {
                                   Threading::OpenProcess(
                                       Threading::PROCESS_ALL_ACCESS,
                                       false,
                                       module.th32ProcessID,
                                   )
                               };

                               if h_process != Foundation::INVALID_HANDLE_VALUE {
                                   let base_address = module.modBaseAddr;

                                   // Crypto
                                   let address = unsafe { base_address.add(0x0A29306) };

                                   let buffer: *mut c_void = std::ptr::null_mut();
                                   if !read_patch(h_process, address, buffer) {
                                       return Err("Crypto Read".to_string());
                                   }

                                   let buffer: *mut c_void = std::ptr::null_mut();
                                   unsafe { std::ptr::write_bytes(buffer, 0x0000090, 2) };
                                   if !write_patch(h_process, address, buffer) {
                                       return Err("Crypto Write".to_string());
                                   }
                                   //

                                   // XC1
                                   let address = unsafe { base_address.add(0x07A5B0A) };

                                   let buffer: *mut c_void = std::ptr::null_mut();
                                   if !read_patch(h_process, address, buffer) {
                                       return Err("XC1 Read".to_string());
                                   }

                                   let buffer: *mut c_void = std::ptr::null_mut();
                                   unsafe { std::ptr::write_bytes(buffer, 0x0000090, 2) };
                                   if !write_patch(h_process, address, buffer) {
                                       return Err("XC1 Write".to_string());
                                   }
                                   //

                                   // XC2
                                   let address = unsafe { base_address.add(0x07A5BF0) };

                                   let buffer: *mut c_void = std::ptr::null_mut();
                                   if !read_patch(h_process, address, buffer) {
                                       return Err("XC2 Read".to_string());
                                   }

                                   let buffer: *mut c_void = std::ptr::null_mut();
                                   unsafe { std::ptr::write_bytes(buffer, 0x00000EB, 1) };
                                   if !write_patch(h_process, address, buffer) {
                                       return Err("XC2 Write".to_string());
                                   }
                                   //

                                   // Wipe IP
                                   let address = unsafe { base_address.add(0x02B41A38) };

                                   let buffer: *mut c_void = std::ptr::null_mut();
                                   if !read_patch(h_process, address, buffer) {
                                       return Err("Wipe IP Read".to_string());
                                   }

                                   let buffer: *mut c_void = std::ptr::null_mut();
                                   unsafe { std::ptr::write_bytes(buffer, 0x0000000, 14) };
                                   if !write_patch(h_process, address, buffer) {
                                       return Err("Wipe IP Write".to_string());
                                   }
                                   //

                                   // IP
                                   let address = unsafe { base_address.add(0x02B41A38) };

                                   let buffer: *mut c_void = std::ptr::null_mut();
                                   if !read_patch(h_process, address, buffer) {
                                       return Err("IP Read".to_string());
                                   }

                                   let buffer: *mut c_void = std::ptr::null_mut();
                                   unsafe {
                                       for s in "192.168.1.33".bytes() {
                                           std::ptr::write_bytes(buffer, s, 1);
                                       }
                                   }
                                   if !write_patch(h_process, address, buffer) {
                                       return Err("IP Write".to_string());
                                   }
                                   //
                               }

                               break;
                           }

                           ret = unsafe { ToolHelp::Module32Next(h_snapshot, &mut module) };
                       }
                   }
                   break;
               }

               ret = unsafe { ToolHelp::Process32Next(h_snapshot, &mut process) };
           }
       }
    */

    Ok(result)
}

fn main() {
    run_game("E:/Black Desert/Client/bin64/BlackDesert64.exe", "1", "1").unwrap();
}
