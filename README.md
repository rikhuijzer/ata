# ata

Ask the Terminal Anything (ATA): OpenAI GPT in the terminal.

At the time of writing, you can use `text-davinci-003` already which is very likely the same as ChatGPT apart from some "Chat" aspects.
When using this for your daily searches, costs will likely stay below a dollar per day.

[![asciicast](https://asciinema.org/a/en3mUMESruzxjLtJkX3Mqi9eY.svg)](https://asciinema.org/a/en3mUMESruzxjLtJkX3Mqi9eY)

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
$ ata --config=ata.toml
```

Or, change the current directory to the one where `ata.toml` is located and run

```sh
$ ata
```

For more information, see:

```sh
$ ata --help
```
