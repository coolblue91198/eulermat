use ndarray::prelude::*;

struct Degrees(f64);


fn float_precision(x: f64) -> f64{
    (x * 100.0).round() / 100.0
}

fn C1(theta: Degrees) -> Array2<f64> {
    let rad = theta.0.to_radians();
    let a1 = array![[1., 0., 0.],
                                                [0., float_precision(rad.cos()), float_precision(rad.sin())],
                                                [0., float_precision(-(rad.sin())), float_precision(rad.cos())]];
    
    a1
}

fn C2(theta: Degrees) -> Array2<f64> {
    let rad = theta.0.to_radians();
    let a1 = array![[float_precision(rad.cos()), 0., float_precision(-(rad.sin()))],
                                                [0., 1., 0.],
                                                [float_precision(rad.sin()), 0., float_precision(rad.cos())]];
    
    a1
}

fn C3(theta: Degrees) -> Array2<f64> {
    let rad = theta.0.to_radians();
    let a1 = array![[float_precision(rad.cos()), float_precision(rad.sin()), 0.],
                                                [float_precision(-(rad.sin())), float_precision(rad.cos()), 0.],
                                                [0., 0., 1.]];
    
    a1
}

fn main() {
    // let a = C1(Degrees(30.));
    // println!("\n{:#?}\n", a);

    // let b = C2(Degrees(30.));
    // println!("\n{:#?}\n", b);

    // let c = C3(Degrees(30.));
    // println!("\n{:#?}\n", c);

    let sol = C2(Degrees(-90.)) * C1(Degrees(90.));

    println!("{:#?}", sol);
}
