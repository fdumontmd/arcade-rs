#[macro_use]
extern crate error_chain;

extern crate sdl2;

mod phi;
mod views;

error_chain! {
    links {
        Phi(phi::Error, phi::ErrorKind);
    }
}

fn run() -> Result<()> {
    ::phi::spawn("ArcadeRS Shooter", |_| {
        Box::new(::views::ViewA)
    }).map_err(|e| e.into())
}

quick_main!(run);
