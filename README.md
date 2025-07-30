# lumix

A simple, fast command-line tool for controlling monitor brightness on Windows and Linux. Designed for easy brightness management across multiple displays.

[![GitHub](https://img.shields.io/badge/github-Yrrrrrf%2Flumix-58A6FF?style=for-the-badge&logo=github)](https://github.com/Yrrrrrf/lumix)
[![Crates.io](https://img.shields.io/crates/v/lumix.svg?style=for-the-badge&logo=rust)](https://crates.io/crates/lumix)
[![License](https://img.shields.io/github/license/Yrrrrrf/dev_utils.svg?style=for-the-badge)](./LICENSE)

## Features

- 🖥️ Multi-monitor support
- 📊 Visual brightness indicator
- 🚀 Fast and lightweight
- 🎯 Simple command interface
- 📈 Show brightness range for each monitor
- 🎨 Formatted output

## Usage

### Basic Commands

- Get brightness:
```bash
lumix get  # List all monitors and their brightness
lumix get 12345  # Specific monitor brightness
```

- Set brightness:
```bash
lumix set 75  # All monitors to 75%
lumix set 10+  # Increase all monitors by 10%
lumix set 5-  # Decrease all monitors by 5%
lumix set 12345 50  # Specific monitor to 50%
```

### Output Example

```
Monitor 12345: 75% [0..=100] ██████████░░░░░░░░░░
```

Where:
- `12345`: Monitor identifier
- `75%`: Current brightness (bold)
- `██████████░░░░░░░░░░`: Visual brightness indicator
- `[0..=100]`: Supported brightness range

## Troubleshooting

### Common Issues

1. **Monitor Not Found**
   - Ensure the monitor supports DDC/CI
   - Verify the monitor handle is correct

2. **Permission Denied**
   - Run the command prompt as administrator

3. **Invalid Brightness Value**
   - Use values between 0 and 100

### Error Messages

- `Monitor X not found`: Invalid monitor handle
- `Invalid brightness value`: Brightness must be 0-100
- `Error setting brightness`: Monitor might not support DDC/CI

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the [MIT License](./LICENSE).