use crate::config;
use rustyline::DefaultEditor;
use std::fs;
use std::fs::File;
use std::io::Write as _;

pub fn commands() {
    println!("
Ctrl-A, Home        Move cursor to the beginning of line
Ctrl-B, Left        Move cursor one character left
Ctrl-E, End         Move cursor to end of line
Ctrl-F, Right       Move cursor one character right
Ctrl-H, Backspace   Delete character before cursor
Ctrl-I, Tab         Next completion
Ctrl-K              Delete from cursor to end of line
Ctrl-L              Clear screen
Ctrl-N, Down        Next match from history
Ctrl-P, Up          Previous match from history
Ctrl-X Ctrl-U       Undo
Ctrl-Y              Paste from Yank buffer (Meta-Y to paste next yank instead)
Meta-<              Move to first entry in history
Meta->              Move to last entry in history
Meta-B, Alt-Left    Move cursor to previous word
Meta-C              Capitalize the current word
Meta-D              Delete forwards one word
Meta-F, Alt-Right   Move cursor to next word
Meta-L              Lower-case the next word
Meta-T              Transpose words
Meta-U              Upper-case the next word
Meta-Y              See Ctrl-Y
Meta-Backspace      Kill from the start of the current word, or, if between words, to the start of the previous word
Meta-0, 1, ..., -   Specify the digit to the argument. – starts a negative argument.

Thanks to <https://github.com/kkawakam/rustyline#emacs-mode-default-mode>.
    ");
}

const EXAMPLE_TOML: &str = r#"api_key = "<YOUR SECRET API KEY>"
model = "gpt-4-turbo-preview"
max_tokens = 2048
temperature = 0.8"#;

pub fn missing_toml(args: Vec<String>) {
    // At this point the old organization name is not used so we can use the new one.
    let old_org = false;
    let default_path = config::default_path(None, old_org);
    eprintln!(
        r#"
Could not find a configuration file.

To fix this, use `{} --config=<Path to ata.toml>` or create `{1}`. For the last option, type `y` to write the following example file:

```
{EXAMPLE_TOML}
```

Next, replace `<YOUR SECRET API KEY>` with your API key, which you can request via https://platform.openai.com/api-keys.
For key permissions, select "Restricted" and select write only for "Model capabilities".

The `max_tokens` sets the maximum amount of tokens that the server can answer with.
Longer answers will be truncated.

The `temperature` sets the `sampling temperature`. From the OpenAI API docs: "What sampling temperature to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer." According to Stephen Wolfram [1], setting it to a higher value such as 0.8 will likely work best in practice.

[1]: https://writings.stephenwolfram.com/2023/02/what-is-chatgpt-doing-and-why-does-it-work/
    "#,
        args[0],
        default_path.display()
    );

    let mut rl = DefaultEditor::new().unwrap();
    let msg = format!(
        "\x1b[1mDo you want me to write this example file to {0:?} for you to edit? [y/N]\x1b[0m",
        default_path
    );
    let readline = rl.readline(&msg);
    if let Ok(msg) = readline {
        let response: bool = msg
            .trim()
            .chars()
            .next()
            .map(|c| c.to_lowercase().collect::<String>() == "y")
            .unwrap_or(false);
        if response {
            if !default_path.exists() && !default_path.parent().unwrap().is_dir() {
                let dir = default_path.parent().unwrap();
                fs::create_dir_all(dir).expect("Could not make configuration directory");
            }
            let mut f = File::create(&default_path).expect("Unable to create file");
            f.write_all(EXAMPLE_TOML.as_bytes())
                .expect("Unable to write to file");
            println!();
            println!("Wrote to {default_path:?}.");
        }
    }

    std::process::exit(1);
}
