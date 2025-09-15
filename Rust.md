Great choice! Rust is a powerful, memory-safe systems programming language. Here's a **quick and complete guide** to get started with **Rust on Ubuntu**:

---

## ✅ 1. **Install Rust (using `rustup`)**

Rust provides a toolchain installer called `rustup` which manages versions, compilers, and components.

### Run this in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> Follow the on-screen instructions. It installs:
>
> * `rustc` (the compiler)
> * `cargo` (build tool and package manager)
> * `rust-std` (standard library)

### After installation:

```bash
source $HOME/.cargo/env
```

Then confirm the installation:

```bash
rustc --version
cargo --version
```

---

## ✅ 2. **Update Rust (Optional but good practice)**

```bash
rustup update
```

---

## ✅ 3. **Install Build Essentials**

Make sure you have the required system dependencies:

```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

---

## ✅ 4. **Create a Rust Project**

```bash
cargo new hello-rust
cd hello-rust
```

This creates:

```
hello-rust/
├── Cargo.toml      # Project metadata & dependencies
└── src/
    └── main.rs     # Entry point
```

---

## ✅ 5. **Build & Run the Project**

```bash
cargo build
cargo run
```

Expected output:

```bash
Hello, world!
```

---

## ✅ 6. **Install VSCode and Rust Plugins (Optional but Recommended)**

If you're using **VSCode**, install:

* `rust-analyzer` — the modern language server for Rust
* `CodeLLDB` — for debugging Rust in VSCode
* Optionally, install `Crates` for dependency highlighting

👉 Open Extensions panel (`Ctrl + Shift + X`) and search for:

* `rust-analyzer` (by rust-lang)
* `CodeLLDB` (for debugging)

---

## ✅ 7. **Common Cargo Commands**

| Command            | Description                                |
| ------------------ | ------------------------------------------ |
| `cargo build`      | Compile the project                        |
| `cargo run`        | Build and run                              |
| `cargo check`      | Quickly check for errors without compiling |
| `cargo test`       | Run tests                                  |
| `cargo fmt`        | Format code (requires `rustfmt`)           |
| `cargo clippy`     | Lint your code (requires `clippy`)         |
| `cargo doc --open` | Generate and open documentation            |
| `cargo add <depe>` | Add A Dependency from cargo                |

To install extra tools:

```bash
rustup component add rustfmt clippy
```

---

## ✅ 8. **Useful Learning Resources**

* [The Rust Book (free official guide)](https://doc.rust-lang.org/book/)
* [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
* [Rustlings (interactive exercises)](https://github.com/rust-lang/rustlings)
* [Crates.io](https://crates.io) for libraries and dependencies
* [Docs.rs](https://docs.rs) for API documentation

---

Would you like help setting up a sample CLI app, REST API (with Axum/Actix), or something else to start your learning?

