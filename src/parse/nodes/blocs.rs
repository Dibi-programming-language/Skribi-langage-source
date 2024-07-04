use std::collections::VecDeque;

use crate::impl_debug;
use crate::parse::nodes::expressions::StaL;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{CustomError, ResultOption};
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
    sta_l: StaL,
}

impl GraphDisplay for KStart {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph KStart_{}[KStart]", id));
        *id += 1;
        if let Some(name) = &self.name {
            name.graph_display(graph, id);
        }
        self.sta_l.graph_display(graph, id);
        graph.push_str("\nend");
    }
}

impl_debug!(KStart);

impl KStart {
    pub fn new(name: Option<KName>, sta_l: StaL) -> Self {
        Self { name, sta_l }
    }

    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <k_start> ::= <sta_l> | <k_name> <sta_l>
        if let Some(sta_l) = StaL::parse(tokens)? {
            Ok(Some(KStart::new(None, sta_l)))
        } else if let Some(name) = KName::parse(tokens)? {
            if let Some(sta_l) = StaL::parse(tokens)? {
                Ok(Some(KStart::new(Some(name), sta_l)))
            } else {
                Err(CustomError::UnexpectedToken("Expected a sta_l".to_string()))
            }
        } else {
            Err(CustomError::UnexpectedToken(
                "Expected a sta_l or a k_name".to_string(),
            ))
        }
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

impl Kodi {
    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <kodi> ::= kodi <k_start>
        if let Some(Token::KeywordSimpleScope) = tokens.pop_front() {
            if let Some(start) = KStart::parse(tokens)? {
                Ok(Some(Kodi { start }))
            } else {
                Err(CustomError::UnexpectedToken(
                    "Expected a k_start".to_string(),
                ))
            }
        } else {
            Ok(None)
        }
    }
}

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

impl Biuli {
    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <biuli> ::= biuli <k_start>
        if let Some(Token::KeywordBubbleScope) = tokens.pop_front() {
            if let Some(start) = KStart::parse(tokens)? {
                Ok(Some(Biuli { start }))
            } else {
                Err(CustomError::UnexpectedToken(
                    "Expected a k_start".to_string(),
                ))
            }
        } else {
            Ok(None)
        }
    }
}

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

impl Spoki {
    pub fn parse(tokens: &mut VecDeque<Token>) -> ResultOption<Self> {
        // <spoki> ::= spoki <k_start>
        if let Some(Token::KeywordUnusedScope) = tokens.pop_front() {
            if let Some(start) = KStart::parse(tokens)? {
                Ok(Some(Spoki { start }))
            } else {
                Err(CustomError::UnexpectedToken(
                    "Expected a k_start".to_string(),
                ))
            }
        } else {
            Ok(None)
        }
    }
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
        if let Some(sta_l) = StaL::parse(tokens)? {
            Ok(Some(ScopeBase::StaL(sta_l)))
        } else if let Some(kodi) = Kodi::parse(tokens)? {
            Ok(Some(ScopeBase::Kodi(kodi)))
        } else if let Some(spoki) = Spoki::parse(tokens)? {
            Ok(Some(ScopeBase::Spoki(spoki)))
        } else if let Some(biuli) = Biuli::parse(tokens)? {
            Ok(Some(ScopeBase::Biuli(biuli)))
        } else {
            Ok(None)
        }
    }
}

// -------------
// --- Scope ---
// -------------

#[derive(PartialEq)]
pub(crate) enum Scope {
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
        if let Some(scope_base) = ScopeBase::parse(tokens)? {
            Ok(Some(Scope::ScopeBase(scope_base)))
        } else if let Some(sta_l) = StaL::parse(tokens)? {
            Ok(Some(Scope::Sta(sta_l)))
        } else {
            Ok(None)
        }
    }
}
