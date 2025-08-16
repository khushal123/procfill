# Procfill

A lightweight process manager written in Rust that allows you to manage multiple processes from a single YAML configuration file. Similar to PM2 but focused on simplicity and ease of use.

> **Note**: Also available in Zig! Check out [Procfill-Zig](https://github.com/khushal123/procfill-zig.git)

## Features

- **Process Management**: Start, stop, and manage multiple processes
- **YAML Configuration**: Simple YAML-based configuration
- **Output Logging**: Capture and log stdout/stderr with timestamps
- **Parallel Execution**: Run multiple processes concurrently
- **Directory Management**: Automatically creates required directories
- **Process Tracking**: Track all processes with PID and status information
- **Cross-platform**: Works on Linux, macOS, and Windows

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/procfill.git
cd procfill

# Build release executable
cargo build --release

# Install options:

# Option 1: Copy to user PATH (no sudo required)
mkdir -p ~/.local/bin
cp target/release/procfill ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Option 2: Install with cargo
cargo install --path .

# Option 3: Add current directory to PATH
echo 'export PATH="'$(pwd)'/target/release:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Building the Executable

```bash
# For development (unoptimized, with debug symbols)
cargo build
# Executable at: target/debug/procfill

# For production (optimized, ~1.2MB)
cargo build --release
# Executable at: target/release/procfill
```

## Usage

### Basic Commands

```bash
# Start processes from configuration file
procfill start --filename config.yaml

# List running processes managed by procfill
procfill ls

# Kill a specific process
procfill kill <pid>

# Kill all processes managed by procfill
procfill kill-all

# Pause a process (Linux/macOS only)
procfill pause <pid>
```

### Configuration File

Create a YAML configuration file (e.g., `procfill.yaml`):

```yaml
# Directory configuration (automatically created if they don't exist)
config-dir: procfill/        # Directory for configuration files
data-dir: procfill/data      # Directory for process metadata
log-dir: procfill/logs       # Directory for output logs

commands:
  options:
    save_output: true        # Save stdout/stderr to log files
    run_parallel: true       # Run tasks in parallel
  
  tasks:
    # Example: Simple command
    - name: "list-files"
      command: "ls"
      dir: /home/user/projects/
      args: ["-la"]
    
    # Example: Long-running process
    - name: "web-server"
      command: "python"
      dir: /path/to/project/
      args: ["-m", "http.server", "8000"]
    
    # Example: Node.js application
    - name: "node-app"
      command: "node"
      dir: /path/to/node/app/
      args: ["server.js"]
    
    # Example: Custom script
    - name: "monitor"
      command: "bash"
      dir: /home/user/scripts/
      args: ["monitor.sh"]
```

### Configuration Options

#### Directory Settings

- `config-dir`: Directory for storing configuration files
- `data-dir`: Directory for storing process metadata (JSON files with PID, status)
- `log-dir`: Directory for storing process output logs

All directories are automatically created if they don't exist.

#### Command Options

- `save_output`: When `true`, captures and saves all stdout/stderr to timestamped log files
- `run_parallel`: When `true`, starts all tasks simultaneously; when `false`, starts sequentially

#### Task Properties

- `name`: Unique identifier for the task (required)
- `command`: The executable to run (required)
- `dir`: Working directory for the process (required, auto-created)
- `args`: Array of command-line arguments (optional)

### Output Logging

When `save_output` is enabled, process output is saved to:

```
<log-dir>/<task-name>_<pid>.log
```

Each log entry includes a timestamp:

```
[2025-08-16 11:57:18.490] Server started on port 8000
[2025-08-16 11:57:19.123] Accepting connections...
[2025-08-16 11:57:19.456] ERROR: Connection refused
```

## Examples

### Web Development Setup

```yaml
config-dir: dev-env/
data-dir: dev-env/data
log-dir: dev-env/logs

commands:
  options:
    save_output: true
    run_parallel: true
  
  tasks:
    - name: "frontend"
      command: "npm"
      dir: /home/user/project/frontend/
      args: ["run", "dev"]
    
    - name: "backend"
      command: "python"
      dir: /home/user/project/backend/
      args: ["manage.py", "runserver"]
    
    - name: "redis"
      command: "redis-server"
      dir: /home/user/project/
      args: []
```

### Monitoring Setup

```yaml
config-dir: monitoring/
data-dir: monitoring/data
log-dir: monitoring/logs

commands:
  options:
    save_output: true
    run_parallel: false  # Start sequentially
  
  tasks:
    - name: "system-monitor"
      command: "python"
      dir: /opt/monitoring/
      args: ["system_monitor.py"]
    
    - name: "log-watcher"
      command: "tail"
      dir: /var/log/
      args: ["-f", "syslog"]
    
    - name: "metrics-collector"
      command: "bash"
      dir: /opt/monitoring/
      args: ["-c", "while true; do date; df -h; sleep 60; done"]
```

## Process Information

Procfill stores process information in JSON files in the `data-dir`:

```json
{
  "name": "web-server",
  "command": "python",
  "args": ["-m", "http.server", "8000"],
  "dir": "/home/user/project/",
  "pid": 12345,
  "status": "running",
  "master_pid": 12340
}
```

## Building from Source

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Development

```bash
# Run directly without building
cargo run -- start --filename procfill.yaml

# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Architecture

Procfill uses a multi-threaded architecture:

1. **Main Process**: Manages configuration and spawns tasks
2. **Task Threads**: Each task runs in a separate thread (when parallel mode)
3. **Output Capture Threads**: Dedicated threads for capturing stdout/stderr
4. **Data Persistence**: Process information stored as JSON files

## Comparison with Similar Tools

| Feature | Procfill | PM2 | Supervisor | systemd |
|---------|----------|-----|------------|---------|
| Configuration | YAML | JSON/JS | INI | INI |
| Language | Rust | Node.js | Python | C |
| Binary Size | ~1.2MB | ~50MB | ~2MB | System |
| Resource Usage | Low | Medium | Low | Very Low |
| Learning Curve | Easy | Easy | Medium | Hard |
| Auto-restart | No* | Yes | Yes | Yes |
| Clustering | No | Yes | No | No |
| Web UI | No | Yes | Yes | No |

*Auto-restart planned for future release

## Troubleshooting

### Common Issues

1. **"No such file or directory" error**
   - Procfill now automatically creates all directories
   - Ensure parent directory exists and you have write permissions

2. **Logs not being created**
   - Verify `save_output: true` is set in configuration
   - Check the process actually produces output
   - Ensure write permissions for log directory

3. **Process not starting**
   - Verify the command exists and is in PATH
   - Check working directory permissions
   - Review error messages in stderr logs

4. **High memory usage**
   - Large log files can consume memory
   - Consider rotating logs periodically
   - Disable `save_output` for verbose processes

## Roadmap

- [x] Basic process management
- [x] YAML configuration
- [x] Output logging with timestamps
- [x] Parallel execution
- [x] Auto-create directories
- [ ] Process restart on failure
- [ ] CPU/Memory monitoring
- [ ] Configuration hot-reload
- [ ] Process groups
- [ ] Environment variable support
- [ ] Log rotation
- [ ] Web dashboard
- [ ] REST API

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details

## Author

Created with Rust by [khushal123](https://github.com/khushal123)

## Related Projects

- [Procfill-Zig](https://github.com/khushal123/procfill-zig.git) - Zig implementation
