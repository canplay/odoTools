#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::Error;
use chrono::Local;
use ini::Ini;
use std::{env, ffi::c_void, io, mem, thread, time::Duration};
use tauri::{generate_context, Builder, Manager, RunEvent, Window};
use tracing::{info, instrument, Level};
use tracing_appender::rolling;
use tracing_subscriber::{
    fmt::{format::Writer, time::FormatTime, Layer},
    layer::SubscriberExt,
    EnvFilter,
};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation,
        System::{
            Diagnostics::{Debug, ToolHelp},
            Registry, Threading,
        },
        UI::{Shell::*, WindowsAndMessaging::*},
    },
};

#[derive(Clone, Debug)]
struct ConfigService {
    typ: String,
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

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

struct LocalTimer;
impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

fn read_patch(h_handle: Foundation::HANDLE, address: *mut u8, buffer: *mut c_void) -> bool {
    let mut size = 0;
    let address_ptr = address as *mut _ as *mut c_void;

    let result = unsafe { Debug::ReadProcessMemory(h_handle, address_ptr, buffer, 64, &mut size) };

    if !result.as_bool() {
        unsafe { Foundation::CloseHandle(h_handle) };
        return false;
    }

    return true;
}

fn write_patch(h_handle: Foundation::HANDLE, address: *mut u8, buffer: *mut c_void) -> bool {
    let mut size = 0;
    let address_ptr = address as *mut _ as *mut c_void;

    let result = unsafe { Debug::WriteProcessMemory(h_handle, address_ptr, buffer, 4, &mut size) };

    if !result.as_bool() {
        unsafe { Foundation::CloseHandle(h_handle) };
        return false;
    }

    return true;
}

fn run_game(path: &str, username: &str, password: &str) -> Result<bool, String> {
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

    if !h.is_invalid() {
        result = true;
    }

    return Ok(result);

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

    if !h.is_invalid() {
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
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    std::env::set_current_dir(&path)?;

    let file_appender = rolling::daily(".", "log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::registry()
        .with(
            EnvFilter::from_default_env()
                .add_directive(Level::ERROR.into())
                .add_directive(Level::WARN.into())
                .add_directive(Level::INFO.into()),
        )
        .with(
            Layer::new().with_writer(io::stdout).event_format(
                tracing_subscriber::fmt::format()
                    .with_level(true)
                    .with_target(true)
                    .with_timer(LocalTimer),
            ),
        )
        .with(
            Layer::new()
                .with_writer(non_blocking)
                .event_format(
                    tracing_subscriber::fmt::format()
                        .with_level(true)
                        .with_target(true)
                        .with_timer(LocalTimer),
                )
                .with_ansi(false),
        );
    tracing::subscriber::set_global_default(subscriber).expect("unable to set a global collector");

    info!("========== launcher start ==========");

    let app = Builder::default()
        .on_page_load(move |window, _| {
            window.set_focus().unwrap();

            let path = path.clone();
            let w = window.clone();
            window.listen("js_ready", move |_| {
                w.clone()
                    .emit(
                        "status",
                        Payload {
                            message: "init".into(),
                        },
                    )
                    .expect("failed to emit event");

                // debug
                // window.open_devtools();

                // path.pop();
                // path.pop();
                // debug

                let args: Vec<String> = env::args().collect();

                // odolauncher://localhost:12347&1&1/
                if args.len() > 1 && args[1].starts_with("odolauncher://") {
                    let v: Vec<&str> = args[1].split(&['/', '&'][..]).collect();
                    let address: Vec<&str> = v[2].split(&[':'][..]).collect();

                    let config = Ini::load_from_file(format!("{}/service.ini", &path.display()));

                    if config.is_ok() {
                        let mut c = config.unwrap();
                        let section_service = c.section(Some("SERVICE")).unwrap();
                        let section_gt = c.section(Some("GT")).unwrap();
                        let section_version = c.section(Some("VERSION")).unwrap();

                        let _config_service = ConfigService {
                            typ: section_service.get("TYPE").unwrap().to_string(),
                            res: section_service.get("RES").unwrap().to_string(),
                        };

                        let _config_gt = ConfigGt {
                            authentic_domain: section_gt
                                .get("AUTHENTIC_DOMAIN")
                                .unwrap()
                                .to_string(),
                            authentic_port: section_gt.get("AUTHENTIC_PORT").unwrap().to_string(),
                            patch_url: section_gt.get("PATCH_URL").unwrap().to_string(),
                            view_trade_market_url: section_gt
                                .get("viewTradeMarketUrl")
                                .unwrap()
                                .to_string(),
                            game_trade_market_url: section_gt
                                .get("gameTradeMarketUrl")
                                .unwrap()
                                .to_string(),
                        };

                        let _config_version = ConfigVersion {
                            launcher: section_version.get("launcher").unwrap().to_string(),
                            client: section_version.get("client").unwrap().to_string(),
                            resource: section_version.get("resource").unwrap().to_string(),
                        };

                        c.with_section(Some("GT"))
                            .set("AUTHENTIC_DOMAIN", address[0])
                            .set("AUTHENTIC_PORT", address[1]);

                        c.with_section(Some("VERSION"))
                            .set("launcher", "1.0.0.0")
                            .set("client", "1.0.0.0")
                            .set("resource", "1.0.0.0");
                    } else {
                        let mut c = Ini::new();
                        c.with_section(Some("SERVICE"))
                            .set("TYPE", "GT")
                            .set("RES", "_EN_");

                        c.with_section(Some("GT"))
                            .set("AUTHENTIC_DOMAIN", address[0])
                            .set("AUTHENTIC_PORT", address[1])
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

                        c.with_section(Some("VERSION"))
                            .set("launcher", "1.0.0.0")
                            .set("client", "1.0.0.0")
                            .set("resource", "1.0.0.0");
                        c.write_to_file(format!("{}/service.ini", &path.display()))
                            .unwrap();
                    }

                    let r = run_game(
                        &format!(
                            "{}/bin64/BlackDesert64.exe",
                            env::current_dir().unwrap().display()
                        ),
                        v[3],
                        v[4],
                    )
                    .unwrap();

                    if r {
                        w.clone()
                            .emit(
                                "status",
                                Payload {
                                    message: "launch_success".into(),
                                },
                            )
                            .expect("failed to emit event");

                        info!("========== launch game client success! ==========");
                    } else {
                        w.clone()
                            .emit(
                                "status",
                                Payload {
                                    message: "launch_failed".into(),
                                },
                            )
                            .expect("failed to emit event");

                        info!("========== launch game client failed! ==========");
                    }
                } else {
                    let r = register_protocol().unwrap();

                    let config =
                        Ini::load_from_file(format!("{}/service.ini", &path.display())).unwrap();
                    let section_gt = config.section(Some("GT")).unwrap();

                    webbrowser::open(&format!(
                        "http://{}:12347",
                        section_gt.get("AUTHENTIC_DOMAIN").unwrap().to_string()
                    ))
                    .unwrap();

                    if r {
                        w.clone()
                            .emit(
                                "status",
                                Payload {
                                    message: "protocol_success".into(),
                                },
                            )
                            .expect("failed to emit event");

                        info!("========== register url protocol success! ==========");
                    } else {
                        w.clone()
                            .emit(
                                "status",
                                Payload {
                                    message: "protocol_failed".into(),
                                },
                            )
                            .expect("failed to emit event");

                        info!("========== register url protocol failed! ==========");
                    }
                }
            });
        })
        .build(generate_context!())
        .expect("error while building application");

    app.run(|_, e| match e {
        RunEvent::ExitRequested { .. } => {
            info!("========== launcher end ==========");
        }
        _ => {}
    });

    Ok(())
}
