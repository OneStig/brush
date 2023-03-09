use crate::ast::{Node, NodeType};
use crate::tokens::{Token, TokenType};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    //start with program as the root node 
    //each call is a child of program (ex. let and draw calls)
    //recursive descent using the {} as markers 
    // keywords are triggers for parents 
    //{} used to recursively call itself to make another tree 
    //multiple children per parent are ok 
    //omit {} and () and use them to call 
    pub fn parse_expression(&mut self, parent:&Node){
        //here you need to add in code that gives you a mathmatical expression as a tree and assign the head of the tree as the child of parent 
        //you can use the stuff you learned from calculator for this part 
    }


    pub fn parse_brackets(&mut self, parent:&Node){
        //iterates through until end of brackets 
        //until tokens value is R curly 
        while self.tokens.first().unwrap().token_type != TokenType::R_CURLY{
            self.parse_main(parent);
        }

    }

    pub fn parse_paren(&mut self, parent:&Node){
        //goes until end of paren
        while self.tokens.first().unwrap().token_type != TokenType::R_PAREN{
            self.parse_main(parent);
        }
    }
    
    pub fn parse_main(&mut self, parent:&Node) {
        match self.tokens.first().unwrap().token_type{
            //endline -> throwaway 
            TokenType::ENDLINE => {
                self.tokens.remove(0);
            }
            //evolve keyword -> make child of parent
            TokenType::EVOLVE_KEYWORD => {
                let p = Node::new(NodeType::ShapeDeclaration, self.tokens.first().unwrap().value);
                self.tokens.remove(0);
                parent.children.push(p);
            } 
            //L curly -> to parse brackets with parent as last child 
            TokenType::L_CURLY => {
                self.tokens.remove(0);
                if(parent.children.last().unwrap().children.len()!=0){
                    self.parse_brackets(parent.children.last().unwrap().children.last().unwrap());
                }
                else{
                self.parse_brackets(parent.children.last().unwrap());
                }
            }
            //L paren 
            TokenType::L_PAREN => {
                self.tokens.remove(0);
                if(parent.children.last().unwrap().children.len()!=0){
                    self.parse_paren(parent.children.last().unwrap().children.last().unwrap());
                }
                else{
                self.parse_paren(parent.children.last().unwrap());
                }
            }
            //property -> make child of parent and parse expression 
            TokenType::PROPERTIES => {
                let p = Node::new(NodeType::ShapeDeclaration, self.tokens.first().unwrap().value);
                self.tokens.remove(0);
                parent.children.push(p);
                if(self.tokens.len()>0&&self.tokens.first().unwrap().value=="="){
                self.tokens.remove(0);
                self.parse_expression(&p);
                }
            }
        }
    }


    pub fn parse(&mut self) -> Node {

        let mut root = Node::new(NodeType::Program, String::from("Program"));

        while self.tokens.len()>0{
            if(self.tokens.first().unwrap().token_type == TokenType::L_CURLY){
                //start new tree from last added child 
                self.tokens.remove(0);
                if(root.children.last().unwrap().children.len()!=0){
                    self.parse_brackets(root.children.last().unwrap().children.last().unwrap());
                }
                else{
                self.parse_brackets(root.children.last().unwrap());
                }
                
            }
            else if(self.tokens.first().unwrap().token_type == TokenType::L_PAREN){
                self.tokens.remove(0);
                if(root.children.last().unwrap().children.len()!=0){
                    self.parse_paren(root.children.last().unwrap().children.last().unwrap());
                }
                else{
                self.parse_paren(root.children.last().unwrap());
                }
            }
            else if(self.tokens.first().unwrap().token_type == TokenType::KEYWORD){
                let mut p = Node::new(NodeType::ShapeDeclaration, self.tokens.first().unwrap().value);
                self.tokens.remove(0);
                root.children.push(p);
                if(self.tokens.first().unwrap().value=="="){
                    self.tokens.remove(0);
                }
                if(self.tokens.first().unwrap().token_type==TokenType::SHAPE_KEYWORD){
                    let mut x = Node::new(NodeType::ShapeDeclaration, self.tokens.first().unwrap().value);
                    p.children.push(x);
                }
                //make parent and recurse down 
            }
            else if (self.tokens.first().unwrap().token_type==TokenType::ENDLINE){
                //throwaway
                self.tokens.remove(0);
            }
            else{
                let mut p = Node::new(NodeType::ShapeDeclaration,self.tokens.first().unwrap().value);
                self.tokens.remove(0);
                root.children.push(p);
                //everything else is made a child 
            }
        }    
        // all parser code 
        root
    }

    // additional functions might be 
}