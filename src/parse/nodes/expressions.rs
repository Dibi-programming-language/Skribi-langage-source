use std::collections::VecDeque;
use std::io::stdin;

use crate::execute::int::InternalInt;
use crate::execute::IntType;
use crate::execute::OperationContext;
use crate::execute::OperationO;
use crate::execute::{Evaluate, EvaluateFromInput, Execute, ExecutionError, GeneralOutput};
use crate::parse::nodes::blocs::ScopeBase;
use crate::parse::nodes::functions::FctDec;
use crate::parse::nodes::id_nodes::{parse_op_in, OpIn, TupleNode};
use crate::parse::nodes::if_else::Cond;
use crate::parse::nodes::operations::{NoValueN, TakePriorityLast};
use crate::parse::nodes::vars::{VarDec, VarMod};
use crate::parse::nodes::{GraphDisplay, Parsable};
use crate::skr_errors::{CustomError, ResultOption};
use crate::tokens::{SpaceTypes, Token, TokenContainer};
use crate::{impl_debug, some_token};

// Grammar of this file :
// <nat_call_in> ::= T_IDENTIFIER ("\n" | <nat_call_in>)
// <nat_call> ::= T_NAT_CALL <nat_call_in>
// <id_use> ::= T_IDENTIFIER (
//     <tuple> <op_in>
//     | <op_in> <var_mod>
//     | <op_in>
//   )
// <id_use_v> ::= T_IDENTIFIER (
//     <tuple> <op_in> (<no_value> |)
//     | <op_in> (<no_value> | <var_mod> |)
//   )
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

/// `NatCallIn` represents an argument of a native call. It contains an identifier and an optional
/// [NatCallIn] to represent the next argument.
///
/// The identifier is the name of the variable that will be passed to the native function. The
/// [NatCallIn] is the next argument to pass to the native function, this is the tail of the list of
/// arguments.
#[derive(PartialEq)]
struct NatCallIn {
    identifier: String,
    nat_call_in: Option<Box<NatCallIn>>,
}

impl GraphDisplay for NatCallIn {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:dec$}subgraph NatCallIn_{}[NatCallIn {}]",
            "",
            id,
            self.identifier,
            dec = indent
        ));
        *id += 1;
        if let Some(nat_call_in) = &self.nat_call_in {
            nat_call_in.graph_display(graph, id, indent + 2);
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
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

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<NatCallIn> {
        // <nat_call_in> ::= T_IDENTIFIER ("\n" | <nat_call_in>)
        if let some_token!(Token::Identifier(identifier)) = tokens.front() {
            let identifier = identifier.to_string();
            let token_container = tokens.pop_front().unwrap();

            if let some_token!(Token::Space(SpaceTypes::NewLine)) = tokens.front() {
                tokens.pop_front();
                Ok(Some(NatCallIn::new(identifier, None)))
            } else {
                let nat_call_in = NatCallIn::parse(tokens)?;
                match nat_call_in {
                    Some(nat_call_in) => Ok(Some(NatCallIn::new(identifier, Some(nat_call_in)))),
                    None => Err(CustomError::UnexpectedToken(format!(
                        "Expected a new line or a nat_call_in (l{}:{})",
                        token_container.line, token_container.column
                    ))),
                }
            }
        } else {
            Ok(None)
        }
    }
}

// ---------------
// --- NatCall ---
// ---------------

/// `NatCall` represents a native call. It contains a [NatCallIn] to represent the first argument
/// and the chain of arguments. The first argument is the name of the native function to call.
#[derive(PartialEq)]
pub struct NatCall {
    nat_call_in: NatCallIn,
}

impl GraphDisplay for NatCall {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:dec$}subgraph NatCall_{}[NatCall]",
            "",
            id,
            dec = indent
        ));
        *id += 1;
        self.nat_call_in.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(NatCall);

impl NatCall {
    fn new(nat_call_in: NatCallIn) -> Self {
        Self { nat_call_in }
    }

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<NatCall> {
        // <nat_call> ::= T_NAT_CALL <nat_call_in>
        if let some_token!(Token::NatCall) = tokens.front() {
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

    fn print(&self, operation_context: &mut OperationContext) -> GeneralOutput {
        let mut current = &self.nat_call_in.nat_call_in;
        while let Some(content) = current {
            print!(
                "{}",
                operation_context.get_variable(&content.identifier, 0)?
            );
            current = &content.nat_call_in;
        }
        Ok(())
    }

    fn input(&self, operation_context: &mut OperationContext) -> GeneralOutput {
        let mut buffer = String::new();
        if stdin().read_line(&mut buffer).is_err() {
            Err(ExecutionError::cannot_read_input())
        } else {
            let mut iter = buffer.trim().split(" ");
            let mut current = &self.nat_call_in.nat_call_in;
            while let (Some(content), Some(str)) = (current, iter.next()) {
                let result = str.parse::<IntType>();
                if let Ok(number) = result {
                    operation_context.change_value(
                        &content.identifier,
                        InternalInt::new_boxed(number),
                        0,
                    )?;
                } else {
                    return Err(ExecutionError::wrong_input_type("int", str));
                }
                current = &content.nat_call_in;
            }
            if current.is_some() {
                Err(ExecutionError::wrong_number_of_inputs())
            } else {
                Ok(())
            }
        }
    }

    fn assert_equal(&self, operation_context: &mut OperationContext) -> GeneralOutput {
        let mut current = &self.nat_call_in.nat_call_in;
        if let Some(content) = current {
            let first_value = operation_context.get_variable(&content.identifier, 0)?;
            current = &content.nat_call_in;
            while let Some(content) = current {
                let value = operation_context.get_variable(&content.identifier, 0)?;
                if !value.basic_equal(&first_value, operation_context)? {
                    return Err(ExecutionError::assertion_error(first_value, value));
                }
                current = &content.nat_call_in;
            }
        }
        Ok(())
    }
}

impl Execute for NatCall {
    fn execute(&self, operation_context: &mut OperationContext) -> GeneralOutput {
        match &self.nat_call_in.identifier[..] {
            "print" => self.print(operation_context),
            "println" => {
                self.print(operation_context)?;
                println!();
                Ok(())
            }
            "read_int" => self.input(operation_context),
            "assert_eq" => self.assert_equal(operation_context),
            name => Err(ExecutionError::native_call_invalid(name)),
        }
    }
}

// -------------
// --- IdUse ---
// -------------

/// `InsideIdUse` represents the possible values that
/// can be inside an [IdUse].
/// It can be:
/// - a [TupleNode],
/// - a [VarMod],
/// - or nothing.
#[derive(PartialEq)]
pub(crate) enum InsideIdUse {
    /// Function call
    Tuple(TupleNode),
    /// Variable modification
    VarMod(VarMod),
    /// Just get the value
    Empty,
}

/// `IdUse` represents two types of expressions that have a link with identifiers.
/// It can be a function call with a [TupleNode],
/// or a variable usage (get / set with [VarMod]).
///
/// # Grammar
///
/// `<id_use> ::= T_IDENTIFIER (<tuple> <op_in> | <op_in> <var_mod> | <op_in>)`
///
/// See also [TupleNode], [OpIn] and [VarMod].
#[derive(PartialEq)]
pub struct IdUse {
    identifier: String,
    op_in: OpIn,
    inside_id_use: InsideIdUse,
}

impl GraphDisplay for IdUse {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:dec$}subgraph IdUse_{}[IdUse {}] dec={}",
            "",
            id,
            self.identifier,
            indent,
            dec = indent
        ));
        *id += 1;
        self.op_in.graph_display(graph, id, indent + 2);
        match &self.inside_id_use {
            InsideIdUse::Tuple(tuple) => tuple.graph_display(graph, id, indent + 2),
            InsideIdUse::VarMod(var_mod) => var_mod.graph_display(graph, id, indent + 2),
            InsideIdUse::Empty => {}
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(IdUse);

impl IdUse {
    pub(crate) fn new(identifier: String, op_in: OpIn, inside_id_use: InsideIdUse) -> Self {
        Self {
            identifier,
            op_in,
            inside_id_use,
        }
    }

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<IdUse> {
        // <id_use> ::= T_IDENTIFIER (
        //     <tuple> <op_in>
        //     | <op_in> <var_mod>
        //     | <op_in>
        //   )
        if let some_token!(Token::Identifier(_)) = tokens.front() {
            if let some_token!(Token::Identifier(identifier)) = tokens.pop_front() {
                if let Some(tuple) = TupleNode::parse(tokens)? {
                    let op_in = parse_op_in(tokens)?;
                    Ok(Some(IdUse::new(
                        identifier,
                        op_in,
                        InsideIdUse::Tuple(tuple),
                    )))
                } else {
                    let op_in = parse_op_in(tokens)?;
                    if let Some(var_mod) = VarMod::parse(tokens)? {
                        Ok(Some(IdUse::new(
                            identifier,
                            op_in,
                            InsideIdUse::VarMod(var_mod),
                        )))
                    } else {
                        Ok(Some(IdUse::new(identifier, op_in, InsideIdUse::Empty)))
                    }
                }
            } else {
                Err(CustomError::UnexpectedToken(
                    "Expected an identifier".to_string(),
                ))
            }
        } else {
            Ok(None)
        }
    }
}

impl Evaluate for IdUse {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match &self.inside_id_use {
            InsideIdUse::Empty => operation_context.get_variable(&self.identifier, 0),
            InsideIdUse::VarMod(modification) => {
                let value = modification.evaluate(operation_context)?;
                operation_context.change_value(&self.identifier, value.clone(), 0)?;
                Ok(value)
            }
            _ => todo!(),
        }
    }
}

// --------------
// --- IdUseV ---
// --------------

/// `InsideIdUseV` represents the possible values that can be inside an [IdUseV].
/// It can be a [TupleNode] (with an optional [NoValueN]),
/// a [VarMod], a [NoValueN], or nothing.
#[derive(PartialEq)]
pub(crate) enum InsideIdUseV {
    Tuple {
        tuple: TupleNode,
        no_value: Option<NoValueN>,
    },
    NoValue(NoValueN),
    VarMod(Box<VarMod>),
    Empty,
}

/// `IdUseV` works like an [IdUse] but can apply operations on the result of a get.
/// This means that it can be an identifier usage
/// on which we apply operations, or not.
/// We must notice that we cannot directly apply a [NoValueN]
/// to an [IdUse] because [NoValueN] has a higher priority than
/// [VarMod] and cannot be used with it.
///
/// # Grammar
///
/// ```
/// <id_use_v> ::= T_IDENTIFIER (
///     <tuple> <op_in> (<no_value> |)
///     | <op_in> (<no_value> | <var_mod> |)
/// )
/// ```
///
/// See also [TupleNode], [OpIn], [NoValueN] and [VarMod].
///
/// # Example
///
/// The expression `a + 1` is an identifier followed by an empty [OpIn] and the [NoValueN] `+ 1`.
///
/// See the test `test_simple_exp_id_use_v` in `src/tests/parse_tests/expressions_tests.rs` for an
/// example of parsing.
#[derive(PartialEq)]
pub struct IdUseV {
    identifier: String,
    op_in: OpIn,
    inside_id_use_v: InsideIdUseV,
}

impl GraphDisplay for IdUseV {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:dec$}subgraph IdUseV_{}[IdUseV {}]",
            "",
            id,
            self.identifier,
            dec = indent
        ));
        *id += 1;
        self.op_in.graph_display(graph, id, indent + 2);
        match &self.inside_id_use_v {
            InsideIdUseV::Tuple { tuple, no_value } => {
                tuple.graph_display(graph, id, indent + 2);
                if let Some(no_value) = no_value {
                    no_value.graph_display(graph, id, indent + 2);
                }
            }
            InsideIdUseV::NoValue(no_value) => no_value.graph_display(graph, id, indent + 2),
            InsideIdUseV::VarMod(var_mod) => var_mod.graph_display(graph, id, indent + 2),
            InsideIdUseV::Empty => {}
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(IdUseV);

impl IdUseV {
    pub(crate) fn new(identifier: String, op_in: OpIn, inside_id_use_v: InsideIdUseV) -> Self {
        Self {
            identifier,
            op_in,
            inside_id_use_v,
        }
    }

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<IdUseV> {
        // <id_use_v> ::= T_IDENTIFIER (
        //     <tuple> <op_in> (<no_value> |)
        //     | <op_in> (<no_value> | <var_mod> |)
        //   )
        if let some_token!(Token::Identifier(_)) = tokens.front() {
            if let some_token!(Token::Identifier(identifier)) = tokens.pop_front() {
                if let Some(tuple) = TupleNode::parse(tokens)? {
                    let op_in = parse_op_in(tokens)?;
                    Ok(Some(IdUseV::new(
                        identifier,
                        op_in,
                        InsideIdUseV::Tuple {
                            tuple,
                            no_value: NoValueN::parse(tokens)?,
                        },
                    )))
                } else {
                    let op_in = parse_op_in(tokens)?;
                    if let Some(no_value) = NoValueN::parse(tokens)? {
                        Ok(Some(IdUseV::new(
                            identifier,
                            op_in,
                            InsideIdUseV::NoValue(no_value),
                        )))
                    } else if let Some(var_mod) = VarMod::parse(tokens)? {
                        Ok(Some(IdUseV::new(
                            identifier,
                            op_in,
                            InsideIdUseV::VarMod(Box::new(var_mod)),
                        )))
                    } else {
                        Ok(Some(IdUseV::new(identifier, op_in, InsideIdUseV::Empty)))
                    }
                }
            } else {
                Err(CustomError::UnexpectedToken(
                    "Expected an identifier".to_string(),
                ))
            }
        } else {
            Ok(None)
        }
    }
}

impl Evaluate for IdUseV {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match &self.inside_id_use_v {
            InsideIdUseV::Empty => operation_context.get_variable(&self.identifier, 0),
            InsideIdUseV::VarMod(modification) => {
                let value = modification.evaluate(operation_context)?;
                operation_context.change_value(&self.identifier, value.clone(), 0)?;
                Ok(value)
            }
            InsideIdUseV::NoValue(nv) => {
                let left = operation_context.get_variable(&self.identifier, 0)?;
                nv.evaluate_from_input(operation_context, left)
            }
            _ => todo!(),
        }
    }
}

// ---------------
// --- ExpBase ---
// ---------------

/// `ExpBase` represents any expression node that has the priority
/// over many grammar rules with high priority, like operations.
#[derive(PartialEq)]
pub enum ExpBase {
    IdUse(Box<IdUse>),
    VarDec(Box<VarDec>),
    Cond(Box<Cond>),
    ScopeBase(Box<ScopeBase>),
    FctDec(Box<FctDec>),
    RightP(Box<Exp>),
}

impl GraphDisplay for ExpBase {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:dec$}subgraph ExpBase_{}[ExpBase]",
            "",
            id,
            dec = indent
        ));
        *id += 1;
        match self {
            ExpBase::IdUse(id_use) => id_use.graph_display(graph, id, indent + 2),
            ExpBase::VarDec(var_dec) => var_dec.graph_display(graph, id, indent + 2),
            ExpBase::Cond(cond) => cond.graph_display(graph, id, indent + 2),
            ExpBase::ScopeBase(scope_base) => scope_base.graph_display(graph, id, indent + 2),
            ExpBase::FctDec(fct_dec) => fct_dec.graph_display(graph, id, indent + 2),
            ExpBase::RightP(exp) => {
                graph.push_str(" with ()");
                exp.graph_display(graph, id, indent + 2)
            }
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(ExpBase);

impl ExpBase {
    fn new(id_use: IdUse) -> Self {
        Self::IdUse(Box::new(id_use))
    }

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<ExpBase> {
        // <exp_base> ::=
        //   <var_dec>
        //   | <id_use>
        //   | <cond>
        //   | <scope_base>
        //   | <fct_dec>
        //   | T_LEFT_P <exp> T_RIGHT_P
        if let Some(var_dec) = VarDec::parse(tokens)? {
            Ok(Some(ExpBase::VarDec(Box::new(var_dec))))
        } else if let Some(id_use) = IdUse::parse(tokens)? {
            Ok(Some(ExpBase::new(id_use)))
        } else if let Some(cond) = Cond::parse(tokens)? {
            Ok(Some(ExpBase::Cond(Box::new(cond))))
        } else if let Some(scope_base) = ScopeBase::parse(tokens)? {
            Ok(Some(ExpBase::ScopeBase(Box::new(scope_base))))
        } else if let Some(fct_dec) = FctDec::parse(tokens)? {
            Ok(Some(ExpBase::FctDec(Box::new(fct_dec))))
        } else if let some_token!(Token::LeftParenthesis) = tokens.front() {
            tokens.pop_front();
            if let Some(exp) = Exp::parse(tokens)? {
                if let some_token!(Token::RightParenthesis) = tokens.pop_front() {
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

impl Evaluate for ExpBase {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match self {
            Self::IdUse(id_use) => id_use.evaluate(operation_context),
            Self::Cond(_cond) => todo!(),
            Self::VarDec(var_dec) => var_dec.evaluate(operation_context),
            Self::RightP(rightp) => rightp.evaluate(operation_context),
            Self::FctDec(_fct_dec) => todo!(),
            Self::ScopeBase(_scope_base) => todo!(),
        }
    }
}

// -------------
// --- ExpTp ---
// -------------

/// `ExpTp` represents the second level of high priority expressions.
/// This contains [ExpBase] and [IdUseV].
/// For now, it is only used to represent the [IdUseV].
#[derive(PartialEq)]
pub enum ExpTp {
    ExpBase(ExpBase),
    IdUseV(IdUseV),
}

impl GraphDisplay for ExpTp {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:dec$}subgraph ExpTp_{}[ExpTp]",
            "",
            id,
            dec = indent
        ));
        *id += 1;
        match self {
            ExpTp::ExpBase(exp_base) => exp_base.graph_display(graph, id, indent + 2),
            ExpTp::IdUseV(id_use_v) => id_use_v.graph_display(graph, id, indent + 2),
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(ExpTp);

impl ExpTp {
    fn new(exp_base: ExpBase) -> Self {
        Self::ExpBase(exp_base)
    }

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<ExpTp> {
        // <exp_tp> ::=
        //   <exp_base>
        //   | <id_use_v>
        if let Some(id_use_v) = IdUseV::parse(tokens)? {
            Ok(Some(ExpTp::IdUseV(id_use_v)))
        } else if let Some(exp_base) = ExpBase::parse(tokens)? {
            Ok(Some(ExpTp::new(exp_base)))
        } else {
            Ok(None)
        }
    }
}

impl Evaluate for ExpTp {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match self {
            Self::IdUseV(id_use_v) => id_use_v.evaluate(operation_context),
            Self::ExpBase(exp_base) => exp_base.evaluate(operation_context),
        }
    }
}

// -----------
// --- Exp ---
// -----------

/// `Exp` represents any expression with low priority.
/// It might be between parentheses to work.
/// It contains [ExpTp] or [TakePriorityLast].
/// [TakePriorityLast] represents any chain of operations,
/// and [ExpTp] a high priority expression.
#[derive(PartialEq)]
pub enum Exp {
    ExpTp(ExpTp),
    TPLast(TakePriorityLast),
}

impl GraphDisplay for Exp {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:dec$}subgraph Exp_{}[Exp]",
            "",
            id,
            dec = indent
        ));
        *id += 1;
        match self {
            Exp::ExpTp(exp_tp) => exp_tp.graph_display(graph, id, indent + 2),
            Exp::TPLast(tp_last) => tp_last.graph_display(graph, id, indent + 2),
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(Exp);

impl Exp {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Exp> {
        // <exp> ::=
        //   <exp_tp>
        //   | <tp_last>
        if let Some(tp_last) = TakePriorityLast::parse(tokens)? {
            Ok(Some(Exp::TPLast(tp_last)))
        } else if let Some(exp_tp) = ExpTp::parse(tokens)? {
            Ok(Some(Exp::ExpTp(exp_tp)))
        } else {
            Ok(None)
        }
    }
}

impl Evaluate for Exp {
    fn evaluate(&self, operation_context: &mut OperationContext) -> OperationO {
        match self {
            Exp::ExpTp(exp_tp) => exp_tp.evaluate(operation_context),
            Exp::TPLast(tp_last) => tp_last.evaluate(operation_context),
        }
    }
}

// --------------
// --- Return ---
// --------------

/// `Return` represents a return statement. It contains an [Exp] that will be returned by the
/// function.
#[derive(PartialEq)]
pub struct Return {
    exp: Exp,
}

impl GraphDisplay for Return {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:dec$}subgraph Return_{}[Return]",
            "",
            id,
            dec = indent
        ));
        *id += 1;
        self.exp.graph_display(graph, id, indent + 2);
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(Return);

impl Return {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Return> {
        // <return> ::= ei <exp>
        if let some_token!(Token::KeywordReturn) = tokens.front() {
            tokens.pop_front();
            if let Some(exp) = Exp::parse(tokens)? {
                Ok(Some(Return { exp }))
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

// -----------
// --- Sta ---
// -----------

/// `Sta` represents a statement. It can be a [Return] or an [Exp].
#[derive(PartialEq)]
pub enum Sta {
    Return(Return),
    Exp(Exp),
    NatCall(NatCall),
}

impl GraphDisplay for Sta {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:dec$}subgraph Sta_{}[Sta]",
            "",
            id,
            dec = indent
        ));
        *id += 1;
        match self {
            Sta::Return(return_node) => return_node.graph_display(graph, id, indent + 2),
            Sta::Exp(exp) => exp.graph_display(graph, id, indent + 2),
            Sta::NatCall(nat_call) => nat_call.graph_display(graph, id, indent + 2),
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(Sta);

impl Sta {
    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<Sta> {
        // <sta> ::=
        //     <return>
        //     | <nat_call>
        //     | <exp>
        if let Some(return_node) = Return::parse(tokens)? {
            Ok(Some(Sta::Return(return_node)))
        } else if let Some(nat_call) = NatCall::parse(tokens)? {
            Ok(Some(Sta::NatCall(nat_call)))
        } else if let Some(exp) = Exp::parse(tokens)? {
            Ok(Some(Sta::Exp(exp)))
        } else {
            Ok(None)
        }
    }
}

impl Execute for Sta {
    fn execute(&self, operation_context: &mut OperationContext) -> GeneralOutput {
        match self {
            Sta::Return(_return_node) => {
                todo!()
            }
            Sta::NatCall(nat_call) => {
                nat_call.execute(operation_context)?;
            }
            Sta::Exp(exp) => {
                exp.evaluate(operation_context)?;
            }
        }
        Ok(())
    }
}

// ------------
// --- StaL ---
// ------------

/// `StaL` is the base of a scope. It contains a list of [Sta] that will be executed in
/// order.
#[derive(PartialEq)]
pub struct StaL {
    sta_l: Vec<Sta>,
}

impl GraphDisplay for StaL {
    fn graph_display(&self, graph: &mut String, id: &mut usize, indent: usize) {
        graph.push_str(&format!(
            "\n{:dec$}subgraph StaL_{}[StaL]",
            "",
            id,
            dec = indent
        ));
        *id += 1;
        for sta in &self.sta_l {
            match sta {
                Sta::Return(return_node) => return_node.graph_display(graph, id, indent + 2),
                Sta::NatCall(nat_call) => nat_call.graph_display(graph, id, indent + 2),
                Sta::Exp(exp) => exp.graph_display(graph, id, indent + 2),
            }
        }
        graph.push_str(&format!("\n{:indent$}end", "", indent = indent));
    }
}

impl_debug!(StaL);

impl StaL {
    pub fn new(sta_l: Vec<Sta>) -> Self {
        Self { sta_l }
    }

    pub fn parse(tokens: &mut VecDeque<TokenContainer>) -> ResultOption<StaL> {
        // <sta_l> ::= T_LEFT_E {<sta>} T_RIGHT_E
        if let some_token!(Token::LeftBrace) = tokens.front() {
            tokens.pop_front();
            let mut sta_l = Vec::new();

            while let Some(sta) = Sta::parse(tokens)? {
                sta_l.push(sta);
            }

            if let Some(TokenContainer {
                token: Token::RightBrace,
                ..
            }) = tokens.pop_front()
            {
                Ok(Some(StaL::new(sta_l)))
            } else {
                Err(CustomError::UnexpectedToken(
                    "Expected a right curly bracket".to_string(),
                ))
            }
        } else {
            Ok(None)
        }
    }
}
