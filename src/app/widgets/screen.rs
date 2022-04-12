use crate::interpreter::*;
use eframe::egui::{Color32, Pos2, Rect, Response, Rounding, Sense, Ui, Vec2, Widget};

pub(crate) struct Chip8Screen<'a> {
    state: &'a Chip8InterpreterState,
}

impl<'a> Chip8Screen<'a> {
    pub fn new(state: &'a Chip8InterpreterState) -> Self {
        Chip8Screen { state }
    }
}

impl Widget for Chip8Screen<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(640.0, 320.0),
            Sense {
                click: false,
                drag: false,
                focusable: false,
            },
        );
        let painter = ui.painter_at(rect);

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                painter.rect_filled(
                    Rect::from_min_size(
                        Pos2::new(rect.left() + 10.0 * x as f32, rect.top() + 10.0 * y as f32),
                        Vec2::new(10.0, 10.0),
                    ),
                    Rounding::none(),
                    if self.state.screen[y][x] == 0 {
                        Color32::BLACK
                    } else {
                        Color32::DARK_GREEN
                    },
                );
            }
        }

        response
    }
}
