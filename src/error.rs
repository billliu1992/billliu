use std::fmt;
use std::fmt::{Display, Formatter};
use std::error::Error;

pub type EmptyResult = Result<(), Box<dyn Error>>;

#[derive(Debug)]
pub struct SiteError {
    pub msg: String,
}
impl Error for SiteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl Display for SiteError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}