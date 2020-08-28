mod helpers;
use helpers::{C1, C2, C3, Degrees};

fn main() {
    let theta = &Degrees(35.);

    (C1(*theta) * C2(*theta) * C3(*theta)).print();

}