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

pub fn missing_toml(args: Vec<String>) {
    eprintln!(
        r#"
Could not find the file `ata.toml`. To fix this, use `{} --config=<Path to ata.toml>` or have `ata.toml` in the current dir.

For example, make a new file `ata.toml` in the current directory with the following content (the text between the ```):

```
api_key = "<YOUR SECRET API KEY>"
model = "text-davinci-003"
max_tokens = 500
temperature = 0.8
```

Here, replace `<YOUR SECRET API KEY>` with your API key, which you can request via https://beta.openai.com/account/api-keys.

The `max_tokens` sets the maximum amount of tokens that the server will answer with.

The `temperature` sets the `sampling temperature`. From the OpenAI API docs: "What sampling temperature to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer." According to Stephen Wolfram [1], setting it to a higher value such as 0.8 will likely work best in practice.

[1]: https://writings.stephenwolfram.com/2023/02/what-is-chatgpt-doing-and-why-does-it-work/

    "#, args[0]);
    std::process::exit(1);
}

