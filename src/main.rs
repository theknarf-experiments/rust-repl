#![feature(rustc_private)]
use rust_repl::run;
use rustyline::error::ReadlineError;
use rustyline::{Cmd, DefaultEditor, KeyEvent, Result};
use std::panic;

fn repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    rl.bind_sequence(KeyEvent::ctrl('j'), Cmd::Newline);

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let src_code = format!(
                    r#"
                fn main() {{
                        {}
                    }}
                "#,
                    line.clone()
                );

                // Run user input in a separate panic catcher so that
                // compilation or runtime errors don't terminate the REPL.
                if let Err(err) = panic::catch_unwind(|| run(src_code)) {
                    // `run` already prints diagnostics, but a panic may still carry
                    // a message. Display it for additional context.
                    if let Some(msg) = err.downcast_ref::<&str>() {
                        eprintln!("Error: {}", msg);
                    } else if let Some(msg) = err.downcast_ref::<String>() {
                        eprintln!("Error: {}", msg);
                    }
                    // Continue looping to allow the user to try again.
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn main() {
    println!("Welcome to the experimental Rust REPL! 🦀");
    println!("ctrl+j for newline");
    println!("");

    let _ = repl();
}
