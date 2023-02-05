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

Request an API key via <https://beta.openai.com/account/api-keys>.
Next, set the API key, the model that you want to use, and the maximum amount of tokens that the server can respond with in `ata.toml`:

```toml
api_key = "<YOUR SECRET API KEY>"
model = "text-davinci-003"
max_tokens = 500
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

## FAQ

**How much will I have to pay for the API?**

Using OpenAI's API is quite cheap, I have been using this terminal application heavily for a few weeks now and my costs are about $0.15 per day ($4.50 per month).
The first $18.00 for free, so you can use it for about 120 days (4 months) before having to pay.

**Can I build the binary myself?**

Yes, you can clone the repository and build the project via [`Cargo`](https://github.com/rust-lang/cargo).
Make sure that you have `Cargo` installed and then run:

```sh
$ git clone https://github.com/rikhuijzer/ata.git

$ cd ata/

$ cargo build --release
```
After this, your binary should be available at `target/release/ata` (Unix-based) or `target/release/ata.exe` (Windows).
