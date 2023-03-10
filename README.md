<h1 align="center"><code>ata</code>: Ask the Terminal Anything</h1>

<h3 align="center">ChatGPT in the terminal</h3>

<p align="center">
  <a href="https://asciinema.org/a/565384"><img src="https://asciinema.org/a/565384.svg" alt="asciicast"></a>
</p>

<h3 align=center>
TIP:<br>
  Run a terminal with this tool in your background and show/hide it with a keypress.<br>
    This can be done via: Iterm2 (Mac), Guake (Ubuntu), scratchpad (i3/sway), or the quake mode for the Windows Terminal.
</h3>

## Productivity benefits

- The terminal starts more quickly and requires **less resources** than a browser.
- The **keyboard shortcuts** allow for quick interaction with the query. For example, press `CTRL + c` to cancel the stream, `CTRL + ↑` to get the previous query again, and `CTRL + w` to remove the last word.
- A terminal can be set to **run in the background and show/hide with one keypress**. To do this, use iTerm2 (Mac), Guake (Ubuntu), scratchpad (i3/sway), or the quake mode for the Windows Terminal.
- The prompts are **reproducible** because each prompt is sent as a stand-alone prompt without history. Tweaking the prompt can be done by pressing `CTRL + ↑` and making changes.

## Usage

Download the binary for your system from [Releases](https://github.com/rikhuijzer/ata/releases).
If you're running Arch Linux, then you can use the AUR packages: [ata](https://aur.archlinux.org/packages/ata), [ata-git](https://aur.archlinux.org/packages/ata-git), or [ata-bin](https://aur.archlinux.org/packages/ata-bin).

To specify the API key and some basic model settings, start the application.
It should give an error and the option to create a configuration file called `ata.toml` for you.
Press `y` and `ENTER` to create a `ata.toml` file.

Next, request an API key via <https://beta.openai.com/account/api-keys> and update the key in the example configuration file.

For more information, see:

```sh
$ ata --help
```

## FAQ

**How much will I have to pay for the API?**

Using OpenAI's API for chat is very cheap.
Let's say that an average response is about 500 tokens, so costs $0.001.
That means that if you do 100 requests per day, which is a lot, then that will cost you about $0.10 per day ($3 per month).
OpenAI grants you $18.00 for free, so you can use the API for about 180 days (6 months) before having to pay.

**How does this compare to LLM-based search engines such as You.com or Bing Chat?**

At the time of writing, the OpenAI API responds much quicker than the large language model-based search engines and contains no adds.
It is particularly useful to quickly look up some things like Unicode symbols, historical facts, or word meanings.

**Can I build the binary myself?**

Yes, you can clone the repository and build the project via [`Cargo`](https://github.com/rust-lang/cargo).
Make sure that you have `Cargo` installed and then run:

```sh
$ git clone https://github.com/rikhuijzer/ata.git

$ cd ata/

$ cargo build --release
```
After this, your binary should be available at `target/release/ata` (Unix-based) or `target/release/ata.exe` (Windows).
