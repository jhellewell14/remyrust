use extendr_api::prelude::*;
use rand::{rngs::SmallRng, SeedableRng};

use parse::{Priors, Param};
use simulation::simulate;
use utils::score_mse;

mod error;
mod simulation;
mod parse;
mod utils;

#[extendr]
pub fn r_entry(par_input: Robj, 
    ngen: usize, max_time: usize, 
    obs: Vec<i32>, iter: usize) -> Result<f64> {

    let priors: Priors = Priors::try_from(par_input)?;

    let mut r = SmallRng::from_os_rng();

    for i in 1..=iter {
        if i.rem_euclid(1000).eq(&0){
            println!("Iteration {i}");
        }
        let par: Param = priors.sample(&mut r);

        let sim = simulate(par, &mut r, ngen, max_time);
        let dist = sim.distance(score_mse, &obs);

        // println!("Distance: {:?}", dist);
    }
    Ok(1.0)
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod remyrust;
    fn r_entry;
}

// #[derive(Default)]
// pub struct Pv(Box<[PriType]>);

// impl Pv {
//     pub fn sample(&self) -> Vec<f64>{
//         let mut r = SmallRng::from_os_rng();
//         self.0.iter().map(|el| el.sample(&mut r)).collect()
//     }
// }
   

// pub struct Pc{
//     r0: PriType,
// }
