use std::collections::VecDeque;

use crate::impl_debug;
use crate::parse::nodes::expressions::StaL;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::ResultOption;
use crate::tokens::Token;

// Grammar of this file :
// <k_name> ::=
//   T_IDENTIFIER
//   | {(* - T_LEFT_E)}
// <k_start> ::= <sta_l> | <k_name> <sta_l>
// <kodi> ::= kodi <k_start>
// <biuli> ::= biuli <k_start>
// <spoki> ::= spoki <k_start>
// <scope_base> ::=
//   <sta_l>
//   | <kodi>
//   | <spoki>
//   | <biuli>
// <scope> ::= <scope_base> | <sta>

// -------------
// --- KName ---
// -------------

#[derive(PartialEq)]
pub struct KName {
    name: String,
}

impl GraphDisplay for KName {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!(
            "\nsubgraph KName_{}[KName {}]\nend",
            id, self.name
        ));
        *id += 1;
    }
}

impl_debug!(KName);

impl KName {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    
    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<KName> {
        // <k_name> ::= T_IDENTIFIER | {(* - T_LEFT_E)}
        if let Some(Token::Identifier(name)) = tokens.pop_front() {
            Ok(Some(KName::new(name)))
        } else {
            // While the token is not a left bracket, we consume it and add it to the name
            let mut name = String::new();
            while let Some(token) = tokens.pop_front() {
                match token {
                    Token::LeftBrace => break,
                    _ => name.push_str(&format!("{} ", token)),
                }
            }
            Ok(Some(KName::new(name)))
        }
    }
}

// --------------
// --- KStart ---
// --------------

#[derive(PartialEq)]
pub struct KStart {
    name: Option<KName>,
    sta_l: Vec<StaL>,
}

// ------------
// --- Kodi ---
// ------------

#[derive(PartialEq)]
pub struct Kodi {
    start: KStart,
}

// -------------
// --- Biuli ---
// -------------

#[derive(PartialEq)]
pub struct Biuli {
    start: KStart,
}

// -------------
// --- Spoki ---
// -------------

#[derive(PartialEq)]
pub struct Spoki {
    start: KStart,
}

// -----------------
// --- ScopeBase ---
// -----------------

#[derive(PartialEq)]
pub enum ScopeBase {
    StaL(StaL),
    Kodi(Kodi),
    Spoki(Spoki),
    Biuli(Biuli),
}
