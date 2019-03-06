extern crate ray_tracer_challenge;

use ray_tracer_challenge::tuples::Tuple;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Self {
        Projectile { position, velocity }
    }
}

impl Environment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Self {
        Environment { gravity, wind }
    }
}

fn tick(environment: &Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;
    Projectile { position, velocity }
}

fn main() {
    let mut projectile = Projectile::new(
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(1.0, 1.0, 0.0).normalize(),
    );
    let environment = Environment::new(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.01, 0.0, 0.0),
    );
    let mut ticks = 0;
    while projectile.position.y > 0.0 {
        projectile = tick(&environment, projectile);
        ticks += 1;
        println!("projectile.position: {:?}", projectile.position);
    }
    println!("ticks: {}", ticks);
}
