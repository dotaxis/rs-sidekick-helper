// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use clipboard::{ClipboardContext, ClipboardProvider};
use std::{env, error::Error, path::Path, process::{Child, Command}, sync::{Arc, Mutex}, thread, time};
use tauri::{image::Image, menu::{MenuBuilder, MenuItemBuilder}, tray::TrayIconBuilder, Manager, WebviewWindow};

fn main() -> Result<(), Box<dyn Error>>  {
    let sidekick_dir = env::var("SIDEKICK_DIR")?;
    let dll_path = Path::new(&sidekick_dir).join("src/Sidekick.Web/bin/Debug/net8.0/Sidekick.dll");

    let dotnet = Command::new("dotnet")
        .arg(dll_path)
        .current_dir(Path::new(&sidekick_dir).join("src/Sidekick.Web"))
        .spawn()
        .expect("failed to run sidekick");

    let dotnet_handle = Arc::new(Mutex::new(dotnet));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(dotnet_handle.clone())
        .setup(|app| {
            // Give dotnet a chance to start before we open the frontend
            thread::sleep(time::Duration::from_secs(2));

            let quit = MenuItemBuilder::new("Quit")
                .id("quit")
                .build(app)
                .unwrap();

            let window = app.handle().get_webview_window("main").unwrap();
            window.eval("window.location.replace('http://localhost:5000/initialize')").expect("failed to set window location");
            window.hide().expect("failed to hide window");
            
            let menu = MenuBuilder::new(app)
                .items(&[&quit])
                .build()
                .unwrap();

            let _tray = TrayIconBuilder::new()
                .icon(Image::from_path(
                    "/home/dot/projects/rust-projects/sidekick-helper/src-tauri/icons/icon.png",
                )?)
                .menu(&menu)
                .on_menu_event(|app, event| if event.id().as_ref() == "quit" {
                    println!("goodbye");
                    let dotnet_handle = app.state::<Arc<Mutex<Child>>>();
                    kill_dotnet(dotnet_handle);
                    app.exit(0);
                })
                .build(app)?;

            use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};
            println!("Hello world");
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_shortcuts(["alt+d", "alt+space"])?
                    .with_handler(move |_app, shortcut, event| {
                        if event.state == ShortcutState::Pressed {
                            if shortcut.matches(Modifiers::ALT, Code::KeyD) {
                                println!("Alt+D triggered");
                                search_for_item(&window);
                            }
                            if shortcut.matches(Modifiers::ALT, Code::Space) {
                                close_window(&window);
                                println!("Alt+Space triggered");
                            }
                        }
                    })
                    .build(),
            )?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
        // .run(move |app, event| {
        //     if let tauri::RunEvent::ExitRequested { api, .. } = event {
        //         // kill_sidekick(Arc::clone(&sidekick_handle));
        //         for (_label, window) in app.webview_windows() {
        //             window.close().unwrap();
        //         }
        //     }
        // });

    Ok(())
}

#[tauri::command]
fn kill_dotnet(dotnet_handle: tauri::State<Arc<Mutex<Child>>>) {
    // Lock the mutex to get mutable access to the child process
    let mut dotnet = dotnet_handle.lock().unwrap();

    // Kill the child process
    if let Err(e) = dotnet.kill() {
        eprintln!("Failed to kill the sidekick process: {}", e);
    } else {
        println!("Sidekick process killed.");
    }
}

pub fn search_for_item(window: &WebviewWindow) {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

    // Store the original clipboard content
    let original_clip = clipboard.get_contents().ok().unwrap_or_default();

    // Simulate pressing Ctrl + C in the game window using xdotool
    Command::new("xdotool")
        .args(["keydown", "ctrl", "key", "c", "keyup", "ctrl"])
        .output()
        .expect("Failed to execute xdotool command");

    // Sleep for a short period to ensure the clipboard is updated
    thread::sleep(time::Duration::from_millis(100));

    // Retrieve the new clipboard content
    let new_clip = clipboard.get_contents().unwrap();

    // Check if the new clipboard content contains "Item Class"
    if new_clip.contains("Item Class") {
        // Encode the clipboard content to base64
        let encoded_clip = base64::encode(new_clip.as_bytes());

        // Open the URL with the encoded clipboard content
        let url = format!("http://localhost:5000/trade/xurl_{}", encoded_clip);
        let _ = window.eval(&format!("window.location.replace('{}')", url));
        window.show().expect("crash");
    } else {
        println!("Did not detect an item from PoE2");
    }

    clipboard.set_contents(original_clip).unwrap();
}

pub fn close_window(window: &WebviewWindow) {
    let _ = window.eval("window.location.replace('about:blank')");
    let _ = window.hide();
}
