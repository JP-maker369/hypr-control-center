use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hyprland Control Center",
        options,
        Box::new(|_cc| Box::new(HyprControlApp::default())),
    )
}

struct HyprControlApp {
    mouse_sensitivity: f32,
    mouse_acceleration: f32,
}

impl Default for HyprControlApp {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.5,
            mouse_acceleration: 0.5,
        }
    }
}

impl eframe::App for HyprControlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hyprland Control Center");

            ui.separator();

            // Mouse Settings
            ui.label("Mouse Settings:");
            ui.horizontal(|ui| {
                ui.label("Sensitivity:");
                ui.add(egui::Slider::new(&mut self.mouse_sensitivity, 0.1..=2.0));
            });
            ui.horizontal(|ui| {
                ui.label("Acceleration:");
                ui.add(egui::Slider::new(&mut self.mouse_acceleration, -1.0..=1.0));
            });
            if ui.button("Apply Mouse Settings").clicked() {
                apply_mouse_settings(self.mouse_sensitivity, self.mouse_acceleration);
            }

            ui.separator();

            // Hyprland Reload
            if ui.button("Reload Hyprland Config").clicked() {
                reload_hyprland_config();
            }
        });
    }
}

// Function to apply mouse settings
fn apply_mouse_settings(sensitivity: f32, acceleration: f32) {
    let command = format!(
        "hyprctl keyword input:sensitivity {} && hyprctl keyword input:accel {}",
        sensitivity, acceleration
    );
    std::process::Command::new("sh")
    .arg("-c")
    .arg(command)
    .spawn()
    .expect("Failed to apply mouse settings");
}

// Function to reload Hyprland config
fn reload_hyprland_config() {
    std::process::Command::new("hyprctl")
    .arg("reload")
    .spawn()
    .expect("Failed to reload Hyprland config");
}
