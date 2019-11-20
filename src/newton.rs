use ndarray::{Array1, Array2};
use ndarray_linalg::Solve;

const N: usize = 100;

#[allow(dead_code)]
pub fn simple_newton_method(f: impl Fn(f64) -> f64, df: impl Fn(f64) -> f64, x0: f64) -> f64 {
    let mut x = x0;
    for _ in 0..N {
        x = x - f(x) / df(x);
    }
    return x;
}

#[allow(dead_code)]
pub fn newton_method(
    f: impl Fn(&Array1<f64>) -> Array1<f64>,
    df: impl Fn(&Array1<f64>) -> Array2<f64>,
    x0: &Array1<f64>,
) -> Array1<f64> {
    let mut x = x0.clone();
    for _ in 0..N {
        println!("x: {}", &x);
        let minus_fx = -f(&x);
        println!("-f(x): {}", &minus_fx);
        let jx = df(&x);
        println!("{}", &jx);
        let dx = match jx.solve_into(minus_fx) {
            Ok(m) => m,
            Err(_) => break,
        };
        x = x + dx;
    }
    return x;
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use ndarray::{arr1, arr2};

    #[test]
    fn test_simple_newton_method() {
        let x = simple_newton_method(|x: f64| 2.0 * x - 4.0, |_: f64| 2.0, 4.0);
        assert_approx_eq!(x, 2.0);

        let f = |x| x * x + 2.0 * x - 3.0;
        let df = |x| 2.0 * x + 2.0;

        let x = simple_newton_method(f, df, 10.0);
        assert_approx_eq!(x, 1.0);

        let x = simple_newton_method(f, df, -10.0);
        assert_approx_eq!(x, -3.0);
    }

    #[test]
    fn test_newton_method() {
        let f = |x: &Array1<f64>| arr1(&[2.0 * x[0] + x[1] - 5.0, 4.0 * x[0] - 3.0 * x[1] + 5.0]);
        let df = |_: &Array1<f64>| arr2(&[[2.0, 1.0], [4.0, -3.0]]);
        let x0 = arr1(&[10.0, 10.0]);
        let x = newton_method(f, df, &x0);
        assert_approx_eq!(x[0], 1.0);
        assert_approx_eq!(x[1], 3.0);
    }
}
