use nalgebra::Vector3;


type Vector = Vector3<f32>;

struct Env {
    gravity: Vector,
    wind: Vector,
}

struct Projectile {
    position: Vector,
    velocity: Vector,
}

fn tick(projectile: Projectile, env: &Env) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + env.gravity + env.wind,
    }
}

fn main() {
    let mut p = Projectile {
        position: Vector::new(0., 1., 0.),
        velocity: Vector::new(1., 1., 0.).normalize(),
    };
    let e = Env {
        gravity: Vector::new(0., -0.1, 0.),
        wind: Vector::new(-0.01, 0., 0.),
    };

    while p.position.y >= 0. {
        println!("{}, {}, {}", p.position.x, p.position.y, p.position.z);
        p = tick(p, &e);
    }
}
