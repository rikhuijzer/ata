use clap::Parser;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::result::Result;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use toml::from_str;

mod help;
mod prompt;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    api_key: String,
    model: String,
    max_tokens: i64,
    temperature: f64
}

/// Ask the Terminal Anything (ATA): OpenAI GPT in the terminal
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Flags {
    /// Path to the configuration TOML file.
    #[arg(short = 'c', long = "config", default_value = "ata.toml")]
    config: String,

    /// Avoid printing the configuration to stdout.
    #[arg(long)]
    hide_config: bool,

    /// Print the keyboard shortcuts.
    #[arg(long)]
    print_shortcuts: bool,
}

fn main() -> prompt::TokioResult<()> {
    let args: Vec<String> = env::args().collect();
    let flags: Flags = Flags::parse();
    if flags.print_shortcuts {
        help::commands();
        return Ok(());
    }
    let filename = flags.config;
    if !Path::new(&filename).exists() {
        help::missing_toml(args);
    }
    let mut contents = String::new();
    File::open(filename).unwrap().read_to_string(&mut contents).unwrap();

    let config: Config = from_str(&contents).unwrap();

    let model = config.clone().model;
    let max_tokens = config.max_tokens;
    let temperature = config.temperature;
    println!("Ask the Terminal Anything");

    if !flags.hide_config {
        println!();
        println!("model: {model}");
        println!("max_tokens: {max_tokens}");
        println!("temperature: {temperature}");
        println!();
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
                let mut count = 0;
                while retry {
                    retry = prompt::request(
                            abort.clone(),
                            is_running.clone(),
                            &config,
                            line.to_string(),
                            count
                        ).unwrap();
                    count = count + 1;
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
                    continue
                }
                rl.add_history_entry(line.as_str());
                tx.send(line).unwrap();
            },
            Err(ReadlineError::Interrupted) => {
                if is_running_clone.load(Ordering::SeqCst) {
                    abort.store(true, Ordering::SeqCst);
                } else {
                    break
                }
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                eprintln!("{err:?}");
                break
            }
        }
    }
    Ok(())
}
