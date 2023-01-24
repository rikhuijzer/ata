# termgpt

OpenAI GPT in the terminal

[![asciicast](https://asciinema.org/a/juh3Zcxn2kq25PI2BgZmRGlx2.svg)](https://asciinema.org/a/juh3Zcxn2kq25PI2BgZmRGlx2)

## Usage

Set the API key and which model you want to use in `termgpt.toml`:

```
api_key = "<YOUR SECRET API KEY>"
model = "text-davinci-003"
max_tokens = "100"
```
and run:
```sh
$ cargo run termgpt.toml
```
