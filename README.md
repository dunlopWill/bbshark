# bbshark

## Introduction

`bbshark` is a *blazingly fast* command-line interface (CLI) written in [Rust](https://www.rust-lang.org/) for anyone to print a subset of lyrics from a very popular kid's song.

## Getting started

### Installation

Run the following:

```bash
cargo install bbshark
```

### Usage

#### Default

Simply run bbshark for default:

```bash
.cargo/bin/bbshark
# Baby shark... doo, doo, doo, doo, doo, doo.
# Baby shark!
```

#### Help

Add a help flag (`-h` or `--help`) to get CLI instructions:

```bash
.cargo/bin/bbshark -h

# ...
# USAGE:
    # bbshark [OPTIONS]
# 
# FLAGS:
    # -h, --help       Prints help information
    # -V, --version    Prints version information
# 
# OPTIONS:
    # -d, --doos <DOOS>    Number of 'doo's (between 1 and 127) [default: 6]
# ...
```

#### Doos

Vary the number of "doo"s by specifying a number between 1 and 127:

```bash
.cargo/bin/bbshark -d 12
# Baby shark... doo, doo, doo, doo, doo, doo, doo, doo, doo, doo, doo, doo.
# Baby shark!
```

### Bonus

Consider piping stdout into `cowsay` and/or `lolcat`:

```bash
.cargo/bin/bbshark | cowsay -f turtle | lolcat
 # ________________________________________
# / Baby shark... doo, doo, doo, doo, doo, \
# \ doo. Baby shark!                       /
 # ----------------------------------------
    # \                                  ___-------___
     # \                             _-~~             ~~-_
      # \                         _-~                    /~-_
             # /^\__/^\         /~  \                   /    \
           # /|  O|| O|        /      \_______________/        \
          # | |___||__|      /       /                \          \
          # |          \    /      /                    \          \
          # |   (_______) /______/                        \_________ \
          # |         / /         \                      /            \
           # \         \^\\         \                  /               \     /
             # \         ||           \______________/      _-_       //\__//
               # \       ||------_-~~-_ ------------- \ --/~   ~\    || __/
                 # ~-----||====/~     |==================|       |/~~~~~
                  # (_(__/  ./     /                    \_\      \.
                         # (_(___/                         \_____)_)
# 
# 
```

## Build steps

Clone the repository:

```bash
git clone https://github.com/dunlopWill/bbshark.git
```

Ensure you're in the correct directory:

```bash
cd bbshark
```

Run using cargo:

```bash
cargo run bbshark
```

## Contribute

Is `bbshark` missing a critical feature? Make a pull request.
