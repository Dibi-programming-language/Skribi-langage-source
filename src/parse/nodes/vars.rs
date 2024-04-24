use crate::parse::nodes::expressions::Exp;

pub struct Type {
    type_def: String, // TODO: Définir le type de type_def
}

pub struct Vd {
    type_: Type,
    identifier: String,
    exp: Box<Exp>,
}

pub struct GlobalVar {
    vd: Vd,
}

pub struct PrivateVar {
    vd: Vd,
}

pub struct ConstVar {
    private_var: Option<PrivateVar>,
    global_var: Option<GlobalVar>,
    vd: Option<Vd>,
}

pub enum VarDec {
    ConstVar(ConstVar),
    PrivateVar(PrivateVar),
    GlobalVar(GlobalVar),
    Vd(Vd),
}

pub struct VarMod {
    exp: Exp,
}