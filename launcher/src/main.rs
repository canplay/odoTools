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

use anyhow::Error;
use chrono::Local;
use ini::Ini;
use std::{env, ffi::c_void, io};
use tracing::{info, instrument};
use tracing_appender::rolling;
use tracing_subscriber::{
    fmt::{self, format::Writer, time::FormatTime},
    layer::SubscriberExt,
    EnvFilter,
};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation,
        System::Registry,
        UI::{Shell::*, WindowsAndMessaging::*},
    },
};

#[derive(Clone, Debug)]
struct ConfigService {
    types: String,
    res: String,
}

#[derive(Clone, Debug)]
struct ConfigGt {
    authentic_domain: String,
    authentic_port: String,
    patch_url: String,
    view_trade_market_url: String,
    game_trade_market_url: String,
}

#[derive(Clone, Debug)]
struct ConfigVersion {
    launcher: String,
    client: String,
    resource: String,
}

struct LocalTimer;
impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

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

fn run_game(path: &str, username: &str, password: &str) -> Result<bool, Error> {
    let mut result = false;
    let s = path.to_string();
    let len_end = s.rfind("/").unwrap();
    let dir = s.get(0..len_end + 1).unwrap();
    let h: Foundation::HINSTANCE;

    unsafe {
        h = ShellExecuteA(
            Foundation::HWND::default(),
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

    if h.is_invalid() {
        result = true;
    }

    Ok(result)
}

fn register_protocol() -> Result<bool, Error> {
    let mut result = false;
    let mut h_key: Registry::HKEY = Registry::HKEY::default();
    let path = std::env::current_exe().unwrap().display().to_string();

    if unsafe {
        Registry::RegCreateKeyA(
            Registry::HKEY_CLASSES_ROOT,
            PCSTR("ODOLauncher\0".as_ptr()),
            &mut h_key,
        )
    } == Foundation::ERROR_SUCCESS
    {
        if unsafe {
            Registry::RegSetKeyValueA(
                h_key,
                PCSTR("\0".as_ptr()),
                PCSTR("URL Protocol\0".as_ptr()),
                Registry::REG_SZ.0,
                "\0".as_ptr() as *mut c_void,
                2,
            )
        } == Foundation::ERROR_SUCCESS
        {
            if unsafe {
                Registry::RegSetValueA(
                    h_key,
                    PCSTR("\0".as_ptr()),
                    Registry::REG_SZ,
                    PCSTR("URL:ODO Launcher Protocol\0".as_ptr()),
                    0,
                )
            } == Foundation::ERROR_SUCCESS
            {
                unsafe { Registry::RegCloseKey(h_key) };

                if unsafe {
                    Registry::RegCreateKeyA(
                        Registry::HKEY_CLASSES_ROOT,
                        PCSTR("ODOLauncher\\DefaultIcon\0".as_ptr()),
                        &mut h_key,
                    )
                } == Foundation::ERROR_SUCCESS
                {
                    if unsafe {
                        Registry::RegSetValueA(
                            h_key,
                            PCSTR("\0".as_ptr()),
                            Registry::REG_SZ,
                            PCSTR(format!("\"{}\", 0\0", path).as_ptr()),
                            0,
                        )
                    } == Foundation::ERROR_SUCCESS
                    {
                        unsafe { Registry::RegCloseKey(h_key) };

                        if unsafe {
                            Registry::RegCreateKeyA(
                                Registry::HKEY_CLASSES_ROOT,
                                PCSTR("ODOLauncher\\shell\\open\\command\0".as_ptr()),
                                &mut h_key,
                            )
                        } == Foundation::ERROR_SUCCESS
                        {
                            if unsafe {
                                Registry::RegSetValueA(
                                    h_key,
                                    PCSTR("\0".as_ptr()),
                                    Registry::REG_SZ,
                                    PCSTR(format!("\"{}\" \"%1\"\0", path).as_ptr()),
                                    0,
                                )
                            } == Foundation::ERROR_SUCCESS
                            {
                                result = true;
                            }
                        }
                    }
                }
            }
        }
        unsafe { Registry::RegCloseKey(h_key) };
    }

    Ok(result)
}

#[instrument]
fn main() -> Result<(), Error> {
    let file_appender = rolling::daily("E:/Black Desert/Client", "log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(
            fmt::Layer::new().with_writer(io::stdout).event_format(
                tracing_subscriber::fmt::format()
                    .with_level(true)
                    .with_target(true)
                    .with_timer(LocalTimer),
            ),
        )
        .with(
            fmt::Layer::new()
                .with_writer(non_blocking)
                .event_format(
                    tracing_subscriber::fmt::format()
                        .with_level(true)
                        .with_target(true)
                        .with_timer(LocalTimer),
                )
                .with_ansi(false),
        );
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global collector");

    info!("========== Start ==========");

    info!(
        "{:?} {:?}",
        std::env::current_dir().unwrap().display(),
        std::env::current_exe().unwrap().as_path()
    );

    let config = Ini::load_from_file("service.ini");

    if config.is_ok() {
        let c = config.unwrap();
        let section_service = c.section(Some("SERVICE")).unwrap();
        let section_gt = c.section(Some("GT")).unwrap();
        let section_version = c.section(Some("VERSION")).unwrap();

        let config_service = ConfigService {
            types: section_service.get("TYPE").unwrap().to_string(),
            res: section_service.get("RES").unwrap().to_string(),
        };
        let config_gt = ConfigGt {
            authentic_domain: section_gt.get("AUTHENTIC_DOMAIN").unwrap().to_string(),
            authentic_port: section_gt.get("AUTHENTIC_PORT").unwrap().to_string(),
            patch_url: section_gt.get("PATCH_URL").unwrap().to_string(),
            view_trade_market_url: section_gt.get("viewTradeMarketUrl").unwrap().to_string(),
            game_trade_market_url: section_gt.get("gameTradeMarketUrl").unwrap().to_string(),
        };

        let config_version = ConfigVersion {
            launcher: section_version.get("launcher").unwrap().to_string(),
            client: section_version.get("client").unwrap().to_string(),
            resource: section_version.get("resource").unwrap().to_string(),
        };
    } else {
        let mut c = config.unwrap();
        c.with_section(Some("SERVICE"))
            .set("TYPE", "GT")
            .set("RES", "_EN_");

        c.with_section(Some("GT"))
            .set("AUTHENTIC_DOMAIN", "127.0.0.1")
            .set("AUTHENTIC_PORT", "9888")
            .set(
                "PATCH_URL",
                "http://dn.global-lab.playblackdesert.com/UploadData/",
            )
            .set(
                "viewTradeMarketUrl",
                "https://trade.global-lab.playblackdesert.com/",
            )
            .set(
                "gameTradeMarketUrl",
                "https://game-trade.global-lab.playblackdesert.com/",
            );
        c.write_to_file("service.ini").unwrap();
    }

    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        // register
        "register" => {
            let r = register_protocol().unwrap();

            info!(
                "========== {}",
                if r {
                    "register url protocol success!"
                } else {
                    "register url protocol fail!"
                }
            );
        }
        // run 1 1
        "run" => {
            let r = run_game(
                &format!(
                    "{}/bin64/BlackDesert64.exe",
                    env::current_dir().unwrap().display()
                ),
                &args[1],
                &args[2],
            )
            .unwrap();

            info!(
                "========== {}",
                if r {
                    "launch game client success!"
                } else {
                    "launch game client fail!"
                }
            );
        }
        _ => {
            // odolauncher://1&1/
            if args[1].starts_with("odolauncher://") {
                let v: Vec<&str> = args[1].split(&['/', '&'][..]).collect();

                let r = run_game(
                    &format!(
                        "{}/bin64/BlackDesert64.exe",
                        env::current_dir().unwrap().display()
                    ),
                    v[2],
                    v[3],
                )
                .unwrap();

                info!(
                    "========== {}",
                    if r {
                        "launch game client success!"
                    } else {
                        "launch game client fail!"
                    }
                );
            }
        }
    }

    info!("========== End ==========");
    Ok(())
}
