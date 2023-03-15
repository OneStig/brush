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

    pub fn run(&self) {
        println!("Interpreting AST");

        let mut shape = <dyn Shape>::new_circle(100.0);

        //initializing the root node
        let mut node = &self.ast;

        // iterate through the AST until we are done with the shape characteristics
        while let NodeType::ShapeDeclaration = node.node_type {
            // specifics for circle shape
            let circle = String::from("circle");
            if let NodeType::Shape(circle) = node.children[0].node_type {
                // changes the radius based on AST value
                let radius = self.evaluate(&node.children[0].children[0]);
                shape.set_radius(radius);
            }

            // generic 'evolve' mechanic condition
            if let Some(evolve_node) = node.children.iter().find(|n| n.node_type == NodeType::ShapeIdentifier) {
                let generations: i32 = self.evaluate(&evolve_node.children[0]) as i32;
                // evolve based on number of generations shown in the AST
                for num in 0..generations {
                    shape.evolve();
                }
            }
            
            // move to the next right node
            node = &node.children[1];
        }
    }

    // evaluates the node based on its node type
    fn evaluate(&self, node: &Node) -> f64 {
        // determines current token type
        match node.value {
            // saves token type 'number' into a value
            TokenType::NUMBER => value,
            // if its an operator, get the values of its children and perform indicated operation
            TokenType::OPERATOR => {
                let left_value = self.evaluate(&node.children[0]);
                let right_value = self.evaluate(&node.children[1]);

            // current operations
                match op {
                    '+' => left_value + right_value,
                    '-' => left_value - right_value,
                    '/' => left_value / right_value,
                    '*' => left_value * right_value,

                }
            }
            
        }
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

// defining Triangle structure
struct Triangle {
    s1: f64,
    s2: f64,
    s3: f64,
}
// defining Rectangle structure
struct Rectangle {
    s1: f64,
    s2: f64,
    s3: f64,
    s4: f64,
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
impl dyn Shape {
    pub fn new_circle(radius: f64) -> impl Shape {
        Circle::new(radius)
    }
} 

