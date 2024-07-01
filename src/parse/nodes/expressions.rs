use crate::impl_debug;
use crate::parse::nodes::blocs::ScopeBase;
use crate::parse::nodes::functions::FctDec;
use crate::parse::nodes::id_nodes::{parse_id_get, parse_id_set, IdGet, IdSet};
use crate::parse::nodes::if_else::Cond;
use crate::parse::nodes::operations::{NoValue, TPLast};
use crate::parse::nodes::vars::{VarDec, VarMod};
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{CustomError, OptionResult, ResultOption};
use crate::tokens::{SpaceTypes, Token};
use std::collections::VecDeque;

// Grammar of this file :
// <nat_call_in> ::= T_IDENTIFIER ("\n" | <nat_call_in>)
// <nat_call> ::= T_NAT_CALL <nat_call_in>
// <id_use> ::=
//   <id_set> (<var_mod> |)
//   | <id_get>
// <id_use_v> ::= <id_use> (<no_value> |)
// <exp_base> ::=
//   <id_use>
//   | <var_dec>
//   | <cond>
//   | <scope_base>
//   | <fct_dec>
//   | T_LEFT_P <exp> T_RIGHT_P
// <exp_tp> ::=
//   <exp_base>
//   | <id_use_v>
// <exp> ::=
//   <exp_tp>
//   | <tp_last>
// <return> ::= ei <exp>
// <sta> ::= <return> | <exp>
// <sta_l> ::= T_LEFT_E {<sta>} T_RIGHT_E

// -----------------
// --- NatCallIn ---
// -----------------

#[derive(PartialEq)]
struct NatCallIn {
    identifier: String,
    nat_call_in: Option<Box<NatCallIn>>,
}

impl GraphDisplay for NatCallIn {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!(
            "\nsubgraph NatCallIn_{}[NatCallIn {}]",
            id, self.identifier
        ));
        *id += 1;
        if let Some(nat_call_in) = &self.nat_call_in {
            nat_call_in.graph_display(graph, id);
        }
        graph.push_str("\nend");
    }
}

impl_debug!(NatCallIn);

impl NatCallIn {
    fn new(identifier: String, nat_call_in: Option<NatCallIn>) -> Self {
        Self {
            identifier,
            nat_call_in: nat_call_in.map(Box::new),
        }
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<NatCallIn> {
        // <nat_call_in> ::= T_IDENTIFIER ("\n" | <nat_call_in>)
        if let Some(Token::Identifier(_)) = tokens.front() {
            if let Some(Token::Identifier(identifier)) = tokens.pop_front() {
                if let Some(Token::Space(SpaceTypes::NewLine)) = tokens.front() {
                    tokens.pop_front();
                    Ok(Some(NatCallIn::new(identifier, None)))
                } else {
                    let nat_call_in = NatCallIn::parse(tokens)?;
                    match nat_call_in {
                        Some(nat_call_in) => {
                            Ok(Some(NatCallIn::new(identifier, Some(nat_call_in))))
                        }
                        None => Err(CustomError::UnexpectedToken(
                            "Expected a new line or a nat_call_in".to_string(),
                        )),
                    }
                }
            } else {
                Err(CustomError::UnexpectedToken(
                    "Had an identifier, but couldn't get it".to_string(),
                ))
            }
        } else {
            Ok(None)
        }
    }
}

// ---------------
// --- NatCall ---
// ---------------

#[derive(PartialEq)]
struct NatCall {
    nat_call_in: NatCallIn,
}

impl GraphDisplay for NatCall {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph NatCall_{}[NatCall]", id));
        *id += 1;
        self.nat_call_in.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(NatCall);

impl NatCall {
    fn new(nat_call_in: NatCallIn) -> Self {
        Self { nat_call_in }
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<NatCall> {
        // <nat_call> ::= T_NAT_CALL <nat_call_in>
        if let Some(Token::NatCall) = tokens.front() {
            tokens.pop_front();
            if let Some(nat_call_in) = NatCallIn::parse(tokens)? {
                Ok(Some(NatCall::new(nat_call_in)))
            } else {
                Err(CustomError::UnexpectedToken(
                    "Expected a nat_call_in".to_string(),
                ))
            }
        } else {
            Ok(None)
        }
    }
}

// -------------
// --- IdUse ---
// -------------

#[derive(PartialEq)]
pub struct IdUse {
    id_set: IdSet,
    var_mod: Option<Box<VarMod>>,
    id_get: Option<IdGet>,
}

impl GraphDisplay for IdUse {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph IdUse_{}[IdUse]", id));
        *id += 1;
        self.id_set.graph_display(graph, id);
        if let Some(var_mod) = &self.var_mod {
            var_mod.graph_display(graph, id);
        }
        if let Some(id_get) = &self.id_get {
            id_get.graph_display(graph, id);
        }
        graph.push_str("\nend");
    }
}

impl_debug!(IdUse);

impl IdUse {
    fn new(id_set: IdSet, var_mod: Option<VarMod>, id_get: Option<IdGet>) -> Self {
        Self {
            id_set,
            var_mod: var_mod.map(Box::new),
            id_get,
        }
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<IdUse> {
        // <id_use> ::=
        //   <id_set> (<var_mod> |)
        //   | <id_get>
        match parse_id_set(tokens) {
            Some(Ok(id_set)) => {
                if let Some(var_mod) = VarMod::parse(tokens)? {
                    Ok(Some(IdUse::new(id_set, Some(var_mod), None)))
                } else if let Some(id_get) = parse_id_get(tokens) {
                    Ok(Some(IdUse::new(id_set, None, Some(id_get?))))
                } else {
                    Ok(Some(IdUse::new(id_set, None, None)))
                }
            }
            Some(Err(err)) => Err(err),
            None => Ok(None),
        }
    }
}

// --------------
// --- IdUseV ---
// --------------

#[derive(PartialEq)]
pub struct IdUseV {
    id_use: IdUse,
    no_value: Option<NoValue>,
}

impl GraphDisplay for IdUseV {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph IdUseV_{}[IdUseV]", id));
        *id += 1;
        self.id_use.graph_display(graph, id);
        if let Some(no_value) = &self.no_value {
            no_value.graph_display(graph, id);
        }
        graph.push_str("\nend");
    }
}

impl_debug!(IdUseV);

impl IdUseV {
    fn new(id_use: IdUse, no_value: Option<NoValue>) -> Self {
        Self { id_use, no_value }
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<IdUseV> {
        // <id_use_v> ::= <id_use> (<no_value> |)
        if let Some(id_use) = IdUse::parse(tokens)? {
            Ok(Some(IdUseV::new(id_use, Some(NoValue::parse(tokens)?))))
        } else {
            Ok(None)
        }
    }
}

// ---------------
// --- ExpBase ---
// ---------------

/// Not yet implemented
#[derive(PartialEq)]
pub enum ExpBase {
    IdUse(Box<IdUse>),
    VarDec(Box<VarDec>),
    Cond(Box<Cond>),
    ScopeBase(Box<ScopeBase>),
    FctDec(Box<FctDec>),
    LeftP(Box<Exp>),
    RightP(Box<Exp>),
}

impl GraphDisplay for ExpBase {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph ExpBase_{}[ExpBase]", id));
        *id += 1;
        match self {
            ExpBase::IdUse(id_use) => id_use.graph_display(graph, id),
            ExpBase::VarDec(var_dec) => var_dec.graph_display(graph, id),
            ExpBase::Cond(cond) => cond.graph_display(graph, id),
            ExpBase::ScopeBase(scope_base) => scope_base.graph_display(graph, id),
            ExpBase::FctDec(fct_dec) => fct_dec.graph_display(graph, id),
            ExpBase::LeftP(exp) => exp.graph_display(graph, id),
            ExpBase::RightP(exp) => exp.graph_display(graph, id),
        }
        graph.push_str("\nend");
    }
}

impl_debug!(ExpBase);

impl ExpBase {
    fn new(id_use: IdUse) -> Self {
        Self::IdUse(Box::new(id_use))
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<ExpBase> {
        // <exp_base> ::=
        //   <id_use>
        //   | <var_dec>
        //   | <cond>
        //   | <scope_base>
        //   | <fct_dec>
        //   | T_LEFT_P <exp> T_RIGHT_P
        if let Some(id_use) = IdUse::parse(tokens)? {
            Ok(Some(ExpBase::new(id_use)))
        } else if let Some(var_dec) = VarDec::parse(tokens)? {
            Ok(Some(ExpBase::VarDec(Box::new(var_dec))))
        } else if let Some(cond) = Cond::parse(tokens)? {
            Ok(Some(ExpBase::Cond(Box::new(cond))))
        } else if let Some(scope_base) = ScopeBase::parse(tokens)? {
            Ok(Some(ExpBase::ScopeBase(Box::new(scope_base))))
        } else if let Some(fct_dec) = FctDec::parse(tokens)? {
            Ok(Some(ExpBase::FctDec(Box::new(fct_dec))))
        } else if let Some(Token::LeftParenthesis) = tokens.front() {
            tokens.pop_front();
            if let Some(exp) = Exp::parse(tokens)? {
                if let Some(Token::RightParenthesis) = tokens.pop_front() {
                    Ok(Some(ExpBase::RightP(Box::new(exp))))
                } else {
                    Err(CustomError::UnexpectedToken(
                        "Expected a right parenthesis".to_string(),
                    ))
                }
            } else {
                Err(CustomError::UnexpectedToken(
                    "Expected an expression".to_string(),
                ))
            }
        } else {
            Ok(None)
        }
    }
}

// -------------
// --- ExpTp ---
// -------------

/// Not yet implemented
#[derive(PartialEq)]
pub enum ExpTp {
    ExpBase(ExpBase),
    IdUseV(IdUseV),
}

impl GraphDisplay for ExpTp {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph ExpTp_{}[ExpTp]", id));
        *id += 1;
        match self {
            ExpTp::ExpBase(exp_base) => exp_base.graph_display(graph, id),
            ExpTp::IdUseV(id_use_v) => id_use_v.graph_display(graph, id),
        }
        graph.push_str("\nend");
    }
}

impl_debug!(ExpTp);

// -----------
// --- Exp ---
// -----------

/// Not yet implemented, but already used by some nodes that are dependent on it
#[derive(PartialEq)]
pub struct Exp {
    exp_tp: ExpTp,
    tp_last: TPLast, // TODO - define tp_last
}

impl GraphDisplay for Exp {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Exp_{}[Exp]", id));
        *id += 1;
        self.exp_tp.graph_display(graph, id);
        self.tp_last.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(Exp);

impl Exp {
    fn new(exp_tp: ExpTp, tp_last: TPLast) -> Self {
        Self { exp_tp, tp_last }
    }

    pub fn parse(_tokens: &mut VecDeque<Token>) -> ResultOption<Exp> {
        // TODO
        Ok(None)
    }
}

// --------------
// --- Return ---
// --------------

#[derive(PartialEq)]
pub struct Return {
    exp: Exp,
}

impl GraphDisplay for Return {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Return_{}[Return]", id));
        *id += 1;
        self.exp.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(Return);

// -----------
// --- Sta ---
// -----------

#[derive(PartialEq)]
pub enum Sta {
    Return(Return),
    Exp(Exp),
}

impl GraphDisplay for Sta {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Sta_{}[Sta]", id));
        *id += 1;
        match self {
            Sta::Return(return_node) => return_node.graph_display(graph, id),
            Sta::Exp(exp) => exp.graph_display(graph, id),
        }
        graph.push_str("\nend");
    }
}

impl_debug!(Sta);

// ------------
// --- StaL ---
// ------------

#[derive(PartialEq)]
pub struct StaL {
    sta_l: Vec<Sta>,
}

impl GraphDisplay for StaL {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph StaL_{}[StaL]", id));
        *id += 1;
        for sta in &self.sta_l {
            match sta {
                Sta::Return(return_node) => return_node.graph_display(graph, id),
                Sta::Exp(exp) => exp.graph_display(graph, id),
            }
        }
        graph.push_str("\nend");
    }
}

impl_debug!(StaL);
