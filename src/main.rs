#![windows_subsystem = "windows"]
mod ui;
mod tab;

fn main() {
    let app= ui::FSwitch::new();
    app.run();
}
