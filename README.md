# ClipGrammarPedant

ClipGrammarPedant is a command-line tool written in Rust that reads text from your clipboard, sends it to the OpenAI API for grammar and spelling corrections, and updates your clipboard with the corrected text.

## Features
- Reads clipboard content.
- Sends the content to OpenAI's GPT model for grammar and spelling checks.
- Updates the clipboard with the corrected text.

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) installed on your system.
- An OpenAI API key.

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/ClipGrammarPedant.git
   cd ClipGrammarPedant
   ```

2. Set your OpenAI API key as an environment variable in your shell configuration file (e.g., `.bashrc` or `.zshrc`):
   ```bash
   export OPENAI_API_KEY="your_openai_api_key"
   ```
   After editing the file, reload your shell:
   ```bash
   source ~/.bashrc  # or `source ~/.zshrc`
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

4. Add the compiled binary to your PATH (optional):
   ```bash
   cp target/release/ClipGrammarPedant /usr/local/bin/
   ```

## Usage

1. Copy some text to your clipboard.
2. Run the tool:
   ```bash
   ClipGrammarPedant
   ```
3. The corrected text will be printed in the terminal and updated in your clipboard.

## Example
If your clipboard contains:
```
this is an example text with errors
```

After running `./ClipGrammarPedant`, the clipboard will contain:
```
This is an example text with errors.
```

## Configuration

- The OpenAI model used can be updated in the `ChatRequest` struct (`model` field).
- The API key is read from the `OPENAI_API_KEY` environment variable. Ensure it is set before running the program.

## Dependencies
- [clipboard](https://crates.io/crates/clipboard)
- [reqwest](https://crates.io/crates/reqwest)
- [serde](https://crates.io/crates/serde)

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributions
Contributions are welcome! Feel free to open an issue or submit a pull request.

## Disclaimer
This tool uses OpenAI's API, and costs may apply based on usage. Ensure you understand the pricing and terms before using the API.

