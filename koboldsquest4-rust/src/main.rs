//extern crate sdl2;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
////use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use sdl2::image::{InitFlag, LoadTexture};
////use sdl2_image::{InitFlag, LoadTexture};



pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
//    let _image_context = sdl2_image::init;

    let window = video_subsystem
        .window("Demon's wizard - Kobold's Quest 4", 800, 600)
	.position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let png = "./pix/firebrand-right-32x32.png";

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(png)?;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        // Set the background
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.clear();

        // Draw a red rectangle
//        canvas.set_draw_color(Color::RGB(255, 0, 0));
//        canvas.fill_rect(Rect::new(100, 100, 600, 400))?;

	canvas.copy(&texture, None, None)?;

        // Show it on the screen
        canvas.present();
    }

    Ok(())
}
