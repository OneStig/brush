use crate::ast::{Node, NodeType};
use crate::tokens::{Token, TokenType};

pub struct Interpreter {
    pub ast: Node,
}

impl Interpreter {
    pub fn new(ast: Node) -> Interpreter {
        Interpreter {
            ast: ast,
        }
    }

    pub fn run(&mut self) {
        println!("Interpreting AST");
    }

}


// defining functions for a Shape
pub trait Shape {
    fn set_radius(&mut self, radius : f64);
    fn evolve(&mut self);
    fn draw(&self, center: (f64, f64), generations: i32);
}

// defining Circle structure
struct Circle {
    radius: f64,
}
// defining keyword function specifically for circle
impl Circle {
    fn new(radius: f64) -> Circle {
        Circle {
            radius: radius,
        }
    }
}

// implementing circle as a shape that changes the radius based on functions accordingly 
impl Shape for Circle {
    fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    fn evolve(&mut self) {
        self.radius /= 2.0;
    }

    fn draw(&self, center: (f64, f64), generations: i32) {
        println!("Drawing circle with radius {} at center {:?} for {} generations", self.radius, center, generations);
    }
}

// initializing a new circle into Shape with radius 
/* impl dyn Shape {
    pub fn new_circle(radius: f64) -> impl Shape {
        Circle::new(radius)
    }
} */
