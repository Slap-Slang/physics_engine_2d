use cvector::cvector::CVector;
use physics_engine_2d::cparticle::CParticle;

fn main() {
    App::run();
}

struct App {
    particle: CParticle,
    gravity: CVector,
}

impl App {
    fn new() -> Self {
        App {
            particle: CParticle::new(50, 50),
            gravity: CVector::new(0, -1),
        }
    }

    fn run() {
        let mut app = App::new();
        println!("{:?}", app.particle);
        for _ in 1..10 {
            app.particle.add_force(&app.gravity);
            app.particle.apply_forces();
            println!("{:?}", app.particle);
        }
    }
}
