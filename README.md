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
Or use your package manager to install the application.
For example, Brew on MacOS or AUR on Arch Linux have packages available.

To specify the API key and some basic model settings, start the application.
It should give an error and the option to create a configuration file called `ata.toml` for you.
Press `y` and `ENTER` to create a `ata.toml` file.

Next, request an API key via <https://platform.openai.com/api-keys> and update the key in the example configuration file.
They key permissions can be "Restricted" to only "Model capabilities".

For example, a `ata.toml` file could look like this:

```toml
api_key = "<YOUR SECRET API KEY>"
model = "gpt-4-turbo-preview"
max_tokens = 2048
temperature = 0.8
```

An `org` field is optional and can be used to specify which organization is used for an API request.
The `org` field should use the Organization ID, which can be found at
https://platform.openai.com/account/organization.

For more information, see:

```sh
$ ata --help
```

## FAQ

**How much will I have to pay for the API?**

Using OpenAI's API for chat is very cheap.
Let's say that an average response is about 500 tokens, so costs about $0.015 (with GPT-4).
That means that if you do 50 requests per day, then that will cost you about $0.75 per day ($15 per month assuming you only use it only on workdays).
If you use GPT-3.5, then the costs will be much lower.

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
