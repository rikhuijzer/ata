# termgpt

OpenAI GPT in the terminal

## Usage

Set the API key and which model you want to use in `termgpt.toml`:

```
api_key = "<YOUR SECRET API KEY>"
model = "text-davinci-003"
max_tokens = "100"
```

```sh
$ cargo watch -c -x 'run termgpt.toml'
```

Where `-c` clears the screen in between.
