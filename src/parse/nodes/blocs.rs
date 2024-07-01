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

impl GraphDisplay for KStart {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph KStart_{}[KStart]", id));
        *id += 1;
        if let Some(name) = &self.name {
            name.graph_display(graph, id);
        }
        for sta_l in &self.sta_l {
            sta_l.graph_display(graph, id);
        }
        graph.push_str("\nend");
    }
}

impl_debug!(KStart);

impl KStart {
    pub fn new(name: Option<KName>, sta_l: Vec<StaL>) -> Self {
        Self { name, sta_l }
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <k_start> ::= <sta_l> | <k_name> <sta_l>
        // TODO
        Ok(None)
    }
}

// ------------
// --- Kodi ---
// ------------

#[derive(PartialEq)]
pub struct Kodi {
    start: KStart,
}

impl GraphDisplay for Kodi {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Kodi_{}[Kodi]", id));
        *id += 1;
        self.start.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(Kodi);

// -------------
// --- Biuli ---
// -------------

#[derive(PartialEq)]
pub struct Biuli {
    start: KStart,
}

impl GraphDisplay for Biuli {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Biuli_{}[Biuli]", id));
        *id += 1;
        self.start.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(Biuli);

// -------------
// --- Spoki ---
// -------------

#[derive(PartialEq)]
pub struct Spoki {
    start: KStart,
}

impl GraphDisplay for Spoki {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Spoki_{}[Spoki]", id));
        *id += 1;
        self.start.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(Spoki);

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

impl GraphDisplay for ScopeBase {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph ScopeBase_{}[ScopeBase]", id));
        *id += 1;
        match self {
            ScopeBase::StaL(sta_l) => sta_l.graph_display(graph, id),
            ScopeBase::Kodi(kodi) => kodi.graph_display(graph, id),
            ScopeBase::Spoki(spoki) => spoki.graph_display(graph, id),
            ScopeBase::Biuli(biuli) => biuli.graph_display(graph, id),
        }
        graph.push_str("\nend");
    }
}

impl_debug!(ScopeBase);

impl ScopeBase {
    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <scope_base> ::= <sta_l> | <kodi> | <spoki> | <biuli>
        // TODO
        Ok(None)
    }
}

// -------------
// --- Scope ---
// -------------

#[derive(PartialEq)]
pub enum Scope {
    ScopeBase(ScopeBase),
    Sta(StaL),
}

impl GraphDisplay for Scope {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph Scope_{}[Scope]", id));
        *id += 1;
        match self {
            Scope::ScopeBase(scope_base) => scope_base.graph_display(graph, id),
            Scope::Sta(sta_l) => sta_l.graph_display(graph, id),
        }
        graph.push_str("\nend");
    }
}

impl_debug!(Scope);

impl Scope {
    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <scope> ::= <scope_base> | <sta>
        // TODO
        Ok(None)
    }
}
