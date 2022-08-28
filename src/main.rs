extern crate sdl2;
pub mod variables;

const CELL_SIZE: u32 = 64;
const WINDOW_PADDING: u32 = 40;
const WINDOW_SIZE: u32 = CELL_SIZE * 8 + WINDOW_PADDING * 2;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("pc sim", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();//code above inits sdl2 somehow, idk what it does

    'running: loop{
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running
                }
                _ => {}
            }
        }
    }
}
