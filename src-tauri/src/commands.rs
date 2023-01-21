use std::collections::HashMap;
use tauri::Manager;
use windows::Win32::{UI::WindowsAndMessaging::{GWL_EXSTYLE, WS_EX_TRANSPARENT, WS_EX_LAYERED}};
use crate::chrome;

#[tauri::command]
pub async fn get_cookies()-> &'static str{
    let key = chrome::get_key();
    let mut result: HashMap<String, String> = HashMap::new();
    let data = chrome::get_raw_cookies(".bilibili.com").await;
    for (name, encrypted_value) in data.iter() {
        result.insert(name.to_owned(),chrome::decrypt_string(&key, encrypted_value));
    }    
    let json_result = serde_json::json!(result);
    Box::leak(json_result.to_string().into_boxed_str())
}

#[tauri::command]
pub fn get_current_path(app_handle: tauri::AppHandle) -> String{
    let path = app_handle.path_resolver(); 
    println!("resource_dir:{}",path.resource_dir().unwrap().to_str().unwrap());
    return path.resource_dir().unwrap().as_path().to_string_lossy().to_string();
}

#[tauri::command]
pub async fn window_mouse_penetration(app_handle: tauri::AppHandle,window: tauri::Window,label: Option<String>)-> tauri::Result<i32>{
    match label {
        Some(lbl) => {
            let win = app_handle.get_window(&lbl);
            if let Some(w) = win {
                Ok(mouse_penetration(w.hwnd().unwrap().0))
            }else {        
                Err(tauri::Error::FailedToExecuteApi(tauri::api::Error::Command(format!("Window {} not found -9999",lbl))))
            }
        },
        None => {
            Ok(mouse_penetration(window.hwnd().unwrap().0))
        },
    }
}

#[tauri::command]
pub async fn window_cancel_mouse_penetration(app_handle: tauri::AppHandle,window: tauri::Window,label: Option<String>)-> tauri::Result<i32>{
    match label {
        Some(lbl) => {
            let win = app_handle.get_window(&lbl);
            if let Some(w) = win {
                Ok(cancel_mouse_penetration(w.hwnd().unwrap().0))
            }else {        
                Err(tauri::Error::FailedToExecuteApi(tauri::api::Error::Command(format!("Window {} not found -9999",lbl))))
            }
        },
        None => {
            Ok(cancel_mouse_penetration(window.hwnd().unwrap().0))
        },
    }
}

fn mouse_penetration(hwnd: isize)->i32{
    // use windows::Win32::UI::WindowsAndMessaging;
    unsafe{
        // let desktop_hwnd = windows::Win32::UI::WindowsAndMessaging::FindWindowW(windows::core::PCWSTR::null(),  windows::core::PCWSTR::from_raw("Program Manager".encode_utf16().collect::<Vec<u16>>().as_ptr()));
        // windows::Win32::UI::WindowsAndMessaging::SetParent(windows::Win32::Foundation::HWND(hwnd), desktop_hwnd);
        // ::SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOSIZE | SWP_NOMOVE);
        // windows::Win32::UI::WindowsAndMessaging::SetWindowPos(windows::Win32::Foundation::HWND(hwnd), WindowsAndMessaging::HWND_TOPMOST, 0, 0, 0, 0, WindowsAndMessaging::SWP_NOMOVE|WindowsAndMessaging::SWP_NOSIZE);
        // windows::Win32::UI::WindowsAndMessaging::SetWindowPos(windows::Win32::Foundation::HWND(hwnd), WindowsAndMessaging::HWND_TOP, 0, 0, 0, 0, WindowsAndMessaging::SWP_NOMOVE|WindowsAndMessaging::SWP_NOSIZE);
        let extended_style = windows::Win32::UI::WindowsAndMessaging::GetWindowLongW(windows::Win32::Foundation::HWND(hwnd), GWL_EXSTYLE);
        windows::Win32::UI::WindowsAndMessaging::SetWindowLongW(windows::Win32::Foundation::HWND(hwnd), GWL_EXSTYLE, extended_style  | WS_EX_TRANSPARENT.0  as i32 | WS_EX_LAYERED.0  as i32)
    }
}
fn cancel_mouse_penetration(hwnd: isize)->i32{
    unsafe{
        let extended_style = windows::Win32::UI::WindowsAndMessaging::GetWindowLongW(windows::Win32::Foundation::HWND(hwnd), GWL_EXSTYLE);
        windows::Win32::UI::WindowsAndMessaging::SetWindowLongW(windows::Win32::Foundation::HWND(hwnd), GWL_EXSTYLE, extended_style  & !WS_EX_TRANSPARENT.0  as i32 & !WS_EX_LAYERED.0  as i32)
    }
}