mod matrix_helpers;
use matrix_helpers::{C1, C2, C3, Degrees, RotMatrix};
use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let num_args = args.len() - 1;

    if (num_args % 2 == 0) & (num_args > 0) {
        let mut results:Vec<RotMatrix> = Vec::new();

        for i in 1..num_args {
            let arg = match &args[i].to_lowercase()[0..] {
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
            
            match arg {
                Some(mat) => results.push(mat),
                None => ()
            }
        }

        let mut res_len = results.len();
        if res_len > 0 {
            res_len = res_len - 1;
        }

        let val = match res_len >= 0 {
            true => {
                for i in 0..results.len()-1 {
                    results[i+1] = &results[i] * &results[i+1];
                }
                results[res_len].print();
                Some(())
            },

            false => { None }
        };
        
        match val {
            Some(()) => Ok(()),
            None => Err(String::from("\nInvalid Use! Pass rotations as separate, ordered arguments: C1(30.), C2(20.), etc.\n"))
        }

    }
    else {
        Err(String::from("\nInvalid Use! Pass rotations as separate, ordered arguments: C1(30.), C2(20.), etc.\n"))
    }   

}

// TODO: Route Errors to stderr instead of stdout
// TODO: Write logic in separate file ?
// TODO: Write tests
// TODO: Double check error handling