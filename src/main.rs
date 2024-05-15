use std::f32::consts::FRAC_PI_2;

use macroquad::prelude as mq;
use nalgebra::{Matrix2, Point2, Rotation2, UnitVector2, Vector2};

#[derive(Debug)]
struct Parameters {
    gravity: Vector2<f32>,
    restitution_lp: f32,
    restitution_pp: f32,
    time_step: f32,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            gravity: Vector2::new(0.0, 1.0),
            restitution_lp: 0.9,
            restitution_pp: 0.9,
            time_step: 1.0,
        }
    }
}

#[derive(Debug)]
struct Particle {
    mass: f32,
    radius: f32,
    position: Point2<f32>,
    velocity: Vector2<f32>,
}

#[derive(Debug, Clone)]
struct Line {
    start: Point2<f32>,
    end: Point2<f32>,
}

#[derive(Debug)]
enum Entity {
    Particle(Particle),
    Line(Line),
}

impl Entity {
    fn update(&mut self, params: &Parameters) {
        match self {
            Entity::Particle(Particle {
                position, velocity, ..
            }) => {
                let acceleration = params.gravity;
                *position += *velocity * params.time_step
                    + 0.5 * acceleration * params.time_step * params.time_step;
                *velocity += acceleration * params.time_step;
            }
            Entity::Line(_) => {}
        }
    }

    fn draw(&self) {
        match self {
            Entity::Particle(Particle {
                radius, position, ..
            }) => mq::draw_circle(position.x, position.y, *radius, mq::WHITE),
            Entity::Line(Line { start, end, .. }) => {
                mq::draw_line(start.x, start.y, end.x, end.y, 5.0, mq::WHITE)
            }
        }
    }

    fn collides_with(&self, other: &Entity) -> bool {
        match (self, other) {
            (
                Entity::Particle(Particle {
                    position, radius, ..
                }),
                Entity::Line(Line { start, end, .. }),
            ) => {
                let unit_normal =
                    Rotation2::new(FRAC_PI_2) * UnitVector2::new_normalize(end - start);
                let distance = (position - *start).dot(&unit_normal);
                distance.abs() < *radius
            }
            (Entity::Line(_), Entity::Particle(_)) => other.collides_with(self),
            (Entity::Particle(this), Entity::Particle(other)) => {
                (this.position - other.position).norm() < (this.radius + other.radius)
            }
            _ => false,
        }
    }
}

fn collide_line_particle(l: &Line, p: &Particle, params: &Parameters) -> Vector2<f32> {
    let line = UnitVector2::new_normalize(l.end - l.start);
    let normal = Rotation2::new(FRAC_PI_2) * line;

    /*
        v.b = u.b
        v.n = -e * u.n
    */

    let w = Matrix2::new(line.x, line.y, normal.x, normal.y)
        .try_inverse()
        .unwrap()
        * Vector2::new(
            p.velocity.dot(&line),
            -params.restitution_lp * p.velocity.dot(&normal),
        );

    w
}

fn collide_particles(
    p: &Particle,
    q: &Particle,
    params: &Parameters,
) -> (Vector2<f32>, Vector2<f32>) {
    let normal = UnitVector2::new_normalize(p.position - q.position);
    let line = Rotation2::new(FRAC_PI_2) * normal;
    todo!()
    //
}

#[macroquad::main("Physics")]
async fn main() {
    // println!("Hello, world!");
    let params = Parameters::default();

    let mut entities = vec![
        // Entity::Particle(Particle {
        //     mass: 1.0,
        //     radius: 10.0,
        //     position: Point2::new(100.0, 300.0),
        //     velocity: Vector2::new(1.0, 0.0),
        // }),
        Entity::Line(Line {
            start: Point2::new(0.0, 200.0),
            end: Point2::new(800.0, 200.0),
            // unit_normal: Vector2::new(0.0, 1.0),
        }),
    ];

    let mut do_time = true;
    let mut is_drawing_line = false;
    let mut line_start = None;

    loop {
        mq::clear_background(mq::BLACK);
        let (width, height) = (mq::screen_width(), mq::screen_height());

        if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
            let pos = mq::mouse_position();
            entities.push(Entity::Particle(Particle {
                mass: 1.0,
                radius: 10.0,
                position: Point2::new(pos.0, pos.1),
                velocity: Vector2::new(0.0, 0.0),
            }));
        }

        if mq::is_key_pressed(mq::KeyCode::Space) {
            do_time = !do_time;
            continue;
        }

        if mq::is_mouse_button_pressed(mq::MouseButton::Right) && !is_drawing_line {
            is_drawing_line = true;
            do_time = false;
            line_start = Some(mq::mouse_position());
            // println!("line started at {:?}", line_start);
            // we need to jump to the next frame to avoid the line being finished immediately
            mq::next_frame().await;
        }

        if is_drawing_line {
            let pos = mq::mouse_position();
            let l = Line {
                start: Point2::new(line_start.unwrap().0, line_start.unwrap().1),
                end: Point2::new(pos.0, pos.1),
            };
            Entity::Line(l.clone()).draw();
            // println!("line drawing to {:?}", pos);
        }

        if mq::is_mouse_button_pressed(mq::MouseButton::Right) && is_drawing_line {
            let pos = mq::mouse_position();
            entities.push(Entity::Line(Line {
                start: Point2::new(line_start.unwrap().0, line_start.unwrap().1),
                end: Point2::new(pos.0, pos.1),
            }));
            is_drawing_line = false;
            do_time = true;
            line_start = None;
            println!("line ended at {:?}", pos);
        }

        if mq::is_key_pressed(mq::KeyCode::Escape) && is_drawing_line {
            is_drawing_line = false;
            do_time = true;
            line_start = None;
            println!("line cancelled");
        }

        for i in 0..entities.len() {
            // match &entities[i] {
            //     Entity::Particle(p) => {
            //         if p.position.x < -10.0 - p.radius
            //             || p.position.x > width + 10.0 + p.radius
            //             || p.position.y < -10.0 - p.radius
            //             || p.position.y > height + 10.0 + p.radius
            //         {
            //             entities.remove(i);
            //             continue;
            //         }
            //     }
            //     _ => {}
            // }
            if do_time {
                entities[i].update(&params);
            }
            entities[i].draw();
            if do_time {
                for j in (i + 1)..entities.len() {
                    if entities[i].collides_with(&entities[j]) {
                        // println!("Collision!");
                        // println!()
                        match (&entities[i], &entities[j]) {
                            (Entity::Particle(p), Entity::Particle(q)) => {
                                let (vp, vq) = collide_particles(p, q, &params);
                            }
                            (Entity::Particle(p), Entity::Line(l)) => {
                                let w = collide_line_particle(l, p, &params);
                                entities[i] = Entity::Particle(Particle { velocity: w, ..*p });
                            }
                            (Entity::Line(l), Entity::Particle(p)) => {
                                let w = collide_line_particle(l, p, &params);
                                entities[j] = Entity::Particle(Particle { velocity: w, ..*p });
                            }
                            (Entity::Line(_), Entity::Line(_)) => {}
                        }
                    }
                }
            }
        }

        // println!("{:?}", entities);

        // macroquad::ui::root_ui().label(None, "hello megaui");
        // if macroquad::ui::root_ui().button(None, "Push me") {
        //     println!("pushed");
        // }

        mq::next_frame().await
    }
}
