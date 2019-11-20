use super::input::Input;
use super::relation::Relation;
use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone)]
pub struct ROutput_mg {
    pub a: f64,
    pub p: f64,
}

#[derive(Debug, Clone)]
pub struct ROutput {
    pub mg: Vec<Vec<ROutput_mg>>,
}

#[derive(Debug, Copy, Clone)]
pub struct MOutput_g {
    pub A: f64,
}

#[derive(Debug, Clone)]
pub struct MOutput {
    pub g: Vec<MOutput_g>,
    pub retailer_products: Vec<Vec<usize>>,
}

impl Default for ROutput_mg {
    fn default() -> Self {
        Self { a: 0.0, p: 0.0 }
    }
}

impl Default for MOutput_g {
    fn default() -> Self {
        Self { A: 0.0 }
    }
}

impl ROutput {
    pub fn new(relation: &Relation) -> Self {
        let mut rng = thread_rng();
        let f = || ROutput_mg {
            p: rng.gen_range(0.1, 50.0),
            a: rng.gen_range(0.1, 50.0),
        };

        let mut mg_rank0: Vec<ROutput_mg> = Vec::new();
        mg_rank0.resize_with(relation.product_count, f);

        let mut mg: Vec<Vec<ROutput_mg>> = Vec::new();
        mg.resize(relation.retailer_count, mg_rank0);

        Self { mg }
    }
}

impl MOutput {
    pub fn new(relation: &Relation) -> Self {
        let mut rng = thread_rng();
        let f = || MOutput_g { A: 0.0 };

        let mut g: Vec<MOutput_g> = Vec::new();
        g.resize_with(relation.product_count, f);

        let mut retailer_products = Vec::new();
        retailer_products.resize_with(relation.retailer_count, Default::default);

        Self {
            g,
            retailer_products,
        }
    }
}
