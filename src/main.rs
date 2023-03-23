extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::framerate::FPSManager;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

mod game_screen;
mod ai;

use crate::game_screen::game_screen::GameScreen;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 480)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    // ミキサーの初期化
    let _audio = sdl_context.audio();
    if let Err(e) = sdl2::mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1_024) {
        println!("Failure to open audio: {}", e);
    }
    let _mixer_context = sdl2::mixer::init(InitFlag::MP3);
    sdl2::mixer::allocate_channels(8);

    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(60).map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut game_screen = GameScreen::new(&texture_creator, &ttf_context);
    game_screen.start_game();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        game_screen.draw(&mut canvas);
        game_screen.on_next_frame(&event_pump);

        fps_manager.delay();
    }

    Ok(())
}