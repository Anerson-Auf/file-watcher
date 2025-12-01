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
- `-i, --ignore-list <file>` - Path to ignore patterns file (glob patterns, one per line)
- `-r, --recursive` - Currently unused (always recursive for directories)

### Ignore Patterns

Create a file with glob patterns (one per line) to ignore certain files:

```
*.txt
*/name.json
target/
**/*.log
```

### Examples

```bash
# Watch current directory (recursive by default)
cargo run watch -p .

# Watch file
cargo run watch -p D:/vids/test.mp4

# Watch with detailed events
cargo run watch -p ./src -d

# Watch with ignore patterns
cargo run watch -p ./src -i ignore.txt

# Combine flags
cargo run watch -p ./src -d -i ignore.txt
```

## Installation

```bash
cargo build --release
```

Binary will be in `target/release/fileWatcher.exe` (Windows) or `target/release/fileWatcher` (Linux/Mac).