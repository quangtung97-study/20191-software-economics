pub mod input;
pub mod output;
pub mod relation;

use input::Input;
use output::{MOutput, ROutput};
use relation::Relation;

fn safe_pow(a: f64, n: f64) -> f64 {
    if a <= 0.0 {
        0.0
    } else if n == 0.0 {
        1.0
    } else {
        f64::powf(a, n)
    }
}

pub struct RRGame<'a, 'b, 'c, 'd> {
    pub relation: &'a Relation,
    pub input: &'b Input,
    pub moutput: &'c MOutput,
    pub routput: &'d ROutput,
}

#[allow(dead_code)]
pub fn DP(m: usize, g: usize, game: &RRGame) -> f64 {
    let product_count = game.relation.product_count;
    let retailer_count = game.relation.retailer_count;
    let retailer_products = &game.moutput.retailer_products;
    let input = game.input;
    let moutput = game.moutput;
    let routput = game.routput;

    let mut sum = 0.0;
    for y in 0..product_count {
        sum += input.mg[m][g].K
            + input.mgy[m][g][y].u * safe_pow(moutput.g[y].A, input.mgy[m][g][y].eA);
    }

    for x in 0..retailer_count {
        for y in &retailer_products[x] {
            sum += input.mgxy[m][g][x][*y].beta
                * safe_pow(routput.mg[x][*y].p, input.mgxy[m][g][x][*y].ep);
            sum += input.mgxy[m][g][x][*y].v
                * safe_pow(routput.mg[x][*y].a, input.mgxy[m][g][x][*y].ea);
        }
    }
    return sum;
}

#[allow(dead_code)]
pub fn pw(m: usize, g: usize, game: &RRGame) -> f64 {
    let input = game.input;
    input.g[g].pw0 - input.g[g].rho * DP(m, g, game)
}

#[allow(dead_code)]
pub fn NP(m: usize, game: &RRGame) -> f64 {
    let retailer_products = &game.moutput.retailer_products;
    let input = game.input;
    let routput = game.routput;

    let mut sum = 0.0;

    for g in &retailer_products[m] {
        sum += DP(m, *g, game) * routput.mg[m][*g].p;
        sum -= DP(m, *g, game) * pw(m, *g, game);
        sum -= DP(m, *g, game) * input.mg[m][*g].zeta;
        sum -= routput.mg[m][*g].a;
    }
    return sum;
}

pub fn dp_DP(m: usize, g: usize, j: usize, game: &RRGame) -> f64 {
    let input = game.input;
    let routput = game.routput;

    let beta_mgmj = input.mgxy[m][g][m][j].beta;
    let ep_mgmj = input.mgxy[m][g][m][j].ep;
    let p_mj = routput.mg[m][j].p;

    beta_mgmj * ep_mgmj * safe_pow(p_mj, ep_mgmj - 1.0)
}

pub fn dp_pw(m: usize, g: usize, j: usize, game: &RRGame) -> f64 {
    let input = game.input;
    -input.g[g].rho * dp_DP(m, g, j, game)
}

#[allow(dead_code)]
pub fn dp_NP(m: usize, j: usize, game: &RRGame) -> f64 {
    let retailer_products = &game.moutput.retailer_products;
    let input = game.input;
    let routput = game.routput;

    let mut sum = 0.0;

    sum += DP(m, j, game);
    for g in &retailer_products[m] {
        sum += dp_DP(m, *g, j, game) * routput.mg[m][*g].p;
    }

    for g in &retailer_products[m] {
        sum -= dp_DP(m, *g, j, game) * pw(m, *g, game);
        sum -= DP(m, *g, game) * dp_pw(m, *g, j, game);
    }

    for g in &retailer_products[m] {
        sum -= input.mg[m][*g].zeta * dp_DP(m, *g, j, game);
    }

    return sum;
}

pub fn da_DP(m: usize, g: usize, j: usize, game: &RRGame) -> f64 {
    let input = game.input;
    let routput = game.routput;

    input.mgxy[m][g][m][j].v
        * input.mgxy[m][g][m][j].ea
        * safe_pow(routput.mg[m][j].a, input.mgxy[m][g][m][j].ea - 1.0)
}

pub fn da_pw(m: usize, g: usize, j: usize, game: &RRGame) -> f64 {
    let input = game.input;
    -input.g[g].rho * da_DP(m, g, j, game)
}

#[allow(dead_code)]
pub fn da_NP(m: usize, j: usize, game: &RRGame) -> f64 {
    let retailer_products = &game.moutput.retailer_products;
    let input = game.input;
    let routput = game.routput;

    let mut sum = 0.0;

    for g in &retailer_products[m] {
        sum += da_DP(m, *g, j, game) * routput.mg[m][*g].p;
    }

    for g in &retailer_products[m] {
        sum -= da_DP(m, *g, j, game) * pw(m, *g, game);
        sum -= DP(m, *g, game) * da_pw(m, *g, j, game);
    }

    for g in &retailer_products[m] {
        sum -= input.mg[m][*g].zeta * da_DP(m, *g, j, game);
    }

    sum -= 1.0;

    return sum;
}

pub fn dpdp_DP(m: usize, g: usize, j: usize, k: usize, game: &RRGame) -> f64 {
    let input = game.input;
    let routput = game.routput;

    if j != k {
        0.0
    } else {
        let beta_mgmj = input.mgxy[m][g][m][j].beta;
        let ep_mgmj = input.mgxy[m][g][m][j].ep;
        let p_mj = routput.mg[m][j].p;

        beta_mgmj * ep_mgmj * (ep_mgmj - 1.0) * safe_pow(p_mj, ep_mgmj - 2.0)
    }
}

pub fn dada_DP(m: usize, g: usize, j: usize, k: usize, game: &RRGame) -> f64 {
    let input = game.input;
    let routput = game.routput;

    if j != k {
        0.0
    } else {
        let v_mgmj = input.mgxy[m][g][m][j].v;
        let ea_mgmj = input.mgxy[m][g][m][j].ea;
        let a_mj = routput.mg[m][j].a;

        v_mgmj * ea_mgmj * (ea_mgmj - 1.0) * safe_pow(a_mj, ea_mgmj - 2.0)
    }
}

pub fn dpdp_pw(m: usize, g: usize, j: usize, k: usize, game: &RRGame) -> f64 {
    let input = game.input;

    let rho_g = input.g[g].rho;
    -rho_g * dpdp_DP(m, g, j, k, game)
}

pub fn dada_pw(m: usize, g: usize, j: usize, k: usize, game: &RRGame) -> f64 {
    let input = game.input;
    let rho_g = input.g[g].rho;
    -rho_g * dada_DP(m, g, j, k, game)
}

#[allow(dead_code)]
pub fn dpdp_NP(m: usize, j: usize, k: usize, game: &RRGame) -> f64 {
    let retailer_products = &game.moutput.retailer_products;
    let input = game.input;
    let routput = game.routput;

    let mut sum = 0.0;

    sum += dp_DP(m, k, j, game);
    sum += dp_DP(m, j, k, game);
    for g in &retailer_products[m] {
        sum += dpdp_DP(m, *g, j, k, game) * routput.mg[m][*g].p;
    }

    for g in &retailer_products[m] {
        sum -= dpdp_DP(m, *g, j, k, game) * pw(m, *g, game);
        sum -= dpdp_pw(m, *g, j, k, game) * DP(m, *g, game);
        sum -= dp_DP(m, *g, j, game) * dp_pw(m, *g, k, game);
        sum -= dp_DP(m, *g, k, game) * dp_pw(m, *g, j, game);
    }

    for g in &retailer_products[m] {
        sum -= input.mg[m][*g].zeta * dpdp_DP(m, *g, j, k, game);
    }

    return sum;
}

#[allow(dead_code)]
pub fn dpda_NP(m: usize, j: usize, k: usize, game: &RRGame) -> f64 {
    let retailer_products = &game.moutput.retailer_products;

    let mut sum = 0.0;

    sum += da_DP(m, j, k, game);

    for g in &retailer_products[m] {
        sum -= dp_DP(m, *g, j, game) * da_pw(m, *g, k, game);
        sum -= da_DP(m, *g, k, game) * dp_pw(m, *g, j, game);
    }

    return sum;
}

#[allow(dead_code)]
pub fn dadp_NP(m: usize, j: usize, k: usize, game: &RRGame) -> f64 {
    let retailer_products = &game.moutput.retailer_products;

    let mut sum = 0.0;

    sum += da_DP(m, k, j, game);

    for g in &retailer_products[m] {
        sum -= dp_DP(m, *g, k, game) * da_pw(m, *g, j, game);
        sum -= da_DP(m, *g, j, game) * dp_pw(m, *g, k, game);
    }

    return sum;
}

#[allow(dead_code)]
pub fn dada_NP(m: usize, j: usize, k: usize, game: &RRGame) -> f64 {
    let retailer_products = &game.moutput.retailer_products;
    let input = game.input;
    let routput = game.routput;

    let mut sum = 0.0;

    for g in &retailer_products[m] {
        sum += dada_DP(m, *g, j, k, game) * routput.mg[m][*g].p;
    }

    for g in &retailer_products[m] {
        sum -= dada_DP(m, *g, j, k, game) * pw(m, *g, game);
        sum -= da_DP(m, *g, j, game) * da_pw(m, *g, k, game);
        sum -= da_DP(m, *g, k, game) * da_pw(m, *g, j, game);
        sum -= DP(m, *g, game) * dada_pw(m, *g, j, k, game);
    }

    for g in &retailer_products[m] {
        sum -= input.mg[m][*g].zeta * dada_DP(m, *g, j, k, game);
    }

    return sum;
}
