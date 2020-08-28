use super::rotational_matrix::RotMatrix;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Degrees(pub f64);

fn float_precision(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

pub fn C1(theta: Degrees) -> RotMatrix {
    let rad = theta.0.to_radians();
    RotMatrix::new(
        vec![1.,           0.,            0.],
        vec![0.,    rad.cos(),     rad.sin()],
        vec![0., -(rad.sin()),     rad.cos()],      
    )
}

pub fn C2(theta: Degrees) -> RotMatrix {
    let rad = theta.0.to_radians();
    RotMatrix::new(
        vec![rad.cos(),   0., -(rad.sin())],
        vec![0.       ,   1.,           0.],
        vec![rad.sin(),   0.,    rad.cos()],        
    )
}

pub fn C3(theta: Degrees) -> RotMatrix {
    let rad = theta.0.to_radians();
    RotMatrix::new(
        vec![  rad.cos() ,    rad.sin(),    0.],
        vec![-(rad.sin()),    rad.cos(),    0.],
        vec![  0.        ,           0.,    1.],        
    )
}
