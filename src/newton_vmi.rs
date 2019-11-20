use crate::newton;
use crate::vmi::input::Input;
use crate::vmi::output::{MOutput, ROutput};
use crate::vmi::{da_NP, dada_NP, dadp_NP, dp_NP, dpda_NP, dpdp_NP, RRGame};
use ndarray::{Array1, Array2};

#[allow(dead_code)]
pub fn to_newton_input(m: usize, game: &RRGame) -> Array1<f64> {
    let routput = game.routput;
    let products = &game.moutput.retailer_products[m];

    let mut vec: Vec<f64> = Vec::new();

    for g in products {
        vec.push(routput.mg[m][*g].p);
    }

    for g in products {
        vec.push(routput.mg[m][*g].a);
    }

    return Array1::from(vec);
}

#[allow(dead_code)]
pub fn from_newton_array(m: usize, newton_input: &Array1<f64>, game: &RRGame) -> ROutput {
    let mut result = game.routput.clone();
    let products = &game.moutput.retailer_products[m];
    let length = products.len();

    for index in 0..length {
        let g = products[index];
        let value = newton_input[index];
        result.mg[m][g].p = value;
    }

    for index in length..(length * 2) {
        let g = products[index - length];
        let value = newton_input[index];
        result.mg[m][g].a = value;
    }

    return result;
}

#[allow(dead_code)]
pub fn newton_f(x: &Array1<f64>, m: usize, __game: &RRGame) -> Array1<f64> {
    let routput = from_newton_array(m, x, __game);
    let products = &__game.moutput.retailer_products[m];
    let game = RRGame {
        routput: &routput,
        ..*__game
    };

    let mut result: Vec<f64> = Vec::new();

    for j in products {
        result.push(dp_NP(m, *j, &game));
    }

    for j in products {
        result.push(da_NP(m, *j, &game));
    }

    return Array1::from(result);
}

#[allow(dead_code)]
pub fn newton_df(x: &Array1<f64>, m: usize, __game: &RRGame) -> Array2<f64> {
    let routput = from_newton_array(m, x, __game);
    let game = RRGame {
        routput: &routput,
        ..*__game
    };

    println!("ROutput: {:?}", routput);
    let products = &game.moutput.retailer_products[m];
    let len = products.len();
    let dlen = len * 2;
    let mut result: Array2<f64> = Array2::zeros((dlen, dlen));

    for row in 0..dlen {
        for col in 0..dlen {
            result[[row, col]] = if row < len {
                let j = products[row];
                if col < len {
                    let k = products[col];
                    dpdp_NP(m, j, k, &game)
                } else {
                    let k = products[col - len];
                    dpda_NP(m, j, k, &game)
                }
            } else {
                let j = products[row - len];
                if col < len {
                    let k = products[col];
                    dadp_NP(m, j, k, &game)
                } else {
                    let k = products[col - len];
                    dada_NP(m, j, k, &game)
                }
            };
        }
    }

    return result;
}

#[allow(dead_code)]
pub fn solve_rr_problem(m: usize, game: &RRGame) -> ROutput {
    let x0 = to_newton_input(m, game);
    let f = |x: &Array1<f64>| newton_f(x, m, game);
    let df = |x: &Array1<f64>| newton_df(x, m, game);

    let x = newton::newton_method(f, df, &x0);
    return from_newton_array(m, &x, game);
}
