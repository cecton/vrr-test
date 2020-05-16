use sdl2;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowBuildError};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

struct App {
    sdl_context: sdl2::Sdl,
    timer: sdl2::TimerSubsystem,
    running: bool,
    canvas: Canvas<Window>,
}

impl App {
    fn new<F>(build_window: F) -> App
    where
        F: Fn(sdl2::VideoSubsystem) -> Result<Window, WindowBuildError>,
    {
        eprintln!("initializing SDL2...");
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        let timer = sdl_context.timer().unwrap();
        let canvas = build_window(video).unwrap().into_canvas().build().unwrap();

        App {
            sdl_context,
            timer,
            running: true,
            canvas,
        }
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn quit(&mut self) {
        eprintln!("termination requested");
        self.running = false;
    }
}

fn initialize_app() -> App {
    App::new(|video| {
        video
            .window("ROMLauncher", 800, 700)
            .position(0, 0)
            .fullscreen_desktop()
            .build()
    })
}

fn game_loop(mut app: App) {
    let mut event_pump = app.sdl_context.event_pump().unwrap();
    let (w, h) = app.canvas.output_size().unwrap();
    let colors = vec![Color::RGB(0, 0, 0), Color::RGB(255, 255, 255)];
    let mut color_it = colors.iter().cycle().cloned();
    let mut fps_counter = 0;
    let t1 = app.timer.ticks();

    eprintln!("window size: {}x{}", w, h);
    while app.is_running() {
        if let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. }
                | Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    app.quit();
                }
                _ => {}
            }
        }

        app.canvas.set_draw_color(color_it.next().unwrap());
        app.canvas.clear();

        app.canvas.present();

        fps_counter += 1;
    }

    eprintln!("fps: {}", fps_counter * 1000 / (app.timer.ticks() - t1));
}

fn main() {
    let app = initialize_app();
    game_loop(app);
}
