use puruspe::{erf, gammp};
use rand_distr::{Binomial, Distribution};
use rand::rngs::SmallRng;
use crate::simulation::Simulation;

#[derive(Debug)]
pub enum Distance{
    Distancef64(f64),
    Distancei32(i32)
}

pub fn score_mse(sim: &Simulation, obs: &Vec<i32>) -> Distance {
    Distance::Distancei32(obs.iter().zip(sim.reports.iter())
    .fold(0, |acc, (a, b) |{
        acc + (*a - *b).pow(2)
    }))
}

pub fn gamma_cdf(x: f64, shp: f64, rt: f64) -> f64 {
    gammp(shp, rt * x)
}

pub fn lognormal_cdf(x:f64, mu: f64, sigma: f64) -> f64 {
    0.5 * (1.0 + erf((x.ln() - mu) / (sigma * 2.0_f64.sqrt())))
}

pub fn sample_multinomial(r: &mut SmallRng, p: Vec<f64>, n: i32) -> Vec<i32>{
    let mut s: i32 = n;
    let mut rho: f64 = 1.0;
    let mut x: Vec<i32> = vec![0; p.len()];
    for i in 1..=(p.len() - 1) {
        if rho.ne(&0.0){
            x[i - 1] = Binomial::new(s as u64, p[i - 1] / rho).unwrap().sample(r) as i32;
            s -= x[i - 1];
            rho -= p[i - 1];
        }
    }
    x[p.len() - 1] = s;
    x
}
