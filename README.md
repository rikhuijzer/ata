# termgpt

OpenAI GPT in the terminal

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
