#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

mod args;
mod config;
mod help;
mod prompt;

use clap::Parser as _;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::fs::File;
use std::io::Read;

use std::result::Result;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::args::Ata;
use crate::config::Config;
use crate::prompt::print_error;

fn main() -> prompt::TokioResult<()> {
    init_logger();
    let flags: Ata = Ata::parse();
    if flags.print_shortcuts {
        help::commands();
        return Ok(());
    }
    let filename = flags.config.location();
    if !filename.exists() {
        help::missing_toml();
    }
    let mut contents = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    let config = Config::from(contents);
    config.validate().unwrap_or_else(|e| {
        error!("Config error!: {e}. Dying.");
        panic!()
    });

    println!("Ask the Terminal Anything");

    if !flags.hide_config {
        println!("{config}");
    }

    let mut rl = Editor::<()>::new()?;

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let is_running = Arc::new(AtomicBool::new(false));
    let is_running_clone = is_running.clone();
    let abort = Arc::new(AtomicBool::new(false));
    let abort_clone = abort.clone();
    thread::spawn(move || {
        let abort = abort_clone.clone();
        let is_running = is_running.clone();
        loop {
            let msg: Result<String, _> = rx.recv();
            if let Ok(line) = msg {
                let mut retry = true;
                let mut count = 1;
                while retry {
                    let result = prompt::request(
                        abort.clone(),
                        is_running.clone(),
                        &config,
                        line.to_string(),
                        count,
                    );
                    retry = match result {
                        Ok(retry) => retry,
                        Err(e) => {
                            eprintln!();
                            eprintln!();
                            let msg = format!("prompt::request failed with: {e}");
                            print_error(is_running.clone(), &msg);
                            false
                        }
                    };
                    count += 1;
                    if retry {
                        let duration = Duration::from_millis(500);
                        thread::sleep(duration);
                    }
                }
            }
        }
    });

    prompt::print_prompt();

    loop {
        // Using an empty prompt text because otherwise the user would
        // "see" that the prompt is ready again during response printing.
        // Also, the current readline is cleared in some cases by rustyline,
        // so being on a newline is the only way to avoid that.
        let readline = rl.readline("");
        match readline {
            Ok(line) => {
                if is_running_clone.load(Ordering::SeqCst) {
                    abort.store(true, Ordering::SeqCst);
                }
                if line.is_empty() {
                    continue;
                }
                rl.add_history_entry(line.as_str());
                tx.send(line).unwrap();
            }
            Err(ReadlineError::Interrupted) => {
                if is_running_clone.load(Ordering::SeqCst) {
                    abort.store(true, Ordering::SeqCst);
                } else {
                    break;
                }
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("{err:?}");
                break;
            }
        }
    }
    Ok(())
}

fn init_logger() {
    let env = env_logger::Env::default().default_filter_or("warn");
    env_logger::Builder::from_env(env)
        .format_timestamp(None)
        .init();
}
