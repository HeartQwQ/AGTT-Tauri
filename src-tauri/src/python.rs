// src-tauri/src/lib.rs
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::Mutex;
use tauri::path::BaseDirectory;
use tauri::{Manager, RunEvent};
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

// 状态管理结构体
struct PythonProcess {
    pid: Mutex<Option<u32>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .manage(PythonProcess {
            pid: Mutex::new(None),
        })
        // 🔥 在这里启动后端
        .setup(|app| {
            // let python_path = app
            //     .path()
            //     .resolve("resources/python-env/python.exe", BaseDirectory::Resource)?;
            // let script_path = app
            //     .path()
            //     .resolve("resources/scripts/server.py", BaseDirectory::Resource)?;

            // // 启动命令
            // let cmd = app
            //     .shell()
            //     .command(python_path)
            //     .args(&["-u", script_path.to_string_lossy().as_ref()]);

            // let (mut rx, child) = cmd
            //     .spawn()
            //     .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            // let pid = child.pid();
            // // 存入状态，以便退出时清理
            // let state = app.state::<PythonProcess>();
            // *state.pid.lock().unwrap() = Some(pid);

            // // 异步打印日志
            // tauri::async_runtime::spawn(async move {
            //     while let Some(event) = rx.recv().await {
            //         match event {
            //             CommandEvent::Stdout(line) | CommandEvent::Stderr(line) => {
            //                 let log = String::from_utf8_lossy(&line);
            //                 if log.to_lowercase().contains("error") {
            //                     eprintln!("[Python ERR]: {}", log);
            //                 } else {
            //                     println!("[Python LOG]: {}", log);
            //                 }
            //             }
            //             _ => {}
            //         }
            //     }
            // });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![]) // 不需要之前的 start 命令了
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            // 退出时自动杀进程
            // match event {
            //     RunEvent::ExitRequested { .. } => {
            //         let state = app_handle.state::<PythonProcess>();
            //         let pid_guard = state.pid.lock().unwrap();
            //         if let Some(pid) = *pid_guard {
            //             #[cfg(target_os = "windows")]
            //             let _ = Command::new("taskkill")
            //                 .args(["/F", "/PID", &pid.to_string(), "/T"])
            //                 .creation_flags(0x08000000)
            //                 .output();
            //         }
            //     }
            //     _ => {}
            // }
        });
}
