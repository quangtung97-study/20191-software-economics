pub struct Relation {
    pub retailer_count: usize,
    pub product_count: usize,
    pub retailer_products: Vec<Vec<usize>>,
}

impl Relation {
    pub fn new(retailer_count: usize, product_count: usize) -> Self {
        let mut retailer_products = Vec::new();
        retailer_products.resize_with(retailer_count, Default::default);

        Self {
            retailer_count,
            product_count,
            retailer_products,
        }
    }
}
