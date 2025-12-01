# File Watcher

Simple file system watcher for monitoring file changes.

## Usage

```bash
cargo run watch -p <path>
```

or

```bash
./fileWatcher.exe watch -p <path>
```

`<path>` can be a file or directory. Directories are always watched recursively.

### Flags

- `-d, --detailed` - Show detailed event types (Create, Modify, Remove)
- `-r, --recursive` - Currently unused (always recursive for directories)

### Examples

```bash
# Watch current directory (recursive by default)
cargo run watch -p .

# Watch file
cargo run watch -p D:/vids/test.mp4

# Watch with detailed events
cargo run watch -p ./src 
```

## Installation

```bash
cargo build --release
```

Binary will be in `target/release/fileWatcher.exe` (Windows) or `target/release/fileWatcher` (Linux/Mac).