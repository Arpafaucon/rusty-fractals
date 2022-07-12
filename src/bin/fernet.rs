extern crate piston_window;

use piston::input::{Button, ButtonState};
use piston_window::*;
use std::f64::consts::PI;

type Complex = num::complex::Complex<f32>;

struct KochModel {
    level: u32,
    line: Vec<Complex>,
    max_level: u32,
}

impl KochModel {
    fn new_triangle(side: f32) -> Self {
        // Height of a equilateral triangle
        let height = side * f32::sqrt(3.0) / 2.0;
        KochModel {
            level: 0,
            max_level: 7,
            line: vec![
                Complex::new(-0.5* side, -height / 3.0),
                Complex::new(0.5*side, -height / 3.0),
                Complex::new(0.0, 2.0 / 3.0 * height),
                Complex::new(-0.5*side, -height / 3.0),
            ],
        }
    }

    fn level_up(&mut self) {
        if self.level >= self.max_level {
            return;
        }
        let j = Complex::new(0.0, 1.0);
        let mut koch_out = Vec::<Complex>::with_capacity(self.line.len() * 4);
        println!("Start iteration. #points={}", self.line.len());
        for i in 0..self.line.len() - 1 {
            let start = self.line[i];
            let end = self.line[i + 1];
            let vector = end - start;
            let base_left = start + 1.0 * vector / 3.0;
            let top = base_left + (-j * PI as f32 / 3.0).exp() * vector / 3.0;
            let base_right = start + 2.0 * vector / 3.0;
            koch_out.push(start);
            koch_out.push(base_left);
            koch_out.push(top);
            koch_out.push(base_right);
        }
        koch_out.push(*self.line.last().unwrap());
        println!("Stop iteration. #points={}", koch_out.len());
        self.line = koch_out;
        self.level += 1;
    }

    fn level_down(&mut self) {
        if self.level == 0 {
            return;
        }
        println!("level down");
        let mut koch_out = Vec::<Complex>::with_capacity(self.line.len() / 4);
        for i in (0..self.line.len() - 1).step_by(4) {
            koch_out.push(self.line[i]);
        }
        koch_out.push(*self.line.last().unwrap());
        self.line = koch_out;
        self.level -= 1;
    }
}

#[derive(Clone, Copy)]
struct KochView {
    zoom_increment: f64,
    move_increment: f64,
    line_size: f64,

    view_size: f64,
    center_x: f64,
    center_y: f64,
}

enum MoveSide {
    Up,
    Down,
    Left,
    Right,
}

impl KochView {
    fn new() -> KochView {
        KochView {
            line_size: 0.001,
            zoom_increment: 0.10,
            move_increment: 0.05,

            view_size: 2.0,
            center_x: 0.0,
            center_y: 0.0,
        }
    }

    fn draw_model(self, model: &KochModel, transform: &[[f64; 3]; 2], g: &mut G2d) {
        //! Expect a transform mapping the [-1; 1]² space to the whole window
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        // First create a transform mapping the square defined by the KochView to the [-1; 1]² space
        let view_transform = transform
            .scale(2.0 / self.view_size, 2.0 / self.view_size)
            .trans(self.center_x, self.center_y);

        for i in 0..model.line.len() - 1 {
            line(
                BLACK,
                self.line_size*self.view_size,
                line_to_view(model.line[i], model.line[i + 1]),
                view_transform,
                g,
            );
        }
    }

    fn zoom_in(&mut self) {
        self.view_size /= 1.0 + self.zoom_increment;
    }

    fn zoom_out(&mut self) {
        self.view_size *= 1.0 + self.zoom_increment;
    }

    fn move_view(&mut self, side: MoveSide) {
        let delta = self.view_size * self.move_increment;
        match side {
            MoveSide::Left => self.center_x -= delta,
            MoveSide::Right => self.center_x += delta,
            MoveSide::Up => self.center_y -= delta,
            MoveSide::Down => self.center_y += delta,
        }
    }
}

fn line_to_view(a: Complex, b: Complex) -> [f64; 4] {
    [a.re as f64, a.im as f64, b.re as f64, b.im as f64]
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("koch", [768; 2])
        .exit_on_esc(true)
        .fullscreen(true)
        .build()
        .unwrap();
    window.set_lazy(true);

    let mut koch_model = KochModel::new_triangle(1.0);
    let mut koch_view = KochView::new();

    while let Some(e) = window.next() {
        if let Some(_args) = e.render_args() {
            window.draw_2d(&e, |c, g, _| {
                clear([1.0; 4], g);

                let win_size = c.get_view_size();
                let scale: f64 = f64::min(win_size[0], win_size[1]) * 0.5;

                let transform = c
                    .transform
                    .trans(win_size[0] / 2.0, win_size[1] / 2.0)
                    .scale(scale, scale);

                koch_view.draw_model(&koch_model, &transform, g);
            });
        }
        if let Some(button_arg) = e.button_args() {
            if button_arg.state == ButtonState::Release {
                if let Button::Keyboard(k) = button_arg.button {
                    match k {
                        Key::Q => {
                            window.set_should_close(true);
                        }

                        Key::Plus | Key::NumPadPlus => koch_view.zoom_in(),
                        Key::Minus | Key::NumPadMinus => koch_view.zoom_out(),

                        Key::Left => koch_view.move_view(MoveSide::Left),
                        Key::Right => koch_view.move_view(MoveSide::Right),
                        Key::Up => koch_view.move_view(MoveSide::Up),
                        Key::Down => koch_view.move_view(MoveSide::Down),

                        Key::PageUp => koch_model.level_up(),
                        Key::PageDown | Key::Space => koch_model.level_down(),

                        _ => {}
                    }
                }
            }
        }
    }
}
