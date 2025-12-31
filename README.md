# sound

TUI sound mixer for focus, built with [fundsp](https://github.com/SamiPerttu/fundsp) and [ratatui](https://github.com/ratatui/ratatui).

## Why pink noise + 40Hz?

These appear to have the best evidence for aiding focus and concentration:

- **Pink noise**: May help mask distracting sounds and improve sleep/focus
- **40Hz (gamma frequency)**: Some studies suggest it may enhance cognitive function

Don't take this too seriously though - your mileage may vary.

## Usage

```bash
cargo run --release
```

### Controls

| Key | Action |
|-----|--------|
| `j` / `↓` | Select next sound |
| `k` / `↑` | Select previous sound |
| `h` / `←` | Decrease volume |
| `l` / `→` | Increase volume |
| `q` | Quit |

## License

MIT
