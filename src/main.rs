mod newton;
mod vmi;

use ndarray::Array1;
use ndarray::{arr1, arr2};

fn main() {
    let f = |x: &Array1<f32>| arr1(&[2.0 * x[0] + x[1] - 5.0, 4.0 * x[0] - 3.0 * x[1] + 5.0]);
    let df = |_: &Array1<f32>| arr2(&[[2.0, 1.0], [4.0, -3.0]]);
    let x0 = arr1(&[10.0, 10.0]);
    let x = newton::newton_method(f, df, &x0);
    println!("{}", x);
}
