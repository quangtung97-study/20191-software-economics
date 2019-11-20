mod data;
mod newton;
mod newton_vmi;
mod vmi;

use vmi::input::Input;
use vmi::output::{MOutput, ROutput};
use vmi::relation::Relation;
use vmi::{da_NP, dp_NP, dpdp_NP, RRGame, DP, NP};

fn main() {
    let mut relation = Relation::new(3, 4);
    relation.retailer_products[0] = vec![0, 1, 2, 3];
    relation.retailer_products[1] = vec![0, 1, 2];
    relation.retailer_products[2] = vec![0, 3];

    let mut input = Input::new(&relation);

    let mut moutput = MOutput::new(&relation);
    moutput.retailer_products[0] = vec![0, 1, 3];
    moutput.retailer_products[1] = vec![0, 1];
    moutput.retailer_products[2] = vec![0, 3];

    let routput = ROutput::new(&relation);

    data::read_v_mgxy(&relation, &mut input);
    data::read_ea_mgxy(&relation, &mut input);
    data::read_beta_mgxy(&relation, &mut input);
    data::init_ep_mgxy(&relation, &mut input);
    data::read_K_mg(&relation, &mut input);
    data::read_zeta_mg(&relation, &mut input);
    data::read_eA_mgy(&relation, &mut input);
    data::read_u_mgy(&relation, &mut input);

    let game = RRGame {
        relation: &relation,
        input: &input,
        moutput: &moutput,
        routput: &routput,
    };

    println!("{}", DP(0, 0, &game));
    println!("{}", NP(0, &game));
    for j in &game.moutput.retailer_products[0] {
        println!("{}", dp_NP(0, *j, &game));
        println!("{}", da_NP(0, *j, &game));
    }

    let mut routput1 = routput.clone();
    routput1.mg[0][1].a += 0.1;
    let game1 = RRGame {
        relation: &relation,
        input: &input,
        moutput: &moutput,
        routput: &routput1,
    };

    println!("Derivative_01: {}", (NP(0, &game1) - NP(0, &game)) / 0.1);

    println!("D2: {}", dpdp_NP(1, 1, 1, &game));

    println!("Before: {}", NP(2, &game));
    println!("Before Derivative: {}", dp_NP(0, 2, &game));

    let new_routput = newton_vmi::solve_rr_problem(2, &game);
    let game2 = RRGame {
        relation: &relation,
        input: &input,
        moutput: &moutput,
        routput: &new_routput,
    };
    println!("After: {}", NP(2, &game2));
    println!("After Derivative: {}", dp_NP(0, 2, &game2));
    println!("{:?}", new_routput);
}
