mod gui;
mod influxdb;
mod trading;

use gui::GuiState;
use iced::Sandbox;
use iced::Settings;

fn main() {
    GuiState::run(Settings::default())
        .expect("Error running the GUI application");
}
