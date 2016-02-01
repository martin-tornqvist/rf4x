extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::Sdl;
use sdl2::render::Renderer;

use cmn_types::*;

pub struct Window<'a>
{
    sdl_rend: Renderer<'a>
}

impl<'a> Window<'a>
{
    pub fn new(sdl : &Sdl, d : P) -> Window
    {
        let video_ss = sdl.video().unwrap();

        let sdl_window = video_ss.window("rf4x", d.x, d.y)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut sdl_rend = sdl_window.renderer()
            .build()
            .unwrap();

        sdl_rend.set_draw_color(Color::RGB(0, 0, 0));

        Window
        {
            sdl_rend: sdl_rend
        }
    }

    pub fn draw(&mut self)
    {
        self.sdl_rend.clear();
        self.sdl_rend.present();
    }
}
