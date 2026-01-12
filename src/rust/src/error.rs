use std::error::Error;
use std::fmt::Display;

// Error type for errors coming from my code such as parsing or prior sampling
#[derive(Debug)]
pub enum ParseError{
    BadParameter(Option<f64>),
    BadPriorString(String),
    BadDistribution(String),
    Rextendr(extendr_api::Error),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Rextendr(e) => {write!(f, "extendr error: {:?}", e.to_string())}
            ParseError::BadParameter(x) => {write!(f, "Bad parameter {x:?}")},
            ParseError::BadPriorString(x) => {write!(f, "Bad prior string {x}")},
            ParseError::BadDistribution(x) => {write!(f, "Distribution error: {x}")}
        }
    }
}

impl Error for ParseError{}

// Convert my errors to extendr_api::Error so that I can use ? operator smoothly
impl From<ParseError> for extendr_api::Error {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::Rextendr(e) => e,
            ParseError::BadParameter(_) => {extendr_api::Error::Other(value.to_string())},
            ParseError::BadPriorString(x) => extendr_api::Error::Other(x),
            ParseError::BadDistribution(x) => extendr_api::Error::Other(x)
        }   
    }
}

impl From<extendr_api::Error> for ParseError {
    fn from(value: extendr_api::Error) -> Self {
        ParseError::Rextendr(value)
    }
}

impl From<rand_distr::uniform::Error> for ParseError {
    fn from(value: rand_distr::uniform::Error) -> Self {
        ParseError::BadDistribution(value.to_string())
    }
}