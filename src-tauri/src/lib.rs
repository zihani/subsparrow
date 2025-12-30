// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ip: String,
    pub mac: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_network_interfaces() -> Vec<NetworkInterface> {
    let mut interfaces = Vec::new();
    
    if let Ok(addrs) = if_addrs::get_if_addrs() {
        for iface in addrs {
            // 跳过 loopback 接口
            if iface.is_loopback() {
                continue;
            }
            
            let ip = iface.ip().to_string();
            
            // 获取 MAC 地址（需要根据平台实现）
            let mac = get_mac_address(&iface.name).unwrap_or_else(|| "N/A".to_string());
            
            interfaces.push(NetworkInterface {
                name: iface.name,
                ip,
                mac,
            });
        }
    }
    
    interfaces
}

#[cfg(target_os = "macos")]
fn get_mac_address(interface_name: &str) -> Option<String> {
    use std::process::Command;
    
    let output = Command::new("ifconfig")
        .arg(interface_name)
        .output()
        .ok()?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    
    // 查找 ether 行（macOS 使用 ether）
    for line in output_str.lines() {
        if line.contains("ether") {
            // macOS 格式: "ether 00:11:22:33:44:55"
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                if part == &"ether" && i + 1 < parts.len() {
                    let mac = parts[i + 1];
                    // 验证 MAC 地址格式 (xx:xx:xx:xx:xx:xx)
                    if mac.matches(':').count() == 5 && mac.len() == 17 {
                        return Some(mac.to_string());
                    }
                }
            }
        }
    }
    
    None
}

#[cfg(target_os = "linux")]
fn get_mac_address(interface_name: &str) -> Option<String> {
    use std::fs;
    
    let path = format!("/sys/class/net/{}/address", interface_name);
    fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

#[cfg(target_os = "windows")]
fn get_mac_address(interface_name: &str) -> Option<String> {
    use std::process::Command;
    
    let output = Command::new("getmac")
        .arg("/fo")
        .arg("csv")
        .arg("/nh")
        .output()
        .ok()?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    
    for line in output_str.lines() {
        if line.contains(interface_name) {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                let mac = parts[1].trim().trim_matches('"');
                if !mac.is_empty() {
                    return Some(mac.to_string());
                }
            }
        }
    }
    
    None
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
fn get_mac_address(_interface_name: &str) -> Option<String> {
    None
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, get_network_interfaces])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
