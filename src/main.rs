#[macro_use]
extern crate error_chain;
extern crate sdl2;

use sdl2::pixels::Color;
use std::thread;
use std::time::Duration;

error_chain! {
    foreign_links {
        WindowBuildError(::sdl2::video::WindowBuildError);
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

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    sdl_context.event_pump()?.pump_events();

    thread::sleep(Duration::from_millis(3000));

    Ok(())
}

quick_main!(run);
