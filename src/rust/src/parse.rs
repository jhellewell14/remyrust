use extendr_api::prelude::*;
use rand_distr::{Distribution, Poisson, LogNormal, Gamma, Uniform};
use rand::{Rng, rngs::SmallRng};
use paste::paste;

use crate::error::ParseError;

// Top level user input struct to define priors
#[extendr]
pub struct Priors{
    r0: PriType,
    gt_shape: PriType,
    gt_scale: PriType,
    del_mean: PriType,
    del_sd: PriType,
    i0: PriType,
}

macro_rules! fetch_or_default {
    ($list_name: expr, $name: expr, $tf: ty, $pty: ident, $($vars: literal),*) => {
        $list_name.dollar("$name").and_then(PriType::try_from).
        unwrap_or(PriType::$pty($pty::new($($vars),*).map_err(|e| ParseError::from(e))?))
    };
}

macro_rules! list_fetch {
    ($list_name: expr, $tf: ty, $par_name: literal) => {
        $list_name.dollar($par_name)
        .and_then(<$tf>::try_from)
    }
}

// Tries to parse priors for each parameter, assigns default if missing
// tries to convert from Robj in this list to PriType
impl TryFrom<Robj> for Priors {
    type Error = extendr_api::Error;
    fn try_from(value: Robj) -> std::result::Result<Self, Self::Error> {
        let inlist = List::try_from(&value)?;

        // Tries to parse priors and uses defaults if not provided
        Ok(Priors{
            r0 : fetch_or_default!(inlist, r0, PriType, Uniform, 0.1, 3.0),
            gt_shape : fetch_or_default!(inlist, gt_shape, PriType, Uniform, 1.5, 3.0),
            gt_scale : fetch_or_default!(inlist, gt_scale, PriType, Uniform, 1.5, 3.0),
            del_mean : fetch_or_default!(inlist, del_mean, PriType, Uniform, 1.5, 3.0),
            del_sd : fetch_or_default!(inlist, del_sd, PriType, Uniform, 1.5, 3.0),  
            i0: fetch_or_default!(inlist, gt_shape, PriType, Fixed, 5.0),
        })
    }
}

// Enum listing all possible prior types
#[derive(Debug)]
enum PriType{
    Lognormal(rand_distr::LogNormal<f64>),
    Gamma(rand_distr::Gamma<f64>),
    Fixed(Fixed<f64>),
    Uniform(rand_distr::Uniform<f64>),
    Poisson(rand_distr::Poisson<f64>),
}

// Convert from list with string of prior distribution type and parameters into an actual prior
impl TryFrom<Robj> for PriType {
    type Error = extendr_api::Error;
    fn try_from(value: Robj) -> std::result::Result<Self, Self::Error> {
        let inlist = List::try_from(&value)?;
        let dist = list_fetch!(inlist, String, "dist")?;

        match dist.as_str() {
            "uniform "=> {PriType::build_Uniform(list_fetch!(inlist, f64, "upper"),
                 list_fetch!(inlist, f64, "lower"))},
            "gamma" => {PriType::build_Gamma(list_fetch!(inlist, f64, "shape"),
             list_fetch!(inlist, f64, "scale"))}
            "lognormal" => {PriType::build_Lognormal(list_fetch!(inlist, f64, "mu"),
             list_fetch!(inlist, f64, "sigma"))},
            "poisson" => {PriType::build_Poisson(list_fetch!(inlist, f64, "lambda"))},
            "Fixed" => {PriType::build_Fixed(list_fetch!(inlist, f64, "value"))}
            _ => {Err(extendr_api::Error::Other(
                format!("Could not recognise prior string: {}", dist)))}
        }

    }
}

macro_rules! build_impl {
    ($dist_name: ident, $pyt: ty, $($vars: ident),*) => {
    paste!{
        fn [<build_ $dist_name>]($($vars: Result<f64>),*) -> Result<PriType> {
            $(let $vars = $vars?;)*
            <$pyt>::new($($vars),*)
            .map_err(|e| extendr_api::Error::Other(
                format!("Trying to build {} distribution with parameters {} and recieved this error {}", stringify!($pyt), stringify!($($vars: {} and )*), e.to_string())))
            .map(|dist| PriType::$dist_name(dist))
        }
    }
    }
}

impl PriType {
    build_impl!(Uniform, Uniform<f64>, upper, lower);
    build_impl!(Lognormal, LogNormal<f64>, mu, sigma);
    build_impl!(Poisson, Poisson<f64>, lambda);
    build_impl!(Fixed, Fixed<f64>, value);
    build_impl!(Gamma, Gamma<f64>, shape, scale);
}


// Struct for "Fixed" distribution that always samples a fixed value
// then I implement the Distribution trait for it so I can sample it
#[derive(Debug)]
pub struct Fixed<T: Copy>{
    value: T,
}
impl<T: Copy> Fixed<T> {
    pub fn new(value: T) -> Result<Self>{
        Ok(Fixed{
            value
        })
    }
}
impl<T: Copy> rand_distr::Distribution<T> for Fixed<T> {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> T {
        self.value
    }
}

// Sample any PriType
impl PriType {
    pub fn sample(&self, r: &mut SmallRng) -> f64 {
        match self {
            PriType::Lognormal(d) => {
                d.sample(r)
            },
            PriType::Gamma(d) => {
                d.sample(r)
            },
            PriType::Fixed(d) => {
                d.sample(r)
            },
            PriType::Uniform(d) => {
                d.sample(r)
            }
            PriType::Poisson(d) => {
                d.sample(r)
            }
        }
    }
}

// Struct to contain parameters sampled from priors
// for a simulation run
pub struct Param{
    pub r0: f64,
    pub gt_shape: f64,
    pub gt_scale: f64,
    pub del_mean: f64,
    pub del_sd: f64,
    pub i0: f64,
}

// Method to sample from all priors and generate Param
impl Priors {
    pub fn sample(&self, r: &mut SmallRng) -> Param {
        Param { r0: self.r0.sample(r), 
                gt_shape: self.gt_shape.sample(r), 
                gt_scale: self.gt_scale.sample(r),
                del_mean: self.del_mean.sample(r),
                del_sd: self.del_sd.sample(r),
                i0: self.i0.sample(r)}
    }
}