use cvector::cvector::CVector;
use physics_engine_2d::{ccircle::CCircle, cparticle::CParticle};

use std::{
    io::{self, stdout, Error, Stdout},
    time::Instant,
};

use crossterm::{
    event::{self, read, KeyCode, KeyEvent, KeyEventKind},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand,
};
// use num::{Float, Integer, ToPrimitive};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::Color,
    symbols::Marker,
    widgets::{canvas::Canvas, Block, Widget},
    Frame, Terminal,
};

fn main() -> Result<(), Error> {
    App::run()
}

struct App {
    ball: CCircle,
    gravity: CVector,

    bounds: Rect,
    tick_cout: u64,
    marker: Marker,
}

impl App {
    fn new() -> Self {
        App {
            ball: CCircle::new(50, 50, 1),
            gravity: CVector::new(0.0, -0.1),

            bounds: Rect::new(10, 10, 200, 100),
            tick_cout: 0,
            marker: Marker::Braille,
        }
    }

    fn run() -> Result<(), Error> {
        let mut app = App::new();
        app.ball.set_vel(1, 0);
        let mut terminal = terminal_init()?;
        terminal.clear()?;

        let mut last_tick = Instant::now();
        let tick_rate = std::time::Duration::from_millis(16);

        loop {
            // ui
            let _ = terminal.draw(|frame| app.ui(frame));

            // events
            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }

            if last_tick.elapsed() > tick_rate {
                app.update();
                last_tick = Instant::now();
            }
        }

        terminal_restore()?;
        Ok(())
    }

    fn ui(&self, frame: &mut Frame) {
        frame.render_widget(self.canvas(), frame.size());
    }

    fn canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title(self.marker.to_string()))
            .background_color(Color::Rgb(100, 100, 100))
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&self.ball.body);
                ctx.print(0.0, 0.0, "Hont");
            })
            .x_bounds([10.0, 210.0])
            .y_bounds([10.0, 110.0])
    }

    fn update(&mut self) {
        self.tick_cout += 1;

        self.ball.add_force(&self.gravity);
        self.ball.apply_forces();
        self.ball.bounce(&self.bounds);
    }
}

fn terminal_init() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn terminal_restore() -> Result<(), Error> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
