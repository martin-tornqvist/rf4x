extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod rend;
mod cmn_types;

use cmn_types::*;

fn main()
{
    println!("main()");

    let sdl = sdl2::init().unwrap();

    let d = P::new(800, 600);

    let mut window = rend::Window::new(&sdl, d);

    window.draw();

    let mut event_pump = sdl.event_pump().unwrap();

    'running: loop
    {
        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::Quit {..} |
                Event::KeyDown
                {
                    keycode: Some(Keycode::Escape), ..
                } =>
                {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }

    println!("main() [DONE]");
}
