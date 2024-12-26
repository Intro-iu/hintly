# 🚀 **Hintly**  
*An AI-powered command-line tool to automate terminal tasks.*

`Hintly` is a simple command-line tool written in Rust that helps automate the process of running terminal commands. It uses an AI model to generate shell commands based on user input and allows for executing or copying the generated commands. This tool can be particularly useful for automating common tasks or simplifying the execution of commands that may require specific syntax.

![Rust](https://img.shields.io/badge/rust-%23e57373.svg?style=flat&logo=rust&logoColor=white)
![License: MIT](https://img.shields.io/badge/License-MIT-green)
[![OpenAI](https://img.shields.io/badge/OpenAI-API-412991.svg)](https://openai.com)

## ✨ Features

- 🤖 **AI-generated shell commands**: Based on your prompt, the tool generates commands to execute in your terminal.
- 🔄 **Multiple operations**: You can choose to run the generated command or copy it to your clipboard for manual execution.
- 🌍 **Cross-platform support**: Works on Windows (via PowerShell) and Unix-based systems (via Shell).
- 🔒 **Configuration management**: Stores API keys and other configuration details in a TOML file.

## 📦 Installation

```bash
cargo install --git https://github.com/Intro-iu/hintly.git
```

## ⚙️ Configuration

Upon the first run, `Hintly` will check for a configuration file. If it doesn't exist, it will create a `config.toml` file in your system's configuration directory:

- On **Unix-based systems**: `~/.config/hintly/config.toml`
- On **Windows**: `C:\Users\username\AppData\Roaming\hintly\config.toml`

The configuration file contains two key fields:

- `api_key`: Your API key to access the service (e.g., OpenAI API or similar).
- `base_url`: The base URL for the API.

The tool will prompt you to enter these values if the configuration file is not present.

## 🚀 Usage

The basic syntax for using `Hintly` is:

```bash
hintly <prompt> [operation]
```

- `<prompt>`: The requirement or task description from the user.
- `[operation]` (optional): The operation to perform on the generated command. Available options are:
  - `r` or `run`: Run the generated command immediately.
  - `c` or `copy`: Copy the generated command to your clipboard.
  - If no operation is specified, the tool will prompt you to choose an operation.

### 🔧 Example

1. **Generating and running a command**:
   ```bash
   hintly "List all files in the current directory" r
   ```

2. **Generating and copying a command**:
   ```bash
   hintly "Show the system's uptime" c
   ```

3. **Interactive mode** (if no operation is provided, the tool will ask for input):
   ```bash
   hintly "Create a new directory called 'test'"
   Enter your operation [r(un) | c(opy) | q(uit)]: r
   ```

### 🛠️ Operations

- **Run operation (`r` or `run`)**: Executes the generated command in the terminal using the appropriate shell (`PowerShell` for Windows, `sh` for Unix-based systems).
- **Copy operation (`c` or `copy`)**: Copies the generated command to your clipboard for manual execution.
- **Quit operation (`q` or `quit`)**: Exits the tool.

## 🗂️ Code Structure

- **`main.rs`**: Contains the entry point of the application and handles the command-line interface (CLI) logic. It processes user input, calls the client to generate commands, and runs or copies the commands.
- **`utils.rs`**: Contains utility functions for managing configuration files, including reading and writing the configuration (e.g., API keys and base URLs).
- **`client.rs`**: Contains the `Client` struct which interfaces with the backend API to generate terminal commands based on user input. It sends a request with the prompt and receives a response with the corresponding shell command.

## 📦 Dependencies

- [**clap**](https://crates.io/crates/clap): Used for building the CLI interface.
- [**clipboard**](https://crates.io/crates/clipboard): Provides functionality to interact with the system clipboard.
- [**reqwest**](https://crates.io/crates/reqwest): Used for making HTTP requests to the API.
- [**serde**](https://crates.io/crates/serde) and [**serde_json**](https://crates.io/crates/serde_json): For serializing and deserializing configuration and API request/response data.
- [**tokio**](https://crates.io/crates/tokio): Async runtime for handling asynchronous tasks.

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

### ⚠️ Note

Make sure to have your API key and base URL configured correctly in the `config.toml` file for the tool to function properly.
