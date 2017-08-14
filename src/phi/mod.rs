#[macro_use]
mod events;

use sdl2::render::Canvas;
use sdl2::video::Window;

error_chain! {
    foreign_links {
        WindowBuildError(::sdl2::video::WindowBuildError);
    }
}

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_space: Space,
    },
    else: {
        quit: Quit { .. },
    }
}

pub struct Phi {
    pub events: Events,
    pub canvas: Canvas<Window>,
}

pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>)
}

pub trait View {
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

pub fn spawn<F>(title: &str, init: F) -> Result<()>
    where F: Fn(&mut Phi) -> Box<View> {
    let sdl_context = ::sdl2::init()?;
    let video = sdl_context.video()?;
    let mut timer = sdl_context.timer()?;

    let window = video.window(title, 800, 600)
        .position_centered()
        .opengl()
        .build()?;

    let mut context = Phi {
        events: Events::new(sdl_context.event_pump()?),
        canvas: window.into_canvas()
            .accelerated()
            .build().map_err(|e| Error::with_chain(e, "cannot get canvas"))?,
    };

    let mut current_view = init(&mut context);

    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    loop {
        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;

        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }

        context.events.pump();

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.canvas.present(),
            ViewAction::Quit => break,
            ViewAction::ChangeView(new_view) =>
                current_view = new_view,
        }
    }

    Ok(())
}
