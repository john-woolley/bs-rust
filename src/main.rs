use statrs::distribution::{Normal, Continuous, ContinuousCDF};

fn main() {
    let spot = 100.0;
    let strike = 105.0;
    let time_to_expiry = 1.0;
    let rate = 0.05;
    let mut put = PutOption::new(strike, time_to_expiry);
    let mut call = CallOption::new(strike, time_to_expiry);
    let straddle_px = 20.0;
    let put_vol = put.implied_volatility(spot, straddle_px, rate);
    let call_vol = call.implied_volatility(spot, straddle_px, rate);
    println!("Put Vol: {}", put_vol);
    put.price_and_risk(spot, put_vol, rate);
    println!("Put Price: {}", put.price);
    println!("Put Delta: {}", put.delta);
    println!("Put Vega: {}", put.vega);
    println!("Put Gamma: {}", put.gamma);
    println!("Put Rho: {}", put.rho);

    println!("Call Vol: {}", call_vol);
    call.price_and_risk(spot, call_vol, rate);
    println!("Call Price: {}", call.price);
    println!("Call Delta: {}", call.delta);
    println!("Call Vega: {}", call.vega);
    println!("Call Gamma: {}", call.gamma);
    println!("Call Rho: {}", call.rho);


}

struct PutOption {
    strike: f64,
    time_to_expiry: f64,
    price: f64,
    delta: f64,
    vega: f64,
    gamma: f64,
    rho: f64,
}

impl PutOption {
    fn new(strike: f64, time_to_expiry: f64) -> Self {
        PutOption {
            strike,
            time_to_expiry,
            price: 0.0,
            delta: 0.0,
            vega: 0.0,
            gamma: 0.0,
            rho: 0.0,
        }
    }

    fn price_and_risk(&mut self, spot: f64, volatility: f64, rate: f64) {
        self.price = self.price(spot, volatility, rate);
        self.delta = self.delta(spot, volatility);
        self.gamma = self.gamma(spot, volatility);
        self.rho = self.rho(spot, volatility, rate);
        self.vega = self.vega(spot, volatility);
    }

    fn vega(&mut self, spot: f64, volatility: f64) -> f64 {
        let d1 = (spot / self.strike).ln() + (0.5 * volatility.powi(2)) * self.time_to_expiry / (volatility * self.time_to_expiry.sqrt());
        spot * norm_pdf(d1) * self.time_to_expiry.sqrt()
    }

    fn rho(&mut self, spot: f64, volatility: f64, rate: f64) -> f64 {
        let d2 = (spot / self.strike).ln() + (0.5 * volatility.powi(2)) * self.time_to_expiry / (volatility * self.time_to_expiry.sqrt());
        -self.strike * self.time_to_expiry * (-rate * self.time_to_expiry).exp() * norm_cdf(-d2)
    }

    fn gamma(&mut self, spot: f64, volatility: f64) -> f64 {
        let d1 = (spot / self.strike).ln() + (0.5 * volatility.powi(2)) * self.time_to_expiry / (volatility * self.time_to_expiry.sqrt());
        norm_pdf(d1) / (spot * volatility * self.time_to_expiry.sqrt())
    }

    fn delta(&mut self, spot: f64, volatility: f64) -> f64 {
        let d1 = (spot / self.strike).ln() + (0.5 * volatility.powi(2)) * self.time_to_expiry / (volatility * self.time_to_expiry.sqrt());
        -norm_cdf(-d1)
    }

    fn price(&mut self, spot: f64, volatility: f64, rate: f64) -> f64{
        let d1 = (spot / self.strike).ln() + (0.5 * volatility.powi(2)) * self.time_to_expiry / (volatility * self.time_to_expiry.sqrt());
        let d2 = d1 - volatility * self.time_to_expiry.sqrt();
        self.strike * (-rate * self.time_to_expiry).exp() * norm_cdf(-d2) - spot * norm_cdf(-d1)
    }

    fn implied_volatility(&mut self, spot: f64, price: f64, rate: f64) -> f64 {
        let mut volatility = 0.5;
        let mut diff = self.price(spot, volatility, rate) - price;
        let mut vega = self.vega(spot, volatility);
        let epsilon = 1e-6;
        while diff.abs() > epsilon {
            volatility -= diff / vega;
            diff = self.price(spot, volatility, rate) - price;
            vega = self.vega(spot, volatility);
        }
        volatility
    }

}

struct CallOption {
    strike: f64,
    time_to_expiry: f64,
    price: f64,
    delta: f64,
    vega: f64,
    gamma: f64,
    rho: f64,
}

impl CallOption {
    fn new(strike: f64, time_to_expiry: f64) -> Self {
        CallOption {
            strike,
            time_to_expiry,
            price: 0.0,
            delta: 0.0,
            vega: 0.0,
            gamma: 0.0,
            rho: 0.0,
        }
    }

    fn price_and_risk(&mut self, spot: f64, volatility: f64, rate: f64) {
        self.price = self.price(spot, volatility, rate);
        self.delta = self.delta(spot, volatility);
        self.gamma = self.gamma(spot, volatility);
        self.rho = self.rho(spot, volatility, rate);
        self.vega = self.vega(spot, volatility);
    }

    fn vega(&mut self, spot: f64, volatility: f64) -> f64 {
        let d1 = (spot / self.strike).ln() + (0.5 * volatility.powi(2)) * self.time_to_expiry / (volatility * self.time_to_expiry.sqrt());
        spot * norm_pdf(d1) * self.time_to_expiry.sqrt()
    }

    fn rho(&mut self, spot: f64, volatility: f64, rate: f64) -> f64 {
        let d2 = (spot / self.strike).ln() + (0.5 * volatility.powi(2)) * self.time_to_expiry / (volatility * self.time_to_expiry.sqrt());
        self.strike * self.time_to_expiry * (-rate * self.time_to_expiry).exp() * norm_cdf(d2)
    }

    fn gamma(&mut self, spot: f64, volatility: f64) -> f64 {
        let d1 = (spot / self.strike).ln() + (0.5 * volatility.powi(2)) * self.time_to_expiry / (volatility * self.time_to_expiry.sqrt());
        norm_pdf(d1) / (spot * volatility * self.time_to_expiry.sqrt())
    }

    fn delta(&mut self, spot: f64, volatility: f64) -> f64 {
        let d1 = (spot / self.strike).ln() + (0.5 * volatility.powi(2)) * self.time_to_expiry / (volatility * self.time_to_expiry.sqrt());
        norm_cdf(d1)
    }

    fn price(&mut self, spot: f64, volatility: f64, rate: f64) -> f64{
        let d1 = (spot / self.strike).ln() + (0.5 * volatility.powi(2)) * self.time_to_expiry / (volatility * self.time_to_expiry.sqrt());
        let d2 = d1 - volatility * self.time_to_expiry.sqrt();
        spot * norm_cdf(d1) - self.strike * (-rate * self.time_to_expiry).exp() * norm_cdf(d2)
    }

    fn implied_volatility(&mut self, spot: f64, price: f64, rate: f64) -> f64 {
        let mut volatility = 0.5;
        let mut diff = self.price(spot, volatility, rate) - price;
        let mut vega = self.vega(spot, volatility);
        let epsilon = 1e-6;
        while diff.abs() > epsilon {
            volatility -= diff / vega;
            diff = self.price(spot, volatility, rate) - price;
            vega = self.vega(spot, volatility);
        }
        volatility
    }

}

fn norm_cdf(x: f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.cdf(x)
}

fn norm_pdf(x: f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.pdf(x)
}
