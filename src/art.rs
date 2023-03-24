use svg::Document;
use svg::parser::Event;
use svg::node::element::Path;
use svg::node::element::tag::Path;
use svg::node::element::path::{Command, Data};
use crate::error::Error;

pub struct Shape {
    pub svg: Path,
    path: Data,
    center: (f64, f64),
    dimensions: (f64, f64),
    fill: (u8, u8, u8),
    outline_color: (u8, u8, u8),
    outline_width: f64,
    rotation: f32,
    stretch: (f64, f64),
    warp: f64,
}


pub trait Drawable{
    fn rotate(&mut self, angle: f32);
    fn new_rect() -> Shape;
    fn new_triangle() -> Shape;
    fn new_polygon() -> Shape;
    fn shift(&mut self, x: f64, y: f64);
}

impl Shape {
    pub fn new(pos: (f32, f32), width: f32, height: f32) -> Shape {
        let mut rdata = Data::new();
        let x = pos.0;
        let y = pos.1;
        rdata = rdata.move_to((x, y));
        rdata = rdata.line_to((x + width, y));
        rdata = rdata.line_to((x + width, y - height));
        rdata = rdata.line_to((x, y - height));
        rdata = rdata.line_to((x, y));

        rdata = rdata.close();

        Rectangle {
            shape: Shape {
                svg: Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 1)
                    .set("d", rdata.clone()),
                path: rdata,
                dimensions: (0.0, 0.0),
                fill: (0, 0, 0),
                outline_color: (0, 0, 0),
                outline_width: 1.0,
                stretch: (1.0, 1.0),
                warp: 0.0,
                center: ((pos.0 + (width/2.0)).into(), (pos.1-(height/2.0)).into()),
                rotation: 0.0,
            },
            width: width,
            height: height,
        }
    }
    pub fn new_circle(
        center: (f64, f64),
        radius: f64,
    ) -> Shape {  
        let mut cdata = Data::new();
        let mut x = center.0 + radius;
        let mut y = center.1;
        let mut angle = 0.0;

        while angle < 360.0 {
            // path.move_to defines the starting point of the path
            // path.line_to defines 
            // path.move_to((x, y));
            // let delta_x = radius * angle.cos();
            // let delta_y = radius * angle.sin();
            // path.line_to((center.0 + delta_x, center.1 + delta_y));
            // x = center.0 + delta_x;
            // y = center.1 + delta_y;
            angle += 1.0;
        }

        let svg = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("d", cdata);
        
        // Shape {
        //     svg: svg,
        //     path: path,
        //     center: center,
        //     dimensions: (radius * 2.0, radius * 2.0),
        // }
        unimplemented!();
        cdata = cdata.close();

        Circle {
            shape: Shape {
                svg: Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 1)
                    .set("d", cdata.clone()),
                path: cdata,
                center: (x, y),
                dimensions: (0.0, 0.0),
                fill: (0, 0, 0),
                outline_color: (0, 0, 0),
                outline_width: 1.0,
                rotation: 0.0,
                stretch: (1.0, 1.0),
                warp: 0.0,
            },
            radius: radius,
        }
    }
}

//impl Rectangle{}


impl Drawable for Shape {
    fn rotate(&mut self, angle: f32) {
        self.rotation += angle;

        // iterate through the path and rotate each point
        let mut cdata = self.path.clone();
        let mut newData = Data::new();

        // bruh we have to handle each type of command
        for cmd in cdata.iter() {
            // derefererence error here
            match cmd {
                Command::Move(_pos, para) => {
                    newData = newData.move_to((para.get(0).unwrap() + angle));
                }

                Command::Line(_pos, para) => {
                    newData = newData.line_to((para.get(0).unwrap() + angle));
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

        self.path = newData.close();

        self.svg = Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 1)
                    .set("d", self.path.clone());
    }

    fn new_rect() -> Shape {
        unimplemented!();
    }

    fn new_triangle() -> Shape {
        unimplemented!();
    }

    fn new_polygon() -> Shape {
        unimplemented!();
    }

    fn shift(&mut self, x: f64, y: f64) {
        self.center.0 += x;
        self.center.1 += y;

        // iterate through the path and shift each point   
    }
}

pub fn draw(shapes: Vec::<Shape>) -> Result<(), Error> { 
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
