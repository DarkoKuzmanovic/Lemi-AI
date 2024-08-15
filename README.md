# Lemi AI

Lemi AI is a super simple desktop application that serves as a web wrapper for a Custom GPT specializing in Helpdesk Customer Support. Made using Tauri and Rust.

## Features

- Seamless integration of a Custom GPT for Helpdesk Customer Support
- Desktop application with web technologies
- Cross-platform compatibility (Windows, macOS, Linux)
- Refresh functionality for easy content updates

## Development

This project uses Tauri with vanilla HTML, CSS, and JavaScript. To get started with development:

1. Ensure you have Rust and Node.js installed on your system.
2. Clone this repository.
3. Run `npm install` to install dependencies.
4. Use `npm run tauri dev` to start the development server.

## Building

To build the application:

1. Run `npm run tauri build`
2. Find the built application in the `src-tauri/target/release` directory

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) with the following extensions:
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[MIT License](LICENSE.md)