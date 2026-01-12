use extendr_api::prelude::*;
// use statrs::distribution::Poisson;
use core::fmt;
use std::error::Error;
use simulation::{run_simulation};
use utils::*;
mod params;
mod error;
mod simulation;
mod utils;
use rand_distr::{Binomial, Distribution, Poisson, LogNormal};
use rand::{SeedableRng, rngs::SmallRng};

use crate::params::priors_and_param_structs;

// pub fn lgn(mu: f64, sigma: f64) -> PriorDistributions {
//     PriorDistributions::Lognormal(mu, sigma)
// }

#[extendr]
pub fn run_from_r2(r0_mean: f64, r0_sd: f64, // Basic reproduction number
                      i0_rate: f64, // Size of 1st generation
                      ngen: usize, // Number of generations to simulate
                      obs: Vec<i32>, // Vector of observed onsets by date of report
                      gt_shape_mean: f64, gt_shape_sd: f64, // Generation time distribution shape
                      gt_rate_mean: f64, gt_rate_sd: f64, // Generation time distribution rate
                      delay_mean_mean: f64, delay_mean_sd: f64, // Delay distribution mean
                      delay_sd_mean: f64, delay_sd_sd: f64, // Delay distribution sd
                      ) -> Result<Vec<i32>>{

            let max_time: usize = obs.len();

            priors_and_param_structs!((r0, f64, R0Error, LogNormal, f64, r0_mean, r0_sd),
                                      (i0, i32, I0Error, Poisson, f64, i0_rate));

            // #[derive(Default)]
            // struct PriorBuilder {
            //     r0: Option<LogNormal<f64>>,
            //     gt_shape: Option<LogNormal<f64>>,
            //     gt_rate: Option<LogNormal<f64>>,
            //     delay_mean: Option<LogNormal<f64>>,
            //     delay_sd: Option<LogNormal<f64>>,
            //     i0: Option<LogNormal<f64>>,
            // }

            // struct Prior {
            //     r0: LogNormal<f64>,
            // }

            impl PriorBuilder {
                // pub fn r0(mut self, r0_mean: f64, r0_sd: f64) -> Self{
                //     self.r0.insert(LogNormal::new(r0_mean, r0_sd).unwrap());
                //     self
                // }

                pub fn build(self) -> Result<Prior>{
                    let Some(r0) = self.r0 else {
                        return Err(extendr_api::Error::Other("Bad r0".into()))
                    };
                    Ok(Prior{r0})
                }
            }

            pub struct Param {
                r0: f64,
            }
            

            impl Prior {
                pub fn sample(self) -> Param {
                    let mut r = SmallRng::from_os_rng();
                    let r0 = self.r0.sample(&mut r);
                    Param {r0}
                }
            }

            let p: Result<Prior> = PriorBuilder::default().r0(r0_mean, r0_sd).build();

            let pp: Param = p.unwrap().sample();

            // let priors = PriorBuilder::new()
            // $(.$param_name($prior_type))*
            // .build()
            // .expect("Prior failure");
            // let priors = Priors{
            //     $($param_name: $prior_type,)*
            // };

            // let params = ParamBuilder::new()
            // $(.$param_name($param_name))*
            // .build()
            // .expect("Parameter failure");
            
            // let sim = run_simulation(params);

            // let result = score_sim(sim, obs, score_mse);

            // Ok(result.sim.reports)

            todo!()
}


params::set_params!((r0, f64, R0Error, lgn, 1.0, 1.5), 
    (i0, i32, I0Error, pois, 1.0), 
    (ngen, usize, NgenError, fix, 5_usize),
    (max_time, usize, MaxTimeError, fix, 25_usize), 
    (gt_shape, f64, GenShapeError, lgn, 1.0, 1.5),
    (gt_rate, f64, GenRateError, lgn, 1.0, 1.5),
    (delay_mean, f64, DelayMeanError, lgn, 1.0, 1.5), 
    (delay_sd, f64, DelaySdError, lgn, 1.0, 1.5));


// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod remyrust;
    fn run_from_r;
}


