#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

use tauri::State;

#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn log_console(phrase: &str) {
    println!("{}", phrase);
}

#[tauri::command]
fn count_many(times: i32, counter: State<'_, Counter>) -> i32 {
    let mut value = counter.0.lock().unwrap();
    *value += times;
    println!("Counter: {}", *value);
    return *value;
}

#[tauri::command]
async fn open_docs(handle: tauri::AppHandle) {
    tauri::WindowBuilder::new(
        &handle,
        "test", /* the unique window label */
        tauri::WindowUrl::External("https://tauri.app/".parse().unwrap()),
    )
    .build()
    .unwrap();
}

fn main() {
    tauri::Builder::default()
        .manage(Counter(Default::default()))
        .invoke_handler(tauri::generate_handler![
            greet,
            log_console,
            count_many,
            open_docs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
