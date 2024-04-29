use crate::impl_debug;
use crate::parse::nodes::GraphDisplay;
use std::fmt;
use std::fmt::{Debug, Formatter};

// Grammar for this file :
/*
<value_base> ::= T_BOOL | T_INT | T_STRING | T_FLOAT
<value> ::=
  | <value_base>
  | <exp_base>
<take_prio> ::=
  T_LEFT_P <exp> T_RIGHT_P
  | <value>
<tp> ::=
  (T_PLUS | T_MINUS | T_NOT) <tp>
  | <take_prio>
<mult> ::= T_MULT <tp1>
<div> ::= T_DIV <tp1>
<md> ::= <mult> | <div>
<tp1> ::= <tp> (<md> |)
<add> ::= T_ADD <tp2>
<sub> ::= T_SUB <tp2>
<as> ::= <add> | <sub>
<tp2> ::= <tp1> (<as> |)
<eq> ::= T_EQUAL <tp3>
<not_eq> ::= T_NOT_EQUAL <tp3>
<eq_not> ::= <eq> | <not_eq>
<tp3> ::= <tp2> (<eq_not> |)
<and> ::= T_AND <tp4>
<tp4> ::= <tp3> (<and> |)
<or> ::= T_OR <tp5>
<tp5> ::= <tp4> (<or> |)
<tp_last> ::= <tp5>
<no_value> ::= (<md> |) (<as> |) (<eq_not> |) (<and> |) (<or> |)
 */

// -----------------
// --- ValueBase ---
// -----------------

#[derive(PartialEq)]
pub enum ValueBase {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
}

impl GraphDisplay for ValueBase {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        match self {
            ValueBase::Bool(value) => {
                graph.push_str(&format!("\nsubgraph_ValueBase_{}[ValueBase Bool {}]\nend", id, value));
            }
            ValueBase::Int(value) => {
                graph.push_str(&format!("\nsubgraph_ValueBase_{}[ValueBase Int {}]\nend", id, value));
            }
            ValueBase::Float(value) => {
                graph.push_str(&format!("\nsubgraph_ValueBase_{}[ValueBase Float {}]\nend", id, value));
            }
            ValueBase::String(value) => {
                graph.push_str(&format!("\nsubgraph_ValueBase_{}[ValueBase String {}]\nend", id, value));
            }
        }
        *id += 1;
    }
}

impl_debug!(ValueBase);

// -------------
// --- Value ---
// -------------

#[derive(PartialEq)]
pub enum ValueNode {
    ValueBase(ValueBase),
    ExpBase, // TODO
}

impl GraphDisplay for ValueNode {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!("\nsubgraph_ValueNode_{}[ValueNode]", id));
        *id += 1;
        match self {
            ValueNode::ValueBase(value) => {
                value.graph_display(graph, id);
            }
            ValueNode::ExpBase => {
                // TODO
            }
        }
        graph.push_str(&"\nend".to_string());
    }
}

impl_debug!(ValueNode);

// ------------
// --- TODO ---
// ------------