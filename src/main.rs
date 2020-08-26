use ndarray::prelude::*;

struct Degrees(f64);

fn C1(theta: Degrees) -> Array2<f64> {
    let rad = theta.0.to_radians();
    let a1 = array![[1., 0., 0.],
                                                [0., rad.cos(), rad.sin()],
                                                [0., -(rad.sin()), rad.cos()]];
    
    a1
}

fn C2(theta: Degrees) -> Array2<f64> {
    let rad = theta.0.to_radians();
    let a1 = array![[rad.cos(), 0., -(rad.sin())],
                                                [0., 1., 0.],
                                                [rad.sin(), 0., rad.cos()]];
    
    a1
}

fn C3(theta: Degrees) -> Array2<f64> {
    let rad = theta.0.to_radians();
    let a1 = array![[rad.cos(), rad.sin(), 0.],
                                                [-(rad.sin()), rad.cos(), 0.],
                                                [0., 0., 1.]];
    
    a1
}

fn main() {
    let a = C1(Degrees(30.));
    println!("\n{:#?}\n", a);

    let b = C2(Degrees(30.));
    println!("\n{:#?}\n", b);

    let c = C3(Degrees(30.));
    println!("\n{:#?}\n", c);
}
