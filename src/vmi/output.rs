use super::input::Input;

#[derive(Copy, Clone)]
pub struct ROutput_mg {
    pub a: f32,
    pub p: f32,
}

#[derive(Clone)]
pub struct ROutput {
    pub mg: Vec<Vec<ROutput_mg>>,
}

#[derive(Copy, Clone)]
pub struct MOutput_g {
    pub A: f32,
}

#[derive(Clone)]
pub struct MOutput {
    pub g: Vec<MOutput_g>,
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
    pub fn new(input: &Input) -> Self {
        let mut mg_rank0: Vec<ROutput_mg> = Vec::new();
        mg_rank0.resize_with(input.product_count, Default::default);

        let mut mg: Vec<Vec<ROutput_mg>> = Vec::new();
        mg.resize(input.retailer_count, mg_rank0);

        Self { mg }
    }
}

impl MOutput {
    pub fn new(input: &Input) -> Self {
        let mut g: Vec<MOutput_g> = Vec::new();
        g.resize_with(input.product_count, Default::default);
        Self { g }
    }
}
