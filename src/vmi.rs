pub mod input;
pub mod output;

use input::Input;
use output::{MOutput, ROutput};

fn safe_pow(a: f32, n: f32) -> f32 {
    if a <= 0.0 {
        0.0
    } else if n == 0.0 {
        1.0
    } else {
        f32::powf(a, n)
    }
}

#[allow(dead_code)]
pub fn DP(m: usize, g: usize, input: &Input, moutput: &MOutput, routput: &ROutput) -> f32 {
    let mut sum = 0.0;
    for y in 0..input.product_count {
        sum += input.mg[m][g].K
            + input.mgy[m][g][y].u * safe_pow(moutput.g[y].A, input.mgy[m][g][y].eA);
    }

    for x in 0..input.retailer_count {
        for y in &input.retailer_products[x] {
            sum += input.mgxy[m][g][x][*y].beta
                * safe_pow(routput.mg[x][*y].p, input.mgxy[m][g][x][*y].ep);
            sum += input.mgxy[m][g][x][*y].v
                * safe_pow(routput.mg[x][*y].a, input.mgxy[m][g][x][*y].ea);
        }
    }
    return sum;
}

#[allow(dead_code)]
pub fn pw(m: usize, g: usize, input: &Input, moutput: &MOutput, routput: &ROutput) -> f32 {
    input.g[g].pw0 - input.g[g].rho * DP(m, g, input, moutput, routput)
}

#[allow(dead_code)]
pub fn NP(m: usize, input: &Input, moutput: &MOutput, routput: &ROutput) -> f32 {
    let mut sum = 0.0;
    for g in &input.retailer_products[m] {
        sum += DP(m, *g, input, moutput, routput) * routput.mg[m][*g].p;
        sum -= DP(m, *g, input, moutput, routput) * pw(m, *g, input, moutput, routput);
        sum -= DP(m, *g, input, moutput, routput) * input.mg[m][*g].zeta;
        sum -= routput.mg[m][*g].a;
    }
    return sum;
}

pub fn dp_DP(m: usize, g: usize, j: usize, input: &Input, routput: &ROutput) -> f32 {
    input.mgxy[m][g][m][j].beta
        * input.mgxy[m][g][m][j].ep
        * safe_pow(routput.mg[m][j].p, input.mgxy[m][g][m][j].ep - 1.0)
}

pub fn dp_pw(m: usize, g: usize, j: usize, input: &Input, routput: &ROutput) -> f32 {
    -input.g[g].rho * dp_DP(m, g, j, input, routput)
}

#[allow(dead_code)]
pub fn dp_NP(m: usize, j: usize, input: &Input, moutput: &MOutput, routput: &ROutput) -> f32 {
    let mut sum = 0.0;

    sum += DP(m, j, input, moutput, routput);
    for g in &input.retailer_products[m] {
        sum += dp_DP(m, *g, j, input, routput) * routput.mg[m][*g].p;
    }

    for g in &input.retailer_products[m] {
        sum -= dp_DP(m, *g, j, input, routput) * pw(m, *g, input, moutput, routput);
        sum -= DP(m, *g, input, moutput, routput) * dp_pw(m, *g, j, input, routput);
    }

    for g in &input.retailer_products[m] {
        sum -= input.mg[m][*g].zeta * dp_DP(m, *g, j, input, routput);
    }

    return sum;
}

pub fn da_DP(m: usize, g: usize, j: usize, input: &Input, routput: &ROutput) -> f32 {
    input.mgxy[m][g][m][j].v
        * input.mgxy[m][g][m][j].ea
        * safe_pow(routput.mg[m][j].a, input.mgxy[m][g][m][j].ea - 1.0)
}

pub fn da_pw(m: usize, g: usize, j: usize, input: &Input, routput: &ROutput) -> f32 {
    -input.g[g].rho * da_DP(m, g, j, input, routput)
}

#[allow(dead_code)]
pub fn da_NP(m: usize, j: usize, input: &Input, moutput: &MOutput, routput: &ROutput) -> f32 {
    let mut sum = 0.0;
    for g in &input.retailer_products[m] {
        sum += da_DP(m, *g, j, input, routput) * routput.mg[m][*g].p;
    }

    for g in &input.retailer_products[m] {
        sum -= da_DP(m, *g, j, input, routput) * pw(m, *g, input, moutput, routput);
        sum -= DP(m, *g, input, moutput, routput) * da_pw(m, *g, j, input, routput);
    }

    for g in &input.retailer_products[m] {
        sum -= input.mg[m][*g].zeta * da_DP(m, *g, j, input, routput);
    }

    sum -= 1.0;

    return sum;
}
