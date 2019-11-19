#[derive(Copy, Clone, Debug)]
pub struct Input_g {
    pub pw0: f32,
    pub rho: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Input_mg {
    pub zeta: f32,
    pub K: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Input_mgy {
    pub eA: f32,
    pub u: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Input_mgxy {
    pub v: f32,
    pub ea: f32,
    pub ep: f32,
    pub beta: f32,
}

impl Default for Input_g {
    fn default() -> Self {
        Self { pw0: 0.0, rho: 0.0 }
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
    pub retailer_count: usize,
    pub product_count: usize,
    pub retailer_products: Vec<Vec<usize>>,
    pub g: Vec<Input_g>,
    pub mg: Vec<Vec<Input_mg>>,
    pub mgy: Vec<Vec<Vec<Input_mgy>>>,
    pub mgxy: Vec<Vec<Vec<Vec<Input_mgxy>>>>,
}

impl Input {
    pub fn new(
        retailer_count: usize,
        product_count: usize,
        retailer_products: Vec<Vec<usize>>,
    ) -> Self {
        // g
        let mut g: Vec<Input_g> = Vec::new();
        g.resize_with(product_count, Default::default);

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
            retailer_count,
            product_count,
            retailer_products,
            g,
            mg,
            mgy,
            mgxy,
        }
    }
}
