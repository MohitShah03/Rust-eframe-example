use eframe::egui::{self, Align2, Color32, Context, FontId, Pos2};

struct PointerApp {
    pointer_pos: Pos2,
}

impl Default for PointerApp {
    fn default() -> Self {
        Self {
            pointer_pos: Pos2::new(0.0, 0.0),
        }
    }
}

impl eframe::App for PointerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            if let Some(pos) = i.pointer.latest_pos() {
                self.pointer_pos = pos;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.painter().text(
                self.pointer_pos + egui::Vec2::new(10.0, 10.0), // Offset near pointer
                Align2::LEFT_TOP,
                "(❁´◡`❁)",
                FontId::monospace(16.0),
                Color32::WHITE,
            );
        });

        ctx.request_repaint(); // Keep the app running
    }
}
fn main() -> Result<(), eframe::Error> {
    let option = eframe::NativeOptions::default();
    eframe::run_native(
        "Pointer App",
        option,
        Box::new(|_cc| Box::new(PointerApp::default())),
    )
}
