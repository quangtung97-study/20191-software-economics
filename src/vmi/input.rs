use super::relation::Relation;

#[derive(Copy, Clone, Debug)]
pub struct Input_g {
    pub pw0: f64,
    pub rho: f64,
    pub V: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Input_m {
    pub w: f64,
    pub TVR: f64,
    pub Ta: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Input_mg {
    pub zeta: f64,
    pub K: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Input_mgy {
    pub eA: f64,
    pub u: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Input_mgxy {
    pub v: f64,
    pub ea: f64,
    pub ep: f64,
    pub beta: f64,
}

impl Default for Input_g {
    fn default() -> Self {
        Self {
            pw0: 0.0,
            rho: 0.0,
            V: 1.0,
        }
    }
}

impl Default for Input_m {
    fn default() -> Self {
        Self {
            w: 1.0,
            TVR: 0.0,
            Ta: 0.0,
        }
    }
}

impl Default for Input_mg {
    fn default() -> Self {
        Self { zeta: 0.0, K: 0.0 }
    }
}

impl Default for Input_mgy {
    fn default() -> Self {
        Self { eA: 0.0, u: 0.0 }
    }
}

impl Default for Input_mgxy {
    fn default() -> Self {
        Self {
            v: 0.0,
            ea: 0.0,
            ep: 0.0,
            beta: 0.0,
        }
    }
}

pub struct Input {
    pub g: Vec<Input_g>,
    pub m: Vec<Input_m>,
    pub mg: Vec<Vec<Input_mg>>,
    pub mgy: Vec<Vec<Vec<Input_mgy>>>,
    pub mgxy: Vec<Vec<Vec<Vec<Input_mgxy>>>>,
}

impl Input {
    pub fn new(relation: &Relation) -> Self {
        let product_count = relation.product_count;
        let retailer_count = relation.retailer_count;

        // g
        let mut g: Vec<Input_g> = Vec::new();
        g.resize_with(product_count, Default::default);

        // m
        let mut m: Vec<Input_m> = Vec::new();
        m.resize_with(retailer_count, Default::default);

        // mg
        let mut mg_rank0: Vec<Input_mg> = Vec::new();
        mg_rank0.resize_with(product_count, Default::default);

        let mut mg: Vec<Vec<Input_mg>> = Vec::new();
        mg.resize(retailer_count, mg_rank0);

        // mgy
        let mut mgy_rank0: Vec<Input_mgy> = Vec::new();
        mgy_rank0.resize_with(product_count, Default::default);

        let mut mgy_rank1: Vec<Vec<Input_mgy>> = Vec::new();
        mgy_rank1.resize(product_count, mgy_rank0);

        let mut mgy = Vec::new();
        mgy.resize(retailer_count, mgy_rank1);

        // mgxy
        let mut mgxy_rank0: Vec<Input_mgxy> = Vec::new();
        mgxy_rank0.resize_with(product_count, Default::default);

        let mut mgxy_rank1: Vec<Vec<Input_mgxy>> = Vec::new();
        mgxy_rank1.resize(retailer_count, mgxy_rank0);

        let mut mgxy_rank2: Vec<Vec<Vec<Input_mgxy>>> = Vec::new();
        mgxy_rank2.resize(product_count, mgxy_rank1);

        let mut mgxy = Vec::new();
        mgxy.resize(retailer_count, mgxy_rank2);

        Self {
            g,
            m,
            mg,
            mgy,
            mgxy,
        }
    }
}
