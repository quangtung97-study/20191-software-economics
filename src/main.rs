mod data;
mod newton;
mod vmi;

use vmi::input::Input;
use vmi::output::{MOutput, ROutput};
use vmi::{da_NP, dp_NP, DP, NP};

fn main() {
    let mut input = Input::new(3, 4, vec![vec![0, 1, 2, 3], vec![0, 1, 2], vec![0, 3]]);

    let moutput = MOutput::new(&input);
    let routput = ROutput::new(&input);

    data::read_v_mgxy("v_mgxy", &mut input);
    data::read_ea_mgxy("ea_mgxy", &mut input);
    data::read_beta_mgxy("beta_mgxy", &mut input);
    data::init_ep_mgxy(&mut input);
    data::read_K_mg("K_mg", &mut input);
    data::read_zeta_mg("zeta_mg", &mut input);
    data::read_eA_mgy("eA_mgy", &mut input);
    data::read_u_mgy("u_mgy", &mut input);

    println!("{}", DP(0, 0, &input, &moutput, &routput));
    println!("{}", NP(0, &input, &moutput, &routput));
    for j in &input.retailer_products[0] {
        println!("{}", dp_NP(0, *j, &input, &moutput, &routput));
        println!("{}", da_NP(0, *j, &input, &moutput, &routput));
    }

    let mut routput1 = routput.clone();
    routput1.mg[0][1].a = 0.2;
    println!(
        "Derivative: {}",
        (NP(0, &input, &moutput, &routput1) - NP(0, &input, &moutput, &routput)) / 0.2
    );
}
