use crate::parse::Scope;

pub struct ClassDec {
    identifier: String,
    scope: Box<Scope>,
}