use macroquad::prelude::*;
use macroquad::rand;


struct Particle {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    radius: f32,
    colour: Color,
}

const GRAVITY: f32 = 10000.0;
const MIN_GRAV_DIST: f32 = 100.0;
const DRAG: f32 = 0.1;
const MIN_SPEED_BOUNDARY: f32 = 20.0;

impl Particle {
    pub fn new(pos: Vec2, vel: Vec2, c: Color) -> Self {
        Self {
            pos: pos,
            vel: vel,
            acc: vec2(0.0, 0.0),
            radius: 3.0,
            colour: c
        }
    }

    pub fn update(&mut self, dt: f32) {
        if ((self.pos.x + self.radius >= screen_width() && self.vel.x > 0.0) || (self.pos.x - self.radius <= 0.0 && self.vel.x < 0.0)) {
            self.vel.x *= -1.0;
        }
        if ((self.pos.y + self.radius >= screen_height() && self.vel.y > 0.0) || (self.pos.y - self.radius <= 0.0 && self.vel.y < 0.0)) {
            self.vel.y *= -1.0;
        }

        //Boundary conditions fo rslowly being pushed off screen, amounts to: if you're outside the
        //boundary, and you're going slow enough, go to the obundary and stop moving
        if (self.pos.x + self.radius >= screen_width() && self.vel.length_squared() < MIN_SPEED_BOUNDARY) {
            self.pos.x = screen_width() - self.radius;
            self.vel.x = 0.0;
        }
        if (self.pos.x - self.radius <= 0.0 && self.vel.length_squared() < MIN_SPEED_BOUNDARY) {
            self.pos.x = 0.0 + self.radius;
            self.vel.x = 0.0;
        }
        if (self.pos.y + self.radius >= screen_height() && self.vel.length_squared() < MIN_SPEED_BOUNDARY) {
            self.pos.y = screen_height() - self.radius;
            self.vel.y = 0.0;
        }
        if (self.pos.y - self.radius <= 0.0 && self.vel.length_squared() < MIN_SPEED_BOUNDARY) {
            self.pos.y = 0.0 + self.radius;
            self.vel.y = 0.0;
        }
        
        self.vel.x += self.acc.x * dt;
        self.vel.y += self.acc.y * dt;
        self.pos.x += self.vel.x * dt;
        self.pos.y += self.vel.y * dt;
        //reset for forces to work properly
        self.acc = vec2(0.0, 0.0);
    }

    // drag, external gravity, any forces not dependant on other particles, this doesnt work with
    // another function, how do I do this properly without resetting it?
    pub fn apply_individual_forces(&mut self) {
        self.acc = vec2(0.0, 0.0);
        self.apply_drag_force();
    }

    pub fn apply_drag_force(&mut self) {
        self.acc -= DRAG * self.vel;
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.colour);
    }
}

// interactions between particles
fn interact(p1: &mut Particle, p2: &mut Particle) {
    if (p1.colour == YELLOW && p2.colour == YELLOW) {
        apply_attraction_interaction(p1, p2, 0.5);
    }
    if (p1.colour == YELLOW && p2.colour == RED) {
        apply_attraction_interaction(p1, p2, -0.85);
    }
    if (p1.colour == YELLOW && p2.colour == BLUE) {
        apply_attraction_interaction(p1, p2, 1.0);
    }
    if (p1.colour == YELLOW && p2.colour == GREEN) {
        apply_attraction_interaction(p1, p2, -0.33);
    }

    if (p1.colour == RED && p2.colour == RED) {
        apply_attraction_interaction(p1, p2, -0.22);
    }
    if (p1.colour == RED && p2.colour == BLUE) {
        apply_attraction_interaction(p1, p2, 0.1);
    }
    if (p1.colour == RED && p2.colour == GREEN) {
        apply_attraction_interaction(p1, p2, -1.0);
    }

    if (p1.colour == BLUE && p2.colour == BLUE) {
        apply_attraction_interaction(p1, p2, 0.69);
    }
    if (p1.colour == BLUE && p2.colour == GREEN) {
        apply_attraction_interaction(p1, p2, 0.56);
    }
    
    if (p1.colour == GREEN && p2.colour == GREEN) {
        apply_attraction_interaction(p1, p2, -0.2);
    }

    p1.apply_drag_force();
}

fn apply_attraction_interaction(p1: &mut Particle, p2: &mut Particle, force: f32) {
    let distance = vec2(p2.pos.x - p1.pos.x, p2.pos.y - p1.pos.y);
    let norm_distance = distance.clone().normalize();
    if (distance.length_squared() < MIN_GRAV_DIST) {
        p1.acc += GRAVITY / MIN_GRAV_DIST * (1.0) * force * norm_distance;
        p2.acc += GRAVITY / MIN_GRAV_DIST * (-1.0) * force * norm_distance;
        return;
    }
    p1.acc += GRAVITY / distance.length_squared() * (1.0) * force * norm_distance;
    p2.acc += GRAVITY / distance.length_squared() * (-1.0) * force * norm_distance;

}


fn create_particles(particles: &mut Vec::<Particle>, num_particles: i16, colour: Color) {
    for i in 0..num_particles {
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
        let p = Particle::new(vec2(x, y), vec2(0.0, 0.0), colour);
        particles.push(p);
    }
}




#[macroquad::main("rust-gui")]
async fn main() {
    //let mut p1 = Particle::new(vec2(screen_width() / 4.0, screen_height() / 4.0), vec2(1000.0, 0.0), YELLOW);
    //let mut p2 = Particle::new(vec2(3.0 * screen_width() / 4.0, 3.0 * screen_height() / 4.0), vec2(-100.0, 0.0), RED);
    //let mut p3 = Particle::new(vec2(screen_width() / 2.0, 1.0 * screen_height() / 4.0), vec2(0.0, -10.0), BLUE);
    //let mut p4 = Particle::new(vec2(3.0 * screen_width() / 4.0, 1.0 * screen_height() / 4.0), vec2(0.0, -10.0), GREEN);
    let mut particles = Vec::new();
    create_particles(&mut particles, 75, YELLOW);
    create_particles(&mut particles, 75, RED);
    create_particles(&mut particles, 75, BLUE);
    create_particles(&mut particles, 75, GREEN);
    //particles.push(p1);
    //particles.push(p2);
    //particles.push(p3);
    //particles.push(p4);

    loop {
        clear_background(BLACK);

        for i in 0..particles.len() {
            particles[i].draw();
        }
        //p1.draw();
        //p2.draw();
        //p3.draw();

        for i in 0..particles.len() {
            particles[i].update(get_frame_time());
        }

        for i in 0..(particles.len() - 1) {
            for j in (i + 1)..particles.len() {
                let (head, tail) = particles.split_at_mut(i + 1);
                //if (particles[i].colour == YELLOW) {
                //    print!("YELLOW, ");
                //}
                //if (particles[i].colour == RED) {
                //    print!("RED, ");
                //}
                //if (particles[i].colour == BLUE) {
                //    print!("BLUE, ");
                //}
                //if (particles[j].colour == YELLOW) {
                //    print!("YELLOW");
                //}
                //if (particles[j].colour == RED) {
                //    print!("RED");
                //}
                //if (particles[j].colour == BLUE) {
                //    print!("BLUE");
                //}
                interact(&mut head[i],  &mut tail[j - i - 1]);
            }
        }
        //p1.update(get_frame_time());
        //p2.update(get_frame_time());
        //p3.update(get_frame_time());
        //interact(&mut p1, &mut p2);
        //interact(&mut p1, &mut p3);
        //interact(&mut p2, &mut p3);
        //println!("p1: ({}, {})", p1.acc.x, p2.acc.y);
        //println!("p2: ({}, {})", p2.acc.x, p2.acc.y);


        next_frame().await
    }
}
