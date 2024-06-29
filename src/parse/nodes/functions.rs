
// Grammar of this file :
// <fct_dec> ::= ums T_IDENTIFIER <tuple> <scope>

// --------------
// --- FctDec ---
// --------------

use std::fmt::Debug;
use crate::parse::nodes::GraphDisplay;
use crate::parse::nodes::id_nodes::TupleNode;
use crate::parse::nodes::if_else::Scope;

struct FctDec {
    identifier: String,
    tuple: TupleNode,
    scope: Scope,
}

impl GraphDisplay for FctDec {
    fn graph_display(&self, graph: &mut String, id: &mut usize) {
        graph.push_str(&format!(
            "\nsubgraph FctDec_{}[FctDec {}]",
            id, self.identifier
        ));
        *id += 1;
        self.tuple.graph_display(graph, id);
        self.scope.graph_display(graph, id);
        graph.push_str("\nend");
    }
}