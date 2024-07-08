// Pakige

pub mod deb;
pub mod rpm;

pub enum VerOp
{
    Gt,
    GtEq,
    Eq,
    LtEq,
    Lt
}

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
