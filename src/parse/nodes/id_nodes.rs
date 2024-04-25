use crate::parse::nodes::GraphDisplay;
use crate::tokens::Token;

pub struct TupleNode {
    // TODO: définir les champs du tuple ici
}

#[derive(PartialEq)]
pub struct CGet {
    pub(crate) name: String,
}

impl GraphDisplay for CGet {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_CGet_{}[CGet {}]\nend", id, self.name));
        *id += 1;
    }
}

pub struct IdGet {
    identifier: String,
    tuple: Option<TupleNode>,
    op_in: Box<OpIn>,
}

pub enum OpIn {
    In {
        id_get: Option<IdGet>,
        cget: Option<CGet>,
    },
    Empty,
}

fn parse_tuple(tokens: &mut Vec<Token>) -> Option<TupleNode> {
    // TODO: implémenter cette fonction
    None
}

fn is_type_def(identifier: &str) -> bool {
    // TODO: implémenter cette fonction
    false
}

fn parse_cget(tokens: &mut Vec<Token>) -> Option<CGet> {
    for token in tokens.iter() {
        if let Token::Identifier(identifier) = token {
            if is_type_def(identifier) {
                return Some(CGet { name: identifier.clone() });
            }
        }
    }
    None
}

fn parse_op_in(tokens: &mut Vec<Token>) -> Option<OpIn> {
    for token in tokens.iter() {
        if let Token::Inside = token {
            return Some(OpIn::In {
                id_get: parse_id_get(tokens),
                cget: parse_cget(tokens),
            });
        }
    }
    Some(OpIn::Empty)
}

fn parse_id_get(tokens: &mut Vec<Token>) -> Option<IdGet> {
    for token in tokens.iter() {
        if let Token::Identifier(identifier) = token {
            return Some(IdGet {
                identifier: identifier.clone(),
                tuple: parse_tuple(tokens),
                op_in: Box::new(parse_op_in(tokens)?),
            });
        }
    }
    None
}
