mod matrix_helpers;
use matrix_helpers::{C1, C2, C3, Degrees, RotMatrix};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_args = args.len() - 1;

    if num_args % 2 == 0 {
        let mut results:Vec<RotMatrix> = Vec::new();

        for i in 1..num_args {
            let val = match &args[i].to_lowercase()[0..] {
                "c1" => {
                    let angle = args[i+1].parse::<f64>().unwrap_or(0.);
                    Some(C1(Degrees(angle)))
                },

                "c2" => {
                    let angle = args[i+1].parse::<f64>().unwrap_or(0.);
                    Some(C2(Degrees(angle)))
                }

                "c3" => {
                    let angle = args[i+1].parse::<f64>().unwrap_or(0.);
                    Some(C3(Degrees(angle)))                    
                }

                &_ => None
            };
            
            match val {
                Some(mat) => results.push(mat),
                None => ()
            }
        }

        let res_len = results.len() - 1;

        match res_len > 0 {
            true => {
                for i in 0..results.len()-1 {
                    results[i+1] = &results[i] * &results[i+1];
                }
                println!("{:}", res_len);
                results[res_len].print();                      
            },

            false => {println!("ITS FALSE!!")}
        }
    }   

}