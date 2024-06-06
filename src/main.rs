use physics_engine_2d::cparticle::{self, CParticle};

fn main() {
    App::run();
}

struct App {
    particle: CParticle,
}

impl App {
    fn new() -> Self {
        App {
            particle: CParticle::new(50, 50),
        }
    }

    fn run() {
        let app = App::new();
        println!("{:?}", app.particle);
    }
}
