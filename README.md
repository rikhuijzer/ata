<h1 align="center"><code>ata</code>: Ask the Terminal Anything</h1>

<h3 align="center">OpenAI GPT in the terminal</h3>

<p align="center">
  <a href="https://asciinema.org/a/557270"><img src="https://asciinema.org/a/557270.svg" alt="asciicast"></a>
</p>

<h3 align=center>
TIP:<br>
  Run a terminal with this tool in your background and show/hide it with a keypress.<br>
    This can be done via: Iterm2 (Mac), Guake (Ubuntu), scratchpad (i3/sway), or the quake mode for the Windows Terminal.
</h3>

At the time of writing, use `text-davinci-003`. Davinci was released together with ChatGPT as part of the [GPT-3.5 series](https://platform.openai.com/docs/model-index-for-researchers/models-referred-to-as-gpt-3-5) and they are very comparable in terms of capabilities; ChatGPT is more verbose.

## Productivity benefits

- The terminal starts more quickly and requires **less resources** than a browser.
- The **keyboard shortcuts** allow for quick interaction with the query. For example, press `CTRL + c` to cancel the stream, `CTRL + ↑` to get the previous query again, and `CTRL + w` to remove the last word.
- A terminal can be set to **run in the background and show/hide with one keypress**. To do this, use iTerm2 (Mac), Guake (Ubuntu), scratchpad (i3/sway), or the quake mode for the Windows Terminal.
- The prompts are **reproducible** because each prompt is sent as a stand-alone prompt without history. Tweaking the prompt can be done by pressing `CTRL + ↑` and making changes.

## Usage

Download the binary for your system from [Releases](https://github.com/rikhuijzer/ata/releases).
If you're running Arch Linux, then you can use the AUR packages: [ata](https://aur.archlinux.org/packages/ata), [ata-git](https://aur.archlinux.org/packages/ata-git), or [ata-bin](https://aur.archlinux.org/packages/ata-bin).

Request an API key via <https://beta.openai.com/account/api-keys>.
Next, set the API key, the model that you want to use, and the maximum amount of tokens that the server can respond with in `ata.toml`:

```toml
api_key = "<YOUR SECRET API KEY>"
model = "text-davinci-003"
max_tokens = 500
temperature = 0.8
```

Here, replace `<YOUR SECRET API KEY>` with your API key, which you can request via https://beta.openai.com/account/api-keys.

The `max_tokens` sets the maximum amount of tokens that the server will answer with.

The `temperature` sets the `sampling temperature`. From the OpenAI API docs: "What sampling temperature to use. Higher values means the model will take more risks. Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer." According to Stephen Wolfram [[1]], setting it to a higher value such as 0.8 will likely work best in practice.

[1]: https://writings.stephenwolfram.com/2023/02/what-is-chatgpt-doing-and-why-does-it-work/

Next, run:

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

## FAQ

**How much will I have to pay for the API?**

Using OpenAI's API is quite cheap, I have been using this terminal application heavily for a few weeks now and my costs are about $0.01 per day ($0.30 per month).
The first $18.00 for free, so you can use it for about 120 days (4 months) before having to pay.

**How can I make `ata` easily available (without having to specify --config)?**

You can wrap the binary to make it easily available.
For example, when you have a system with Bash available, you can create the following file:

```sh
#!/usr/bin/env bash

/path/to/ata/binary --config=/path/to/config/toml "$@"
```

and call it `ata.sh` and place it somewhere in your `PATH`.
For example, at `~/.local/bin/ata.sh`.
Next, you can use

```sh
$ ata
```

From anywhere on your system and have it automatically loaded with the right configuration.

**Can I build the binary myself?**

Yes, you can clone the repository and build the project via [`Cargo`](https://github.com/rust-lang/cargo).
Make sure that you have `Cargo` installed and then run:

```sh
$ git clone https://github.com/rikhuijzer/ata.git

$ cd ata/

$ cargo build --release
```
After this, your binary should be available at `target/release/ata` (Unix-based) or `target/release/ata.exe` (Windows).
