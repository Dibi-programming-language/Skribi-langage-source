use std::path::Path;

use inkwell::{builder::Builder, context::Context, module::Module};

use crate::{
    ast::{
        nodes::{
            AstRoot,
            calls::{FunctionCall, IdentifierChain, VariableModification},
            conditions::Condition,
            declarations::{FunctionDeclaration, VariableDeclaration},
            expressions::Expression,
            loops::Ci,
            operations::BinaryOperation,
            statements::{Return, StatementList},
        },
        visitors::NodeVisitor,
    },
    execute::IntType,
};

pub struct CodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    verbose: bool,
}

impl CodeGenerator<'_> {
    pub fn compile(root: &AstRoot, verbose: bool) -> Result<(), ()> {
        let context = Context::create();
        let module = context.create_module("main");
        let builder = context.create_builder();

        // Create main function
        // TODO: add arguments
        let main_function_type = context.i32_type().fn_type(&[], false);
        let main_function = module.add_function("main", main_function_type, None);
        let main_block = context.append_basic_block(main_function, "main");
        builder.position_at_end(main_block);

        let mut compiler = CodeGenerator {
            context: &context,
            module,
            builder,
            verbose,
        };
        compiler.visit_root(root)?;

        compiler
            .module
            .print_to_file(Path::new("out.ll"))
            .expect("Failed to save program in HIR format");
        Ok(())
    }
}

impl NodeVisitor for CodeGenerator<'_> {
    type Value = Result<(), ()>;

    fn visit_root(&mut self, v: &AstRoot) -> Self::Value {
        if self.verbose {
            eprintln!("Compiling the root");
        }
        for statement in &v.content {
            statement.accept(self)?;
        }
        Ok(())
    }

    fn visit_return(&mut self, _v: &Return) -> Self::Value {
        todo!()
    }

    fn visit_statements(&mut self, _v: &StatementList) -> Self::Value {
        todo!()
    }

    fn visit_binary(&mut self, _v: &BinaryOperation) -> Self::Value {
        todo!()
    }

    fn visit_not(&mut self, _v: &Expression) -> Self::Value {
        todo!()
    }

    fn visit_plus(&mut self, _v: &Expression) -> Self::Value {
        todo!()
    }

    fn visit_minus(&mut self, _v: &Expression) -> Self::Value {
        todo!()
    }

    fn visit_ci(&mut self, _v: &Ci) -> Self::Value {
        todo!()
    }

    fn visit_function_dec(&mut self, _v: &FunctionDeclaration) -> Self::Value {
        todo!()
    }

    fn visit_variable_dec(&mut self, _v: &VariableDeclaration) -> Self::Value {
        todo!()
    }

    fn visit_condition(&mut self, _v: &Condition) -> Self::Value {
        todo!()
    }

    fn visit_variable_mod(&mut self, _v: &VariableModification) -> Self::Value {
        todo!()
    }

    fn visit_function_call(&mut self, v: &FunctionCall) -> Self::Value {
        if v.native {
            if self.verbose {
                eprintln!("Compiling a native function call");
            }

            if v.identifier.previous.is_some() {
                return Err(());
            }

            let name = v.identifier.identifier;
            match name {
                "exit" => {
                    if self.verbose {
                        eprintln!("Found an exit call");
                    }

                    let argument_type = self.context.i32_type();
                    let exit_function_type = self
                        .context
                        .void_type()
                        .fn_type(&[argument_type.into()], false);
                    let exit_function = self.module.add_function("exit", exit_function_type, None);

                    if self.verbose {
                        eprintln!("Function declared");
                    }

                    let argument = argument_type.const_int(1, false);
                    self.builder
                        .build_call(exit_function, &[argument.into()], "call_exit")
                        .expect("Failed to create call to exit");
                    self.builder
                        .build_unreachable()
                        .expect("Cannot build unreachable end of branch");

                    if self.verbose {
                        eprintln!("Function called");
                    }
                    Ok(())
                }
                _ => Err(()),
            }
        } else {
            todo!()
        }
    }

    fn visit_identifier_chain(&mut self, _v: &IdentifierChain) -> Self::Value {
        todo!()
    }

    fn visit_bool(&mut self, _v: bool) -> Self::Value {
        todo!()
    }

    fn visit_int(&mut self, _v: IntType) -> Self::Value {
        todo!()
    }

    fn visit_f32(&mut self, _v: f32) -> Self::Value {
        todo!()
    }

    fn visit_string(&mut self, _v: &String) -> Self::Value {
        todo!()
    }
}
