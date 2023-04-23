# Cheatsheet CLI

`cheatsheet` is a command-line tool written in Rust that enables users to quickly view cheat sheets.  Use the `--list` command to view all available cheat sheets.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
  - [List all cheat sheets](#list-all-cheat-sheets)
  - [View a cheat sheet](#view-a-cheat-sheet)
  - [Create a cheat sheet](#create-a-cheat-sheet)
  - [Edit a cheat sheet](#edit-a-cheat-sheet)
  - [Delete a cheat sheet](#delete-a-cheat-sheet)
- [Contributing](#contributing)
- [License](#license)

## Installation

To install `cheatsheet`, ensure you have Rust and Cargo installed on your system. Then, follow these steps:

1. Clone the repository:

```sh
git clone https://github.com/yourusername/cheatsheet.git
```

2. Change to the cheatsheet directory:
```sh
cd cheatsheet
```
3. Build and Install
```sh
cargo build --release
cargo install --path .
cp -r sheets ~/.cheatsheet
export CHEAT_SHEET_PATH=~/.cheatsheets
```

## Usage


