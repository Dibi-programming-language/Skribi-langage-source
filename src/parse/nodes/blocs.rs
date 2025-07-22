use std::collections::VecDeque;

use crate::execute::{Evaluate, OperationContext, OperationO};
use crate::parse::nodes::expressions::StaL;
use crate::parse::nodes::GraphDisplay;
use crate::skr_errors::{ParsingError, ResultOption};
use crate::tokens::{Token, TokenContainer};
use crate::{impl_debug, some_token};

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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph KName_{}[KName {}]\nend",
            "",
            id,
            self.name,
            indent = indent
        ));
        *id += 1;
    }
}

impl_debug!(KName);

impl KName {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<KName> {
        // <k_name> ::= T_IDENTIFIER | {(* - T_LEFT_E)}
        if let some_token!(Token::Identifier(_)) = tokens.front() {
            if let some_token!(Token::Identifier(name)) = tokens.pop_front() {
                Ok(Some(KName::new(name)))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Expected an identifier".to_string(),
                ))
            }
        } else {
            // While the token is not a left bracket, we consume it and add it to the name
            let mut name = String::new();
            while let some_token!(token) = tokens.pop_front() {
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph KStart_{}[KStart]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        if let Some(name) = &self.name {
            name.graph_display(graph, id, indent + 2);
        }
        self.sta_l.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(KStart);

impl KStart {
    pub fn new(name: Option<KName>, sta_l: StaL) -> Self {
        Self { name, sta_l }
    }

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <k_start> ::= <sta_l> | <k_name> <sta_l>
        if let Some(sta_l) = StaL::parse(tokens)? {
            Ok(Some(KStart::new(None, sta_l)))
        } else if let Some(name) = KName::parse(tokens)? {
            if let Some(sta_l) = StaL::parse(tokens)? {
                Ok(Some(KStart::new(Some(name), sta_l)))
            } else {
                Err(ParsingError::UnexpectedToken(
                    "Expected a sta_l".to_string(),
                ))
            }
        } else {
            Err(ParsingError::UnexpectedToken(
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph Kodi_{}[Kodi]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        self.start.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(Kodi);

impl Kodi {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <kodi> ::= kodi <k_start>
        if let some_token!(Token::KeywordSimpleScope) = tokens.front() {
            tokens.pop_front();
            if let Some(start) = KStart::parse(tokens)? {
                Ok(Some(Kodi { start }))
            } else {
                Err(ParsingError::UnexpectedToken(
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph Biuli_{}[Biuli]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        self.start.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(Biuli);

impl Biuli {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <biuli> ::= biuli <k_start>
        if let some_token!(Token::KeywordBubbleScope) = tokens.front() {
            tokens.pop_front();
            if let Some(start) = KStart::parse(tokens)? {
                Ok(Some(Biuli { start }))
            } else {
                Err(ParsingError::UnexpectedToken(
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph Spoki_{}[Spoki]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        self.start.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(Spoki);

impl Spoki {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
        // <spoki> ::= spoki <k_start>
        if let some_token!(Token::KeywordUnusedScope) = tokens.front() {
            tokens.pop_front();
            if let Some(start) = KStart::parse(tokens)? {
                Ok(Some(Spoki { start }))
            } else {
                Err(ParsingError::UnexpectedToken(
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph ScopeBase_{}[ScopeBase]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        match self {
            ScopeBase::StaL(sta_l) => sta_l.graph_display(graph, id, indent + 2),
            ScopeBase::Kodi(kodi) => kodi.graph_display(graph, id, indent + 2),
            ScopeBase::Spoki(spoki) => spoki.graph_display(graph, id, indent + 2),
            ScopeBase::Biuli(biuli) => biuli.graph_display(graph, id, indent + 2),
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(ScopeBase);

impl ScopeBase {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
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

impl Evaluate for ScopeBase {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match self {
            Self::StaL(stal) => stal.evaluate(operation_context),
            Self::Kodi(_kodi) => todo!(),
            Self::Spoki(_spoki) => todo!(),
            Self::Biuli(_biuli) => todo!(),
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
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:indent$}subgraph Scope_{}[Scope]",
            "",
            id,
            indent = indent
        ));
        *id += 1;
        match self {
            Scope::ScopeBase(scope_base) => scope_base.graph_display(graph, id, indent + 2),
            Scope::Sta(sta_l) => sta_l.graph_display(graph, id, indent + 2),
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(Scope);

impl Scope {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Self> {
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

impl Evaluate for Scope {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match self {
            Self::Sta(stal) => stal.evaluate(operation_context),
            Self::ScopeBase(scope_base) => scope_base.evaluate(operation_context),
        }
    }
}
