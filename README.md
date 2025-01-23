# Cursor Helper

A command-line tool to manage Cursor IDE workspace and settings.

## Features

- List all available workspaces with status indicators
- Add and list notes for each workspace
- SQLite database integration for data persistence
- Colored output for better user experience

## Prerequisites

- Rust toolchain (1.70 or later)
- Cargo (comes with Rust)
- SQLite development libraries

On Ubuntu/Debian:
```bash
sudo apt-get install libsqlite3-dev
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/cursor-helper.git
cd cursor-helper
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release/cursor-helper`

## Usage

### List Workspaces

To list all available workspaces:

```bash
cursor-helper workspace list
```

This will show all workspaces with their status:
- ✓ (green): Workspace has notes
- ✗ (red): Workspace has no notes

### Manage Notes

To add a note to a workspace:

```bash
cursor-helper notepad add --workspace <workspace-id> "Your note content"
```

To list all notes in a workspace:

```bash
cursor-helper notepad list --workspace <workspace-id>
```

## Testing

1. Run the test suite:
```bash
cargo test
```

2. Manual testing steps:

   a. List workspaces:
   ```bash
   cargo run -- notepad list
   ```

   b. Add a test note:
   ```bash
   cargo run -- notepad add your_workspace "Test note"
   ```

   c. View notes:
   ```bash
   cargo run -- notepad show your_workspace
   ```

   d. Remove a note:
   ```bash
   cargo run -- notepad remove your_workspace 0
   ```

   e. Import notes between workspaces:
   ```bash
   cargo run -- notepad import source_workspace target_workspace
   ```

## Development

The project uses Rust and several key dependencies:
- clap: For command-line argument parsing
- rusqlite: For SQLite database integration
- serde: For JSON serialization/deserialization
- colored: For terminal colors

## Project Structure

```
cursor-helper/
├── src/
│   ├── main.rs          # Entry point
│   ├── cli.rs           # CLI interface
│   ├── db.rs            # Database operations
│   └── error.rs         # Error handling
├── tests/               # Integration tests
├── Cargo.toml           # Project configuration
└── README.md           # This file
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Ensure tests pass (`cargo test`)
4. Run formatter and linter (`cargo fmt && cargo clippy`)
5. Commit your changes (`git commit -m 'Add some amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
