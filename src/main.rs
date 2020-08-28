mod helpers;
use helpers::{C1, C2, C3, Degrees};

fn main() {
    let a1 = C1(Degrees(90.));

    let a2 = C2(Degrees(-90.));

    a1.print(); a2.print();

    (a1 * a2).print();

}