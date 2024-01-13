use std::time::Instant;

use crate::search::{search, sort};

#[tauri::command]
pub async fn handle_input(input: String) -> (Vec<String>, f32, i32) {
    let mut result: Vec<String> = Vec::new();
    let start_time = Instant::now();

    if !input.starts_with("/") {
        #[cfg(target_os = "macos")]
        {
            result = search(
                input.as_str(),
                vec![
                    "/Applications",
                    "/System/Applications",
                    "/System/Applications/Utilities",
                ],
                Some(".app"),
                Some(1),
            );
        }

        #[cfg(target_os = "linux")]
        {
            result = search(
                input.as_str(),
                vec![
                    "/usr/bin",
                    "/usr/sbin",
                    "/usr/local/bin",
                    "/usr/local/sbin",
                    "/home",
                ],
                None,
                Some(1),
            );
        }

        #[cfg(target_os = "windows")]
        {
            result = search(
                input.as_str(),
                vec![
                    "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs",
                    "C:\\Users\\%USERNAME%\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs",
                    "C:\\ProgramData\\chocolatey\\bin",
                    "C:\\Program Files",
                    "C:\\Program Files (x86)",
                    "C:\\Users\\%USERNAME%\\AppData\\Local\\Microsoft\\WindowsApps",
                ],
                Some(".exe"),
                Some(1),
            );
        }

        sort(&mut result, input.as_str());
    }

    (result, start_time.elapsed().as_secs_f32(), 1)
}