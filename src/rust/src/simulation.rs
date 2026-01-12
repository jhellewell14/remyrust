use crate::parse::Param;
use crate::utils::{gamma_cdf, sample_multinomial, lognormal_cdf, Distance};

use rand::{rngs::SmallRng, SeedableRng};
use std::iter::successors;
use rand_distr::{Distribution, Poisson, Binomial};


pub struct Simulation{
    pub pars: Param,
    pub onsets: Vec<i32>,
    pub reports: Vec<i32>,
}

pub fn simulate(pars: Param, r: &mut SmallRng, ngen: usize, max_time: usize) -> Simulation {

        // Create iterator for generation sizes
        let gen_size = successors(Some(pars.i0 as i32), |prev|{
        if prev.eq(&0) {
            Some(0)
            } else {
            Some(Poisson::new(pars.r0 * *prev as f64).unwrap().sample(r) as i32)
        }
        });

        // Create iterator for generation times
        let gen: Vec<i32> = (0..ngen as i32).collect();
        let gendist = gen.iter().map(|gen|{
            let i = *gen as f64 + 1.0;
            let mut out: Vec<f64> = vec![];
            for j in 1..=max_time {
                let t = j as f64;
                let p: f64 = gamma_cdf(t + (pars.gt_shape / pars.gt_scale), pars.gt_shape * i, pars.gt_scale) - 
                gamma_cdf(t + (pars.gt_shape / pars.gt_scale) - 1.0, pars.gt_shape * i, pars.gt_scale);
                out.push(p.max(0.0));
            }
            let s: f64 = out.iter().sum();
            out.push(1.0 - s);
            out
        });

        // Loop over generation sizes and times
        let mut onsets: Vec<i32> = vec![0; max_time];
        let mut r2 = SmallRng::from_os_rng();
        for (size, dist) in gen_size.zip(gendist).take(ngen) {

            // Take a multinomial sample for this generation
            let samp: Vec<i32> = sample_multinomial(&mut r2, dist, size);

            // Add new onsets to total onsets vector
            onsets
            .iter_mut()
            .zip(samp)
            .for_each(|(rep, new_inf)| *rep += new_inf);
        }

        let reports: Vec<i32> = onsets.iter().enumerate().map(|(i, x)| {
            let t: f64 = (max_time - i) as f64;
            let p: f64 = lognormal_cdf(t, pars.del_mean, pars.del_sd);
            let mut r = SmallRng::from_os_rng();
            Binomial::new(*x as u64, p).unwrap().sample(&mut r) as i32
        })
        .collect();


    Simulation {pars, 
                onsets,
                reports}
}

impl Simulation {
    pub fn distance<F: Fn(&Self, &Vec<i32>) -> Distance>(&self, f: F, obs: &Vec<i32>) -> Distance {
        f(&self, obs)
    }
}