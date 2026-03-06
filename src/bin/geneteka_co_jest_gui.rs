#![windows_subsystem = "windows"]

use anyhow::Result;
use fifak_lib::window; 
use slint::ComponentHandle;

slint::include_modules!();

fn main() -> Result<()> {
    let ui= GenetekaCoJestWindow::new()?;

    // 1. Zamykanie
    ui.on_quit(move || {
        let _ = slint::quit_event_loop();
    });

    // 2. Minimalizacja
    let ui_weak_min = ui.as_weak();
    ui.on_minimize(move || {
        if let Some(ui) = ui_weak_min.upgrade() {
            ui.window().set_minimized(true);
        }
    });

    // 3. Przesuwanie - IDENTYCZNIE jak w Twoim GenPkWindow
    let ui_weak_move = ui.as_weak();
    ui.on_window_move(move |dx, dy| {
        if let Some(ui) = ui_weak_move.upgrade() {
            if dx == 0.0 && dy == 0.0 {
                window::start_drag(ui.window());
            } else {
                window::window_move(ui.window(), dx, dy);
            }
        }
    });

    let ui_weak_resize = ui.as_weak();
    ui.on_window_resize(move |direction| {
        if let Some(ui) = ui_weak_resize.upgrade() {
            // Używamy Twojej nowej funkcji z lib/window.rs
            window::window_resize(ui.window(), direction);
        }
    });

    // 4. Szukanie (na razie puste)
    ui.on_search(|text| {
        println!("[*] Szukamy: {}", text);
    });

    ui.run()?;
    Ok(())
}