#[macro_use]
mod events;

use sdl2::render::{Canvas, RenderTarget};

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
    },
    else: {
        quit: Quit { .. },
    }
}

pub struct Phi<T: RenderTarget> {
    pub events: Events,
    pub renderer: Canvas<T>,
}

pub enum ViewAction {
    None,
    Quit,
}

pub trait View<T: RenderTarget> {
    fn render(&mut self, context: &mut Phi<T>, elapsed: f64) -> ViewAction;
}
