// there are a number of ways you can structure the data, I found this most intutitve to understand later on
#![allow(dead_code)]

pub struct OptionContract {
    pub strike: f64,
    pub expiry: f64,
    pub option_type: OptionType,
}

pub enum OptionType {
    Call,
    Put,
}

pub struct EuropeanOption {
    pub contract: OptionContract,
}

pub struct AmericanOption {
    contract: OptionContract,
}

impl OptionContract {
    fn payoff(&self, spot: f64) -> f64 {
        match self.option_type {
            OptionType::Call => (spot - self.strike).max(0.0),
            OptionType::Put => (self.strike - spot).max(0.0),
        }
    }
}

pub struct Binomial {
    pub steps: usize,
}

pub struct MarketData {
    pub spot: f64,
    pub rate: f64,
    pub vol: f64,
    pub div: f64,
}

// will be implemented for both american and euroopean option types
pub trait Priceable {
    fn price(&self, engine: &Binomial, data: &MarketData) -> f64;
}

impl Priceable for EuropeanOption {
    fn price(&self, engine: &Binomial, data: &MarketData) -> f64 {
        let OptionContract { expiry, .. } = self.contract;
        let MarketData {
            spot,
            rate,
            vol,
            div,
        } = data;
        let steps = engine.steps;

        let dt: f64 = expiry / steps as f64;
        let u = ((rate - div) * dt + vol * dt.sqrt()).exp();
        let d = ((rate - div) * dt - vol * dt.sqrt()).exp();
        let pu = (((rate - div) * dt).exp() - d) / (u - d);
        let pd = 1.0 - pu;
        let disc = (-rate * dt).exp();

        let mut x: Vec<f64> = (1..=steps + 1)
            .map(|i| {
                let spot_at_node = spot * u.powi((steps + 1 - i) as i32) * d.powi((i - 1) as i32); // this gives me the s values
                self.contract.payoff(spot_at_node) // this returns the x values
            })
            .collect();

        for j in (1..=steps).rev() {
            for i in 0..j {
                x[i] = disc * (pu * x[i] + pd * x[i + 1])
            }
        }

        x[0]
    }
}

// impl Priceable for AmericanOption {
//     fn price(&self, engine: &Binomial, data: &MarketData) -> f64 {
//         todo!();
//         let OptionContract { expiry, .. } = self.contract;
//         let MarketData {
//             spot,
//             rate,
//             vol,
//             div,
//         } = data;
//         let steps = engine.steps;

//         let dt: f64 = expiry / steps as f64;
//         let u = ((rate - div) * dt + vol * dt.sqrt()).exp();
//         let d = ((rate - div) * dt - vol * dt.sqrt()).exp();
//         let pu = (((rate - div) * dt).exp() - d) / (u - d);
//         let pd = 1.0 - pu;
//         let disc = (-rate * dt).exp();

//         // matrix here...

//     }
// }
