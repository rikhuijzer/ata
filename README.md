# ata

Ask the Terminal Anything (ATA): OpenAI GPT in the terminal.

At the time of writing, you can use `text-davinci-003` already which is very likely the same as ChatGPT apart from some "Chat" aspects which give it a prettier appearance.
When using this as a daily driver for your searches, costs will likely stay below a dollar per day.

[![asciicast](https://asciinema.org/a/553907.svg)](https://asciinema.org/a/553907)

## Usage

Request an API key via <https://beta.openai.com/account/api-keys>.
Next, set the API key, which model you want to use, and the maximum amount of tokens that the server can respond with in `ata.toml`:

```toml
api_key = "<YOUR SECRET API KEY>"
model = "text-davinci-003"
max_tokens = 250
temperature = 0
```

and run:

```sh
$ cargo run ata.toml
```

## Developer notes

Use:

```sh
$ cargo watch -c -x 'run ata.toml'
```

I've tried to use the `rustyline` and `inquire` terminal packages for Rust but they both capture CTRL + C which makes them unwieldy.
That's why this package is going full Richard Hipp style and doing it ourselves.
