use std::fs::create_dir_all;
use std::path::Path;

use inkwell::context::Context as InkContext;
use inkwell::{builder::Builder, module::Module};
use log::trace;
use miette::{Context, IntoDiagnostic, Result};

use crate::ast::{nodes::FileTreeRoot, visitors::AstMutVisitor};

pub struct CodeGenerator<'ctx> {
    context: &'ctx InkContext,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl CodeGenerator<'_> {
    pub fn compile(root: &FileTreeRoot, name: &str, folder: &str) -> Result<()> {
        let context = InkContext::create();
        let module = context.create_module(name);
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
        };
        compiler.visit_file_tree_root(root)?;

        let path = Path::new(folder).join(name).with_added_extension("ll");
        let parent = path.as_path().parent().context("No parent folder")?;
        create_dir_all(parent)
            .into_diagnostic()
            .context(format!("Cannot create folders for `{}`", name))?;
        let path_str = path.to_str().context("Invalid path format")?.to_owned();
        compiler
            .module
            .print_to_file(path)
            .into_diagnostic()
            .context(format!(
                "Failed to save program in HIR format file {}",
                path_str
            ))?;
        Ok(())
    }
}

impl AstMutVisitor<'_, ()> for CodeGenerator<'_> {
    fn default_t(_: super::DefaultCause) -> miette::Result<(), miette::Error> {
        Ok(())
    }

    fn visit_function_call(
        &mut self,
        function_call: &crate::ast::nodes::calls::functions::FunctionCall<'_>,
    ) -> Result<(), miette::Error> {
        trace!("Compiling a native function call");

        let name = function_call.name;
        match name {
            "exit" => {
                trace!("Found an exit call");

                let argument_type = self.context.i32_type();
                let exit_function_type = self
                    .context
                    .void_type()
                    .fn_type(&[argument_type.into()], false);
                let exit_function = self.module.add_function("exit", exit_function_type, None);

                trace!("Function declared");

                let argument = argument_type.const_int(42, false);
                self.builder
                    .build_call(exit_function, &[argument.into()], "call_exit")
                    .into_diagnostic()
                    .context("While creating call to exit")?;
                self.builder
                    .build_unreachable()
                    .into_diagnostic()
                    .context("While creating unreachable end of branch")?;

                trace!("Function called");

                Ok(())
            }
            _ => todo!("Cannot compile other functions for now"),
        }
    }
}
