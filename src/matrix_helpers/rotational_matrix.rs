use std::ops::Mul;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RotMatrix {
    arr: Vec<Vec<f64>>
}

impl RotMatrix {
    pub fn new(row1: Vec<f64>, row2: Vec<f64>, row3: Vec<f64>) -> Self {
        RotMatrix {
            arr: vec![row1, row2, row3]
        }
    }

    pub fn print(&self) {
        println!("\n");
        println!("{:.3}  {:.3}  {:.3}", self.arr[0][0], self.arr[0][1], self.arr[0][2]);
        println!("{:.3}  {:.3}  {:.3}", self.arr[1][0], self.arr[1][1], self.arr[1][2]);
        println!("{:.3}  {:.3}  {:.3}", self.arr[2][0], self.arr[2][1], self.arr[2][2]);
        println!("\n");
    }
}

impl Mul for RotMatrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = RotMatrix::new(
            vec![-999.;3],
            vec![-999.; 3],
            vec![-999.; 3]
        );

        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.;
                for k in 0..3 {
                    sum += self.arr[i][k] * rhs.arr[k][j];
                }
                res.arr[i][j] = sum;
            }
        }
        res
    }
}

impl Mul for &RotMatrix {
    type Output = RotMatrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = RotMatrix::new(
            vec![-999.;3],
            vec![-999.; 3],
            vec![-999.; 3]
        );

        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.;
                for k in 0..3 {
                    sum += self.arr[i][k] * rhs.arr[k][j];
                }
                res.arr[i][j] = sum;
            }
        }
        res
    }
}
