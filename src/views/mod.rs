use phi::{Phi, View, ViewAction};

use sdl2::pixels::Color;

pub struct DefaultView;

impl View for DefaultView {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let canvas = &mut context.canvas;
        let events = &context.events;

        if events.now.quit || events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        ViewAction::None
    }
}

pub struct ViewA;

impl View for ViewA {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let canvas = &mut context.canvas;
        let events = &context.events;

        if events.now.quit || events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if events.now.key_space == Some(true) {
            return ViewAction::ChangeView(Box::new(ViewB));
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        ViewAction::None
    }
}

pub struct ViewB;

impl View for ViewB {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let canvas = &mut context.canvas;
        let events = &context.events;

        if events.now.quit || events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if events.now.key_space == Some(true) {
            return ViewAction::ChangeView(Box::new(ViewA));
        }

        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.clear();

        ViewAction::None
    }
}
