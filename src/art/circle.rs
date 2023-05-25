use std::ops::DerefMut;
use std::ptr::addr_of;

use crate::error::Error;
use svg::node::element::path::{Command, Data, Parameters};
use svg::node::element::tag::Path;
use svg::node::element::{Line, Path, Circle};
use svg::parser::Event;
use svg::Document;

use crate::art::{Drawable, Shape, BCircle};

impl BCircle {
    pub fn new(x: f32, y: f32, radius: f32, outline_color: Option<(u8, u8, u8)>) -> BCircle {
        BCircle {
            shape: Shape {
                svg: None,
                path: None,

                circ: Some(Circle::new()
                    .set("fill", "none")
                    .set("stroke", "#000000")
                    .set("stroke-width", 1)
                    .set("r", radius)
                    .set("cx", x)
                    .set("cy", y)
                    .set("transform", "rotate")
                ),
                rect: None,

                center: (x, y),
                dimensions: (0.0, 0.0),
                fill: (0, 0, 0),
                outline_color: outline_color.unwrap_or((0, 0, 0)),
                outline_width: 1.0,
                rotation: 0.0,
                point_of_rotation: (0.0, 0.0),
                rotation_about: 0.0,
                warp_vals: (0.0, 0.0),
                stretch: (1.0, 1.0),
                transform_str: "".to_string()
            },

            radius: radius,
        }
    }
}

impl Drawable for BCircle {
    fn rotate(&mut self, angle: f32) {
        self.shape.rotate(angle);
    }

    fn rotate_to(&mut self, angle: f32) {
        self.shape.rotate_to(angle);
    }

    fn rotate_about(&mut self, angle: f32, x: f32, y: f32) {
        self.shape.rotate_about(angle, x, y);
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
        if x == y {
            self.radius = x;
        }
    }

    fn reflect(&mut self, p1x: f32, p1y: f32, p2x: f32, p2y: f32) {
        self.shape.reflect(p1x, p1y, p2x, p2y);
    }

    fn warp(&mut self, freq: f32, ampl: f32) {
        self.shape.warp(freq, ampl);
    }
    fn hue_shift(&mut self, amount: f32) {
        self.shape.hue_shift(amount);
    }

    fn update(&mut self) {
        let o_color = format!("#{:02x?}{:02x?}{:02x?}", self.shape.outline_color.0, self.shape.outline_color.1, self.shape.outline_color.2);
        let rotate = format!("rotate({} {} {})", self.shape.rotation, self.shape.center.0, self.shape.center.1);
        let rotate_about = format!("rotate({} {} {})", self.shape.rotation_about, self.shape.point_of_rotation.0, self.shape.point_of_rotation.1);
        
        let all_rotate = format!("{} {}", rotate, rotate_about);
        self.shape.circ = Some(Circle::new()
                    .set("fill", "none")
                    .set("stroke", o_color)
                    .set("stroke-width", 1)
                    .set("r", self.radius)
                    .set("cx", self.shape.center.0)
                    .set("cy", self.shape.center.1)
                    .set("transform", all_rotate)
                    
                );
                    
                    
                    
                    
                    
    }

    
}