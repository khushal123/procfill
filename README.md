### This is still under construction, The command usage might change. Right now its not usefull. 
# Procfill

`procfill` (alias: `pf`) is a command-line tool written in Rust for managing processes. It allows you to create, kill, and track logs of processes with ease.

## Features

- **Create Processes**: Start new processes with specified commands, it takes text file as an input too.
- **List Processes**: List all the process which are created by you with help of procfill.
- **Kill Processes**: Terminate running processes by their PID or name.
- **Track Logs**: Monitor and track logs for specific processes.

## Installation

To install `procfill`, you need to have Rust installed on your system. You can install Rust using `rustup`:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once Rust is installed, you can build and install `procfill` with the following commands:

```sh
git clone https://github.com/yourusername/procfill.git
cd procfill
cargo build --release
cargo install --path .
```

## Usage

`procfill` can be used with its full name or its alias `pf`.

### Create a Process

To create a new process, use the `create` command:

```sh
procfill create <command>
# or
pf create <command>
```

### Kill a Process

To kill a process, use the `kill` command:

```sh
procfill kill <pid>
# or
pf kill <pid>
```





