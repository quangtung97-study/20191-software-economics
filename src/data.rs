use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::vmi::input::Input;
use crate::vmi::relation::Relation;

pub fn read_v_mgxy(relation: &Relation, input: &mut Input) {
    let file = File::open("v_mgxy").expect("Can't open file v_mgxy");
    let reader = BufReader::new(file);

    let mut it = reader.lines();
    while let Some(line) = it.next() {
        let line = line.unwrap();
        let mut line_it = line.split_whitespace();
        if let Some(s) = line_it.next() {
            let m = s.parse::<usize>().unwrap();
            let g = line_it.next().unwrap().parse::<usize>().unwrap();

            for x in 0..relation.retailer_count {
                let line = it.next().unwrap().unwrap();
                for (y, s) in line.split_whitespace().enumerate() {
                    let v = s.parse::<f64>().unwrap();
                    input.mgxy[m][g][x][y].v = v;
                }
            }
        }
    }
}

pub fn read_ea_mgxy(relation: &Relation, input: &mut Input) {
    let file = File::open("ea_mgxy").expect("Can't open file ea_mgxy");
    let reader = BufReader::new(file);

    let mut it = reader.lines();
    while let Some(line) = it.next() {
        let line = line.unwrap();
        let mut line_it = line.split_whitespace();
        if let Some(s) = line_it.next() {
            let m = s.parse::<usize>().unwrap();
            let g = line_it.next().unwrap().parse::<usize>().unwrap();

            for x in 0..relation.retailer_count {
                let line = it.next().unwrap().unwrap();
                for (y, s) in line.split_whitespace().enumerate() {
                    let ea = s.parse::<f64>().unwrap();
                    input.mgxy[m][g][x][y].ea = ea;
                }
            }
        }
    }
}

pub fn read_beta_mgxy(relation: &Relation, input: &mut Input) {
    let file = File::open("beta_mgxy").expect("Can't open file beta_mgxy");
    let reader = BufReader::new(file);

    let mut it = reader.lines();
    while let Some(line) = it.next() {
        let line = line.unwrap();
        let mut line_it = line.split_whitespace();
        if let Some(s) = line_it.next() {
            let m = s.parse::<usize>().unwrap();
            let g = line_it.next().unwrap().parse::<usize>().unwrap();

            for x in 0..relation.retailer_count {
                let line = it.next().unwrap().unwrap();
                for (y, s) in line.split_whitespace().enumerate() {
                    let beta = s.parse::<f64>().unwrap();
                    input.mgxy[m][g][x][y].beta = beta;
                }
            }
        }
    }
}

pub fn init_ep_mgxy(relation: &Relation, input: &mut Input) {
    for m in 0..relation.retailer_count {
        for x in 0..relation.retailer_count {
            for g in 0..relation.product_count {
                for y in 0..relation.product_count {
                    input.mgxy[m][g][x][y].ep = 1.0;
                }

                for y in &relation.retailer_products[x] {
                    input.mgxy[m][g][x][*y].ep = 0.0;
                }
            }
        }
    }
}

pub fn read_K_mg(relation: &Relation, input: &mut Input) {
    let file = File::open("K_mg").expect("Can't open file K_mg");
    let reader = BufReader::new(file);
    let mut it = reader.lines();
    for m in 0..relation.retailer_count {
        let line = it.next().unwrap().unwrap();
        for (g, s) in line.split_whitespace().enumerate() {
            let K = s.parse::<f64>().unwrap();
            input.mg[m][g].K = K;
        }
    }
}

pub fn read_zeta_mg(relation: &Relation, input: &mut Input) {
    let file = File::open("zeta_mg").expect("Can't open file zeta_mg");
    let reader = BufReader::new(file);
    let mut it = reader.lines();
    for m in 0..relation.retailer_count {
        let line = it.next().unwrap().unwrap();
        for (g, s) in line.split_whitespace().enumerate() {
            let zeta = s.parse::<f64>().unwrap();
            input.mg[m][g].zeta = zeta;
        }
    }
}

pub fn read_eA_mgy(relation: &Relation, input: &mut Input) {
    let file = File::open("eA_mgy").expect("Can't open file eA_mgy");
    let reader = BufReader::new(file);
    let mut it = reader.lines();
    for m in 0..relation.retailer_count {
        for g in 0..relation.product_count {
            let line = it.next().unwrap().unwrap();
            for (y, s) in line.split_whitespace().enumerate() {
                let eA = s.parse::<f64>().unwrap();
                input.mgy[m][g][y].eA = eA;
            }
        }
    }
}

pub fn read_u_mgy(relation: &Relation, input: &mut Input) {
    let file = File::open("u_mgy").expect("Can't open file u_mgy");
    let reader = BufReader::new(file);
    let mut it = reader.lines();
    for m in 0..relation.retailer_count {
        for g in 0..relation.product_count {
            let line = it.next().unwrap().unwrap();
            for (y, s) in line.split_whitespace().enumerate() {
                let u = s.parse::<f64>().unwrap();
                input.mgy[m][g][y].u = u;
            }
        }
    }
}
