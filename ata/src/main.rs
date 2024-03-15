mod config;
mod help;
mod prompt;

use crate::config::Config;
use crate::config::ConfigLocation;
use crate::prompt::print_error;
use crate::prompt::print_prompt;
use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::Cmd;
use rustyline::ConditionalEventHandler;
use rustyline::DefaultEditor;
use rustyline::Event;
use rustyline::EventContext;
use rustyline::EventHandler;
use rustyline::KeyEvent;
use rustyline::RepeatCount;
use std::env;
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
use toml::from_str;

/// Ask the Terminal Anything (ATA): OpenAI GPT in the terminal
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Flags {
    /// Path to the configuration TOML file.
    #[arg(short = 'c', long = "config", default_value = "default")]
    config: ConfigLocation,

    /// Avoid printing the configuration to stdout.
    #[arg(long)]
    hide_config: bool,

    /// Print the keyboard shortcuts.
    #[arg(long)]
    print_shortcuts: bool,

    /// Print the default config location.
    #[arg(long)]
    print_default_config_location: bool,
}

struct ClearEventHandler;
impl ConditionalEventHandler for ClearEventHandler {
    fn handle(&self, _: &Event, _: RepeatCount, _: bool, _: &EventContext) -> Option<Cmd> {
        thread::spawn(|| {
            thread::sleep(Duration::from_millis(100));
            print_prompt();
        });
        Some(Cmd::ClearScreen)
    }
}

static HAD_FIRST_INTERRUPT: AtomicBool = AtomicBool::new(false);

fn main() -> prompt::TokioResult<()> {
    let args: Vec<String> = env::args().collect();
    let flags: Flags = Flags::parse();
    if flags.print_shortcuts {
        help::commands();
        return Ok(());
    }
    if flags.print_default_config_location {
        let old_org = false;
        let default_path = config::default_path(None, old_org);
        println!("{default_path:?}");
        return Ok(());
    }
    let old_filename = flags.config.location(true);
    let filename = flags.config.location(false);
    println!("Ask the Terminal Anything");
    if !old_filename.exists() && !filename.exists() {
        help::missing_toml(args);
    }
    let filename = if old_filename.exists() {
        old_filename
    } else {
        filename
    };
    let mut contents = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    let config: Config = from_str(&contents).unwrap();

    let model = config.clone().model;
    let max_tokens = config.max_tokens;
    let temperature = config.temperature;

    if !flags.hide_config {
        println!();
        println!("model: {model}");
        println!("max_tokens: {max_tokens}");
        println!("temperature: {temperature}");
        println!();
    }

    if model.contains("text") {
        eprintln!(
            "\x1b[1mWARNING:\x1b[0m\n\
            It looks like you are using a text completion model.\n\
            This will likely result in an \"Invalid URL (POST /v1/chat/completions)\" error.\n\
            This application only supports chat models such as `gpt-3.5-turbo` since\n\
            they are cheaper and, according to Greg Brockman, perform better.\n\
            "
        );
    }

    let mut rl = DefaultEditor::new().unwrap();

    let clear_handler = EventHandler::Conditional(Box::new(ClearEventHandler));
    rl.bind_sequence(KeyEvent::ctrl('L'), clear_handler);

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
                let _ = rl.add_history_entry(line.as_str());
                tx.send(line).unwrap();
                HAD_FIRST_INTERRUPT.store(false, Ordering::Relaxed);
            }
            Err(ReadlineError::Interrupted) => {
                if is_running_clone.load(Ordering::SeqCst) {
                    abort.store(true, Ordering::SeqCst);
                } else {
                    if !HAD_FIRST_INTERRUPT.load(Ordering::Relaxed) {
                        HAD_FIRST_INTERRUPT.store(true, Ordering::Relaxed);
                        println!("\nPress Ctrl-C again to exit.");
                        thread::sleep(Duration::from_millis(100));
                        println!();
                        prompt::print_prompt();
                        continue;
                    } else {
                        break;
                    }
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
