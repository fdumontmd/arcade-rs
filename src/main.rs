#[macro_use]
extern crate error_chain;
extern crate sdl2;

#[macro_use]
mod events;

use sdl2::pixels::Color;

error_chain! {
    foreign_links {
        WindowBuildError(::sdl2::video::WindowBuildError);
    }
}

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
    },
    else: {
        quit: Quit { .. }
    }
}

fn run() -> Result<()> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;

    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered()
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas()
        .accelerated()
        .build().map_err(|e| Error::with_chain(e, "cannot get canvas"))?;

    let mut events = Events::new(sdl_context.event_pump()?);

    loop {
        events.pump();

        if events.now.quit || events.now.key_escape == Some(true) {
            break;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
    }

    Ok(())
}

quick_main!(run);
