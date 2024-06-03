use arboard::Clipboard;
use rdev::{listen, EventType, Key};
use std::collections::HashSet;
use std::fs;
use std::fs::create_dir_all;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use tauri::SystemTrayMenuItem;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

// Файл збереження історії буферу обміну
const HISTORY_FILE: &str = "../config/clipboard_history.json";

// Перевірка існування теки та створення її, якщо вона не існує
fn ensure_config_dir() {
    if let Some(parent) = std::path::Path::new(HISTORY_FILE).parent() {
        create_dir_all(parent).expect("Unable to create config directory");
    }
}

// Збереження історії буферу обміну в файл
fn save_history(history: &Vec<String>) {
    ensure_config_dir();
    let json = serde_json::to_string(history).unwrap();
    fs::write(HISTORY_FILE, json).expect("Unable to write history file");
}

// Завантаження історії буферу обміну з файлу
fn load_history() -> Vec<String> {
    if let Ok(json) = fs::read_to_string(HISTORY_FILE) {
        serde_json::from_str(&json).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

// Додає текст до історії буфера обміну
fn add_to_clipboard_history(
    text: String,
    clipboard_history: &mut Vec<String>,
    sender: &Sender<()>,
) {
    if clipboard_history.len() >= 5 {
        clipboard_history.remove(0);
    }
    clipboard_history.push(text);
    save_history(clipboard_history);
    sender.send(()).unwrap();
}

// Створює меню системного tray на основі історії буфера обміну та стану прослуховування
fn create_tray_menu(clipboard_history: &Vec<String>, is_listen: bool) -> SystemTrayMenu {
    let mut tray_menu = SystemTrayMenu::new();
    if clipboard_history.is_empty() {
        tray_menu = tray_menu
            .add_item(CustomMenuItem::new("empty".to_string(), "Copy something...").disabled());
    } else {
        for (index, item) in clipboard_history.iter().enumerate() {
            let mut title = item.clone();
            if title.chars().count() > 15 {
                title = title.chars().take(15).collect::<String>() + "...";
            }
            let button = CustomMenuItem::new(index.to_string(), title);
            tray_menu = tray_menu.add_item(button);
        }
    }
    let listen_stop_text = if is_listen {
        "Pause listening"
    } else {
        "Listen"
    };
    let listen_stop_button =
        CustomMenuItem::new("change_listen_state".to_string(), listen_stop_text);
    let clear = CustomMenuItem::new("clear".to_string(), "Clear");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    tray_menu
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(listen_stop_button)
        .add_item(clear)
        .add_item(quit)
}

fn main() {
    // Ініціалізація змінних відповідальних за слухання буферу обміну
    let is_listen: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
    let just_started_listening: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    // Змінна що відповідає за ініціалізацію історії буферу обміну
    let initial_history = load_history();

    // Ініціалізація змінних відповідальних за
    let clipboard_history: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(initial_history));
    let pressed_keys = Arc::new(Mutex::new(HashSet::new()));
    let pressed_keys_clone = pressed_keys.clone();
    let (sender, receiver): (Sender<()>, Receiver<()>) = mpsc::channel();

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_clipboard::init())
        .system_tray(SystemTray::new().with_menu(create_tray_menu(
            &clipboard_history.lock().unwrap(),
            *is_listen.lock().unwrap(),
        )))
        .on_system_tray_event({
            let clipboard_history = clipboard_history.clone();
            let is_listen = is_listen.clone();
            let just_started_listening = just_started_listening.clone();
            move |app, event| {
                if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                    let mut clipboard_history = clipboard_history.lock().unwrap();
                    match id.as_str() {
                        "change_listen_state" => {
                            let mut is_listen_guard = is_listen.lock().unwrap();
                            *is_listen_guard = !*is_listen_guard;
                            println!("Change listen state on: {}", *is_listen_guard);
                            if *is_listen_guard {
                                let mut just_started_listening_guard =
                                    just_started_listening.lock().unwrap();
                                *just_started_listening_guard = true;
                            }
                            let tray_menu = create_tray_menu(&clipboard_history, *is_listen_guard);
                            app.tray_handle().set_menu(tray_menu).unwrap();
                        }
                        "clear" => {
                            clipboard_history.clear();
                            save_history(&clipboard_history);
                            let tray_menu =
                                create_tray_menu(&clipboard_history, *is_listen.lock().unwrap());
                            app.tray_handle().set_menu(tray_menu).unwrap();
                        }
                        "quit" => {
                            std::process::exit(0);
                        }
                        _ => {
                            if let Ok(index) = id.parse::<usize>() {
                                if let Some(item) = clipboard_history.get(index) {
                                    println!("Menu item {} clicked: {}", index, item);
                                    let mut clipboard = Clipboard::new().unwrap();
                                    clipboard.set_text(item.clone()).unwrap();
                                }
                            }
                        }
                    }
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    let app_handle = app.handle();

    // Потік для прослуховування клавіш
    std::thread::spawn({
        let sender_clone = sender.clone();
        let is_listen_clone = is_listen.clone();
        let clipboard_history_clone_for_thread = clipboard_history.clone();
        move || {
            listen(move |event| {
                let mut keys = pressed_keys_clone.lock().unwrap();
                match event.event_type {
                    EventType::KeyPress(key) => {
                        keys.insert(key);
                        if keys.contains(&Key::ControlLeft) && keys.contains(&Key::KeyC) {
                            println!("Ctrl+C pressed!");
                            let is_listen = is_listen_clone.clone();
                            let clipboard_history = clipboard_history_clone_for_thread.clone();
                            let sender = sender_clone.clone();

                            if *is_listen.lock().unwrap() {
                                thread::spawn(move || {
                                    thread::sleep(Duration::from_millis(500));
                                    if *is_listen.lock().unwrap() {
                                        let mut clipboard = Clipboard::new().unwrap();
                                        let current_text = clipboard.get_text().unwrap_or_default();
                                        let mut clipboard_history =
                                            clipboard_history.lock().unwrap();
                                        add_to_clipboard_history(
                                            current_text,
                                            &mut clipboard_history,
                                            &sender,
                                        );
                                    }
                                });
                            }
                        }
                    }
                    EventType::KeyRelease(key) => {
                        keys.remove(&key);
                    }
                    _ => {}
                }
            })
            .unwrap();
        }
    });

    // Потік для оновлення меню системного tray при зміні історії буфера обміну
    std::thread::spawn({
        let clipboard_history_clone_for_thread = clipboard_history.clone();
        let is_listen_clone = is_listen.clone();
        move || {
            while let Ok(_) = receiver.recv() {
                let clipboard_history = clipboard_history_clone_for_thread.lock().unwrap();
                let tray_menu =
                    create_tray_menu(&clipboard_history, *is_listen_clone.lock().unwrap());
                let _ = app_handle.tray_handle().set_menu(tray_menu);
            }
        }
    });

    // Запуск Tauri додатку
    app.run(|_, _| {});
}
