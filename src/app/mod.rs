mod widgets;

use eframe::egui::{
    Align, Color32, FontSelection, Pos2, Rect, RichText, Rounding, Sense, TextEdit, TextStyle,
    Vec2, Widget,
};
use eframe::{egui, epi};

use crate::app::widgets::*;
use crate::interpreter::*;
use crate::programs::PROGRAMS;

pub struct TemplateApp {
    interpreter: Chip8Interpreter,
    running: bool,
    lock_disassembly_to_pc: bool,
    disassembly_starts_at_one: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            interpreter: Chip8Interpreter::new(),
            running: false,
            lock_disassembly_to_pc: true,
            disassembly_starts_at_one: false,
        }
    }
}

impl TemplateApp {
    fn handle_input(&mut self, ctx: &egui::Context) {
        let input = ctx.input();
        let mut keys: u32 = 0;
        if input.key_down(egui::Key::Num1) {
            keys |= 1u32 << 0x1;
        }
        if input.key_down(egui::Key::Num2) {
            keys |= 1u32 << 0x2;
        }
        if input.key_down(egui::Key::Num3) {
            keys |= 1u32 << 0x3;
        }
        if input.key_down(egui::Key::Num4) {
            keys |= 1u32 << 0xc;
        }
        if input.key_down(egui::Key::Q) {
            keys |= 1u32 << 0x4;
        }
        if input.key_down(egui::Key::W) {
            keys |= 1u32 << 0x5;
        }
        if input.key_down(egui::Key::E) {
            keys |= 1u32 << 0x6;
        }
        if input.key_down(egui::Key::R) {
            keys |= 1u32 << 0xd;
        }
        if input.key_down(egui::Key::A) {
            keys |= 1u32 << 0x7;
        }
        if input.key_down(egui::Key::S) {
            keys |= 1u32 << 0x8;
        }
        if input.key_down(egui::Key::D) {
            keys |= 1u32 << 0x9;
        }
        if input.key_down(egui::Key::F) {
            keys |= 1u32 << 0xe;
        }
        if input.key_down(egui::Key::Z) {
            keys |= 1u32 << 0xa;
        }
        if input.key_down(egui::Key::X) {
            keys |= 1u32 << 0x0;
        }
        if input.key_down(egui::Key::C) {
            keys |= 1u32 << 0xb;
        }
        if input.key_down(egui::Key::V) {
            keys |= 1u32 << 0xf;
        }

        self.interpreter.set_input_keys(keys);
    }
}

impl epi::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        if self.running {
            self.handle_input(ctx);
            for _ in 0..20 {
                self.interpreter.tick();
            }
            ctx.request_repaint();
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        self.interpreter.reset();
                        if let Err(e) = self.interpreter.try_load_rom(&PROGRAMS[0].data) {
                            println!("Unable to load rom: {:?}", e);
                        }
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Window::new("Screen").show(ctx, |ui| {
                let state = self.interpreter.state();
                ui.add(Chip8Screen::new(&state));
            });

            egui::Window::new("Interpreter").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("🔁").clicked() {
                        self.interpreter.reset();
                    }
                    if ui.button("⏵").clicked() {
                        self.interpreter.tick();
                    }

                    let toggle_run_icon = if self.running { "⏸" } else { "▶" };
                    if ui.button(toggle_run_icon).clicked() {
                        self.running = !self.running;
                    }
                });

                ui.separator();
                ui.label("Registers");

                egui::Grid::new("register_view")
                    .striped(true)
                    .show(ui, |ui| {
                        let state = self.interpreter.state();
                        for i in 0..REGISTER_COUNT {
                            ui.monospace(format!("V{:x}: {:3}", i, state.registers[i]));
                            if i > 0 && i % 4 == 3 {
                                ui.end_row();
                            } else {
                                ui.monospace(" | ".to_string());
                            }
                        }
                    });

                ui.separator();
                ui.label("Special Registers");

                ui.horizontal(|ui| {
                    let state = self.interpreter.state();
                    ui.monospace(format!("PC: {:04x}", state.pc));
                    ui.monospace(format!(" | I: {:04x}", state.i));
                    ui.monospace(format!(" | ST: {:3}", state.st));
                    ui.monospace(format!(" | DT: {:3}", state.dt));
                });

                ui.separator();
                ui.label("Stack");

                {
                    let state = self.interpreter.state();
                    ui.monospace(format!("SP: {:2}", state.sp));
                }
                egui::ScrollArea::vertical()
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        let state = self.interpreter.state();
                        for i in 0..STACK_SIZE {
                            if i == state.sp {
                                ui.monospace(format!("{:02}: {:04x}  ⬅", i, state.stack[i]));
                            } else {
                                ui.monospace(format!("{:02}: {:04x}", i, state.stack[i]));
                            }
                        }
                    });
            });

            egui::Window::new("Disassembly").show(ctx, |ui| {
                let state = self.interpreter.state();
                let row_count = (MEMORY_SIZE as usize / 2) + 1;

                if self.lock_disassembly_to_pc {
                    self.disassembly_starts_at_one = ((state.pc as usize) & 1) == 1;
                }
                ui.horizontal(|ui| {
                    ui.checkbox(
                        &mut self.disassembly_starts_at_one,
                        "Disassembly starts at 0001",
                    );
                    ui.checkbox(
                        &mut self.lock_disassembly_to_pc,
                        "Lock disassembly view to PC",
                    );
                });

                egui::ScrollArea::vertical()
                    .id_source("disassembly_view")
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        if self.running {
                            ui.monospace("Disassembly is unavailable while running");
                        } else {
                            for row in 0..row_count {
                                let address =
                                    row * 2 + if self.disassembly_starts_at_one { 1 } else { 0 };
                                let text = if let Ok(opcode) =
                                    self.interpreter.try_read_instruction(address)
                                {
                                    format!("{:04x}:  {}", address, opcode)
                                } else {
                                    format!("{:04x}:", address)
                                };
                                let mut label = RichText::new(text).monospace();
                                if address == (state.pc as usize) {
                                    label = label.background_color(Color32::BLUE);
                                }

                                let response = ui.label(label);
                                if self.lock_disassembly_to_pc && address == (state.pc as usize) {
                                    response.scroll_to_me(Some(Align::Center));
                                }
                            }
                        }
                    });
            });

            egui::Window::new("Memory").show(ctx, |ui| {
                let state = self.interpreter.state();
                egui::ScrollArea::vertical()
                    .id_source("memory_view")
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        if self.running {
                            ui.monospace("Memory view is unavailable while running");
                        } else {
                            egui::Grid::new("hex_view").striped(true).show(ui, |ui| {
                                for (row_start, row_data) in state.memory.chunks(16).enumerate() {
                                    ui.monospace(format!("{:04x}  ", row_start * 16));

                                    ui.horizontal(|ui| {
                                        for (i, byte) in row_data.iter().enumerate() {
                                            if i == 7 {
                                                ui.monospace(format!("{:02x} ", byte));
                                            } else {
                                                ui.monospace(format!("{:02x}", byte));
                                            }
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        ui.monospace(" ");
                                        ui.monospace(
                                            row_data
                                                .iter()
                                                .map(|&c| {
                                                    if c.is_ascii_graphic() {
                                                        c as char
                                                    } else {
                                                        '·'
                                                    }
                                                })
                                                .collect::<String>(),
                                        );
                                    });

                                    ui.end_row();
                                }
                            });
                        }
                    });
            });
        });
    }

    fn setup(
        &mut self,
        _ctx: &egui::Context,
        frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        frame.set_window_size(Vec2::new(1100.0, 800.0));
    }

    fn name(&self) -> &str {
        "Chippie"
    }
}
