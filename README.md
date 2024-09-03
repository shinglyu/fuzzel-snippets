# fuzzel-snippets

fuzzel-snippets is a command-line tool written in Rust that allows you to manage and copy code snippets to clipboard using a YAML configuration file. It leverages the `fuzzel` command for displaying a selection menu. It supports Wayland via the `wl-copy` tool.

## Features

- Load snippets from a YAML configuration file.
- Display snippets using `fuzzel`.
- Copy the selected snippets into the clipboard.

## Dependencies

- `fuzzel` (a minimalistic application launcher)
- `wl-copy` from [wl-clipboard](https://github.com/bugaevc/wl-clipboard)

## Installation

1. **Install Rust**: Follow the instructions on [rust-lang.org](https://www.rust-lang.org/).
2. **Install fuzzel**: Follow the instructions on [fuzzel](https://codeberg.org/dnkl/fuzzel).
3. **Install wl-copy**: Follow the instructions on [wl-clipboard](https://github.com/bugaevc/wl-clipboard).

4. **Clone the repository**:
    ```sh
    git clone https://github.com/yourusername/fuzzel-snippets.git
    cd fuzzel-snippets
    ```

5. **Build the project**:
    ```sh
    cargo install --path .
    ```

6. **Run the executable**:
    ```sh
    fuzzel-snippets --configfile path/to/your/config.yaml
    
    fuzzel-snippets --help # To see all commandline options
    ```

## Configuration file format

By default the tool looks for the configuration file in `~/.snippets`. Use spaces instead of tabs when indenting.

```
snippets:
  - name: My Snippet 1
    content: |
      Mutiline 1
      Mutiline 2
  - name: My Snippet 2
    content: |
      Mutiline 1
      Mutiline 2
```

## Credit
This prorject is inspired by https://github.com/chmouel/raffi.
