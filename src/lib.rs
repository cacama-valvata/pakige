// Pakige
use std::fmt;

pub mod deb;
pub mod rpm;

#[derive(Clone, Copy)]
pub enum VerOp
{
    Gt,
    GtEq,
    Eq,
    LtEq,
    Lt
}

#[derive(Debug)]
pub enum PakigeParseError
{
    EmptyInput,
    MissingMandatoryField,
    InvalidFormat,
    InvalidValue,
    DuplicateField
}

impl fmt::Display for PakigeParseError 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        match self {
            PakigeParseError::EmptyInput => write!(f, "Input is empty."),
            PakigeParseError::MissingMandatoryField => write!(f, "Input is missing a mandatory field."),
            PakigeParseError::InvalidFormat => write!(f, "Input is not in a valid format."),
            PakigeParseError::InvalidValue => write!(f, "A given field has an invalid value."),
            PakigeParseError::DuplicateField => write!(f, "A given field was present twice in the stanza.")
        }
    }
}

// TODO: handle different kinds of errors from deb_version?
impl From<deb_version7::Error> for PakigeParseError
{
    fn from (error: deb_version7::Error) -> PakigeParseError
    {
        return PakigeParseError::InvalidFormat;
    }
}

impl std::error::Error for PakigeParseError {}

pub trait Pakige
{
    fn ver_compare ();
}









// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
