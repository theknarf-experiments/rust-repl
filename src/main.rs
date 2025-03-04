#![feature(rustc_private)]
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use rust_repl::run;

fn repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let src_code = format!(r#"
                fn main() {{
                        {}
                    }}
                "#, line.clone());

                run(src_code);
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
