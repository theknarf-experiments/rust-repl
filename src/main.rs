#![feature(rustc_private)]

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use rustc_codegen_cranelift::{
    rustc_span::FileName,
    rustc_interface::{Config, interface::Compiler, run_compiler, passes},
    rustc_session::{
        EarlyDiagCtxt,
        config::{Input, ErrorOutputType},
        config,
    },
		rustc_errors::{
        codes,
				registry::Registry,
    },
    rustc_driver_impl::{
        DEFAULT_LOCALE_RESOURCES,
        args,
        handle_options,
    },
};

use rustc_codegen_cranelift::CodegenMode;
use rustc_codegen_cranelift::driver::jit::run_jit;
use std::sync::atomic::AtomicBool;

static USING_INTERNAL_FEATURES: AtomicBool = AtomicBool::new(false);

fn diagnostics_registry() -> Registry {
    Registry::new(codes::DIAGNOSTICS)
}

fn test(user_snippet: String) {
    let mut default_early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());

    let src_code = format!(r#"
        fn main() {{
            {}
        }}
    "#, user_snippet);

    let at_args: Vec<String> = vec![
        "rustc".to_string(), // fake argv[0]
        "--crate-type=bin".to_string(),
        "-Clink-args=-m64".to_string(),
        "-Cprefer-dynamic".to_string(),
    ];
    let args = args::arg_expand_all(&default_early_dcx, &at_args);

    let Some(matches) = handle_options(&default_early_dcx, &args) else {
        return;
    };

    let sopts = config::build_session_options(&mut default_early_dcx, &matches);

    let config = Config {
				opts: sopts,
//        crate_cfg: matches.opt_strs("cfg"),
//        crate_check_cfg: matches.opt_strs("check-cfg"),
				crate_cfg: vec![],
        crate_check_cfg: vec![],
        input: Input::Str {
            name: FileName::Custom("repl_input.rs".into()),
            input: src_code.clone()
        },
				output_dir: None,
				output_file: None,
				ice_file: None,
				file_loader: None,
        locale_resources: DEFAULT_LOCALE_RESOURCES.to_vec(),
        lint_caps: Default::default(),
				psess_created: None,
				hash_untracked_state: None,
				register_lints: None,
				override_queries: None,
				make_codegen_backend: None,
        registry: diagnostics_registry(),
				using_internal_features: &USING_INTERNAL_FEATURES,
        expanded_args: args,
    };

    drop(default_early_dcx);

    let _result = run_compiler(config, move |compiler: &Compiler| {
				let sess = &compiler.sess;
        let _codegen_backend = &*compiler.codegen_backend;

        // Parse the crate root source code (doesn't parse submodules yet)
        // Everything else is parsed during macro expansion.
        let krate = passes::parse(sess);

        let _linker = passes::create_and_enter_global_ctxt(compiler, krate, |tcx| {
            // Make sure name resolution and macro expansion is run.
            let _ = tcx.resolver_for_lowering();

            passes::write_dep_info(tcx);

            tcx.ensure_ok().analysis(());

						run_jit(
								tcx,
								CodegenMode::JitLazy,
								vec![], // no command-line args
						);
				});
    });

}

fn repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                test(line.clone());
                //println!("Line: {}", line.clone());
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    Ok(())
}

fn main() {
    let _ = repl();
}
