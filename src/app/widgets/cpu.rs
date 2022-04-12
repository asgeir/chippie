use crate::interpreter::*;
use eframe::egui::{self, Color32, Pos2, Rect, Response, Rounding, Sense, Ui, Vec2, Widget};

pub(crate) struct Chip8Cpu<'a> {
    state: &'a Chip8InterpreterState,
}

impl<'a> Chip8Cpu<'a> {
    pub fn new(state: &'a Chip8InterpreterState) -> Self {
        Chip8Cpu { state }
    }
}

impl Widget for Chip8Cpu<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.allocate_response(
            egui::vec2(100.0, 200.0),
            egui::Sense {
                click: false,
                drag: false,
                focusable: false,
            },
        );
        // ui.horizontal(|ui| {
        //     if ui.button("🔁").clicked() {
        //         self.interpreter.reset();
        //     }
        //     if ui.button("⏵").clicked() {
        //         self.interpreter.tick();
        //     }
        //
        //     let toggle_run_icon = if self.running { "⏸" } else { "▶" };
        //     if ui.button(toggle_run_icon).clicked() {
        //         self.running = !self.running;
        //     }
        // });
        //
        // ui.separator();
        // ui.label("Registers");
        //
        // egui::Grid::new("register_view")
        //     .striped(true)
        //     .show(ui, |ui| {
        //         for i in 0..REGISTER_COUNT {
        //             ui.monospace(format!("V{:x}: {:3}", i, state.registers[i]));
        //             if i > 0 && i % 4 == 3 {
        //                 ui.end_row();
        //             } else {
        //                 ui.monospace(" | ".to_string());
        //             }
        //         }
        //     });
        //
        // ui.separator();
        // ui.label("Special Registers");
        //
        // ui.horizontal(|ui| {
        //     ui.monospace(format!("PC: {:04x}", state.pc));
        //     ui.monospace(format!(" | I: {:04x}", state.i));
        //     ui.monospace(format!(" | ST: {:3}", state.st));
        //     ui.monospace(format!(" | DT: {:3}", state.dt));
        // });
        //
        // ui.separator();
        // ui.label("Stack");
        //
        // ui.monospace(format!("SP: {:2}", state.sp));
        // egui::ScrollArea::vertical()
        //     .auto_shrink([false, true])
        //     .show(ui, |ui| {
        //         for i in 0..STACK_SIZE {
        //             if i == state.sp {
        //                 ui.monospace(format!("{:02}: {:04x}  ⬅", i, state.stack[i]));
        //             } else {
        //                 ui.monospace(format!("{:02}: {:04x}", i, state.stack[i]));
        //             }
        //         }
        //     });

        response
    }
}
