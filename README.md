# A Calc and Counter Written With Rust.

Actually i just tried to write something and i said "Why am i don't use any library i can use GTK Lib and i tried" then i make this 2 adjacent application using `gtk::Stack` and there is the application.

## Depencides

- [cargo](https://github.com/rust-lang/cargo)

## Installation

### On Unix like OS
```
git clone https://github.com/CnegAsuy/gtk4-tries
cd gtk4-tries
cargo build --release
```

The binary file will in `gtk4-tries/target/release/gtk4-tries`

### On Windows
> I don't write rust on windows, sory idk.


## File Architecture

```
    gtk4-tries
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    ├── gui_calc.rs
    ├── gui_counter.rs
    ├── gui.rs
    └── main.rs

2 directories, 7 files
```