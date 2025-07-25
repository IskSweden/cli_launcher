# CLI Launcher 

The CLI Launcher is an application launcher from the CLI, that allows you to search, filter and execute all Path and Desktop applications on your system.

## Features
- Scans `$PATH` and `.desktop` apps
- Fuzzy searching with arrow-key navigation
- Launch apps with Enter
- Terminal-native UI

## Why?
Because i wanted to.

![alt text](image.png)

## Install

### Install with cargo
```sh
cargo install --git https://github.com/IskSweden/cli_launcher

cargo run --release

```

### Or clone and install locally
```sh
git clone https://github.com/IskSweden/cli_launcher
cd cli_launcher
cargo install --path .
```

## Usage
```sh
cla
```
## Controls

| Key         | Action              |
| ----------- | ------------------- |
| `↑ / ↓`     | Navigate list       |
| `Enter`     | Launch selected app |
| `:`         | Enter command mode  |
| `q` / `:q`  | Quit                |
| `Backspace` | Edit input          |



## Built with:
- ratatui
- fuzzy-matcher
- walkdir
- freedesktop-desktop-entry




## License

MIT - SEE [LICENSE](./LICENSE) for full details.