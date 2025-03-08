use eframe::egui::{self, Align2, Color32, Context, FontId, Pos2, Vec2};
use std::time::{Duration, Instant};

struct PointerApp {
    pointer_pos: Pos2,
    last_move_time: Instant,
    show_text: bool,
}

impl Default for PointerApp {
    fn default() -> Self {
        Self {
            pointer_pos: Pos2::new(0.0, 0.0),
            last_move_time: Instant::now(),
            show_text: false,
        }
    }
}

/// Implementation of the `eframe::App` trait for `PointerApp`.
///
/// This implementation handles the update logic for the application, which includes:
/// - Tracking the pointer position and detecting if it has moved.
/// - Updating the `pointer_pos` and `last_move_time` when the pointer moves.
/// - Showing a text message if the pointer has not moved for a specified duration.
/// - Rendering the text message near the pointer position using `egui`.
/// - Continuously requesting repaint to keep the application running smoothly.
///
/// # Parameters
/// - `ctx`: The `Context` object that provides input and rendering capabilities.
/// - `_frame`: A mutable reference to the `eframe::Frame` object (not used in this implementation).
///
/// # Fields
/// - `pointer_moved`: A boolean flag to track if the pointer has moved.
/// - `pointer_pos`: The current position of the pointer.
/// - `last_move_time`: The last time the pointer was moved.
/// - `show_text`: A boolean flag to determine if the text message should be displayed.
///
/// # Behavior
/// - If the pointer moves, `pointer_pos` and `last_move_time` are updated, and `show_text` is set to false.
/// - If the pointer has not moved for at least 200 milliseconds, `show_text` is set to true.
/// - The text message "You are not moving the pointer" is displayed near the pointer position if `show_text` is true.
/// - The application continuously requests repaint to ensure smooth updates.
impl eframe::App for PointerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let mut pointer_moved = false;

        ctx.input(|i| {
            if let Some(pos) = i.pointer.latest_pos() {
                if pos != self.pointer_pos {
                    self.pointer_pos = pos;
                    self.last_move_time = Instant::now();
                    self.show_text = false;
                    pointer_moved = true;
                }
            }
        });

        // If no movement happened and enough time has passed, show text
        if !pointer_moved && self.last_move_time.elapsed() >= Duration::from_millis(200) {
            self.show_text = true;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_text {
                ui.painter().text(
                    self.pointer_pos + Vec2::new(10.0, 10.0), // Offset near pointer
                    Align2::LEFT_TOP,
                    "You are not moving the pointer",
                    FontId::monospace(16.0),
                    Color32::WHITE,
                );
            }
        });

        ctx.request_repaint(); // Keep app running smoothly
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
