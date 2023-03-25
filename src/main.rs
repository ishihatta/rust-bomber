extern crate sdl2;

use sdl2::event::{Event, WindowEvent};
use sdl2::gfx::framerate::FPSManager;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use title_screen::screen::TitleScreen;

mod screen;
mod game_screen;
mod title_screen;
mod ai;

use crate::screen::{Screen, ScreenEvent};
use crate::game_screen::screen::GameScreen;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Bomber mates", 800, 480)
        .position_centered()
        .resizable()
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

    let mut screen: Box<dyn Screen> = Box::new(TitleScreen::new(&texture_creator, &ttf_context));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(w, h) = win_event {
                        // ウィンドウサイズが変更された
                        if let Err(error) = canvas.set_scale(w as f32 / 800f32, h as f32 / 480f32) {
                            println!("Failed to resize window: {}", error);
                        }
                    }
                }
                _ => {}
            }
        }

        // The rest of the game loop goes here...
        screen.draw(&mut canvas);
        match screen.on_next_frame(&event_pump) {
            ScreenEvent::None => (),
            ScreenEvent::GoToGameScreen(player_type1, player_type2) => {
                screen = Box::new(GameScreen::new(&texture_creator, &ttf_context, player_type1, player_type2));
            }
            ScreenEvent::ReturnToTitleScreen => {
                screen = Box::new(TitleScreen::new(&texture_creator, &ttf_context));
            }
        }

        fps_manager.delay();
    }

    Ok(())
}