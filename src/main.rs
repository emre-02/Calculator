#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


// *We defined an enum called calc
enum Calc {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// * 
fn calculate(operator: Calc, operand1: f64, operand2: f64) -> Option<f64> {
    match operator {
       Calc::Add => Some(operand1 + operand2),
       Calc::Subtract => Some(operand1 - operand2),
        Calc::Multiply => Some(operand1 * operand2),
        Calc::Divide => {
            if operand2 != 0.0 {
                Some(operand1 / operand2)
            } else {
                None
            }
        }
    }
}

fn main() {
    let operator = Calc::Multiply;
    let operand1 = 9.0;
    let operand2 = 5.0;

    if let Some(result) = calculate(operator, operand1, operand2) {
        println!("Result: {}", result);
    } else {
        println!("Invalid operation: division cannot be divided by zero!");
    }
}
