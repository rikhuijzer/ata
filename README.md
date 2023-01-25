# termgpt

OpenAI GPT in the terminal.

At the time of writing, you can use `text-davinci-003` already which is very likely the same as ChatGPT apart from some "Chat" aspects which give it a prettier appearance.
When using this as a daily driver for your searches, costs will likely stay below a dollar per day.

[![asciicast](https://asciinema.org/a/553907.svg)](https://asciinema.org/a/553907)

## Usage

Request an API key via <https://beta.openai.com/account/api-keys>.
Next, set the API key, which model you want to use, and the maximum amount of tokens that the server can respond with in `termgpt.toml`:

```toml
api_key = "<YOUR SECRET API KEY>"
model = "text-davinci-003"
max_tokens = "100"
```

and run:

```sh
$ cargo run termgpt.toml
```

## Development

Use:

```sh
$ cargo watch -c -x 'run termgpt.toml'
```
