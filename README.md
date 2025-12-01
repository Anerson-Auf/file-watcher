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

- `-d, --detailed` - Show detailed event types:
- `-i, --ignore-list <file>` - Path to ignore patterns file (glob patterns, one per line)
- `-f, --find-list <file>` - Path to find patterns file (only show events matching these glob patterns)
- `-r, --recursive` - Currently unused (always recursive for directories)

### Ignore and Find Patterns

Create files with glob patterns (one per line):

**Ignore patterns** (`-i`) - Exclude files matching these patterns:
```
*.txt
*/name.json
target/
**/*.log
```

**Find patterns** (`-f`) - Only show events matching these patterns:
```
*.rs
src/**/*.toml
**/*.md
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

# Watch with find patterns (only show matching files)
cargo run watch -p ./src -f find.txt

# Combine flags
cargo run watch -p ./src -d -i ignore.txt -f find.txt
```

## Installation

```bash
cargo build --release
```

Binary will be in `target/release/fileWatcher.exe` (Windows) or `target/release/fileWatcher` (Linux/Mac).