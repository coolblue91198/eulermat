mod matrix_helpers;
use matrix_helpers::{C1, C2, C3, Degrees, RotMatrix};
use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let val: Option<RotMatrix> = match &args[1][0..] {
        "c1" => {
            let angle = args[2].parse::<f64>().unwrap_or(0.);

            Some(C1(Degrees(angle)))
        },

        &_ => None 
    };

    match val {
        None => Err(String::from("Please enter Ci(angle) -  Where i is 1, 2, or 3 and angle is a floating point.")),

        Some(mat) => {
            mat.print();
            Ok(())
        }
    }

    // Ok(())

    // let theta = &Degrees(35.);

    // (C1(*theta) * C2(*t"heta) * C3(*theta)).print();

}