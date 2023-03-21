use std::ops::DerefMut;
use std::ptr::addr_of;

use crate::error::Error;
use svg::node::element::path::{Command, Data, Parameters};
use svg::node::element::tag::Path;
use svg::node::element::{Line, Path};
use svg::parser::Event;
use svg::Document;

pub struct Shape {
    pub svg: Path,
    path: Data,
    center: (f32, f32),
    dimensions: (f32, f32),
    fill: (u8, u8, u8),
    outline_color: (u8, u8, u8),
    outline_width: f32,
    rotation: f32,
    stretch: (f32, f32),
}

trait Drawable {
    fn rotate(&mut self, angle: f32);
    fn rotate_to(&mut self, angle: f32);
    fn shift(&mut self, x: f32, y: f32);
    fn shift_to(&mut self, x: f32, y: f32);
    fn stretch(&mut self, x: f32, y: f32);
    fn stretch_to(&mut self, x: f32, y: f32);
    fn update(&mut self);
}

pub struct Circle {
    shape: Shape,
    radius: f32,
}

pub struct Rectangle {
    shape: Shape,
    width: f32,
    height: f32,
}

pub struct Polygon {
    shape: Shape,
    points: Vec<(f32, f32)>,
}

pub struct SVG {
    shape: Shape,
    dimensions: (f32, f32),
}

impl Circle {
    pub fn new() -> Circle {
        Circle {
            shape: Shape {
                svg: Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 1),
                path: Data::new(),
                center: (0.0, 0.0),
                dimensions: (0.0, 0.0),
                fill: (0, 0, 0),
                outline_color: (0, 0, 0),
                outline_width: 1.0,
                rotation: 0.0,
                stretch: (1.0, 1.0),
            },
            radius: 0.0,
        }
    }
}

impl Drawable for Shape {
    fn rotate(&mut self, angle: f32) {
        unimplemented!();
    }

    fn rotate_to(&mut self, angle: f32) {
        unimplemented!();
    }

    fn shift(&mut self, x: f32, y: f32) {
        self.center.0 += x;
        self.center.1 += y;

        // iterate through the path and shift each point
        let mut cdata = self.path.clone();
        let mut newData = Data::new();

        // bruh we have to handle each type of command
        for cmd in cdata.iter() {
            // derefererence error here
            match cmd {
                Command::Move(_pos, para) => {
                    newData = newData.move_to((para.get(0).unwrap() + x, para.get(1).unwrap() + y));
                }

                Command::Line(_pos, para) => {
                    newData = newData.move_to((para.get(0).unwrap() + x, para.get(1).unwrap() + y));
                }

                Command::HorizontalLine(pos, para) => {
                    unimplemented!();
                }

                Command::VerticalLine(pos, para) => {
                    unimplemented!();
                }

                Command::QuadraticCurve(pos, para) => {
                    unimplemented!();
                }

                Command::SmoothQuadraticCurve(pos, para) => {
                    unimplemented!();
                }

                Command::SmoothCubicCurve(pos, para) => {
                    unimplemented!();
                }

                Command::EllipticalArc(pos, para) => {
                    unimplemented!();
                }

                Command::CubicCurve(pos, para) => {
                    unimplemented!();
                }

                Command::Close => {}
            }
        }
    }

    fn shift_to(&mut self, x: f32, y: f32) {
        unimplemented!();
    }

    fn stretch(&mut self, x: f32, y: f32) {
        unimplemented!();
    }

    fn stretch_to(&mut self, x: f32, y: f32) {
        unimplemented!();
    }

    fn update(&mut self) {
        unimplemented!();
    }
}

impl Drawable for Circle {
    fn rotate(&mut self, angle: f32) {
        unimplemented!();
    }

    fn rotate_to(&mut self, angle: f32) {
        unimplemented!();
    }

    fn shift(&mut self, x: f32, y: f32) {
        self.shape.center = (self.shape.center.0 + x, self.shape.center.1 + y);
    }

    fn shift_to(&mut self, x: f32, y: f32) {
        self.shape.center = (x, y);
    }

    fn stretch(&mut self, x: f32, y: f32) {
        if x == y {
            self.radius *= x;
        }
    }

    fn stretch_to(&mut self, x: f32, y: f32) {
        unimplemented!();
    }

    fn update(&mut self) {
        unimplemented!();
    }
}

pub fn draw(shapes: Vec<Shape>) -> Result<(), Error> {
    let mut canvas: Document = Document::new()
        .set("viewBox", (0, 0, 1000, 1000))
        .set("width", "100%")
        .set("height", "100%")
        .set("preserveAspectRatio", "xMidYMid meet");

    for shape in shapes {
        canvas = canvas.add(shape.svg);
    }

    svg::save("art.svg", &canvas).unwrap();

    Ok(())
}
