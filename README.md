```
██╗  ██╗███████╗███╗   ███╗██╗   ██╗
██║  ██║██╔════╝████╗ ████║██║   ██║
███████║█████╗  ██╔████╔██║██║   ██║
██╔══██║██╔══╝  ██║╚██╔╝██║██║   ██║
██║  ██║███████╗██║ ╚═╝ ██║╚██████╔╝
╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝ ╚═════╝ 
```
# HEMU 🚀 - A Rust Riscv Emulator

HEMU (HDU Emulator) is a RISCV64 processor simulator written in Rust. It draws inspiration from NEMU(NJU emulator)'s design philosophy and rewrites the logic of the processor using Rust, making the entire system more secure and efficient. Currently, it is an ISA-level simulator but has plans to expand into a cycle-level simulator in the future. At present, HEMU only supports the RISCV64IM instruction set architecture, but there are intentions to continue adding more instruction sets.

## Installation ⚙️

To install HEMU, you will need to have Rust installed on your machine. You can download and install Rust using the official installer from [rust-lang.org](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can clone the HEMU repository from Github:

```sh
git clone https://github.com/Clo91eaf/hemu.git
cd hemu
```

To run HEMU, you can use the following command:

```sh
cargo run --release --bin
```

<!-- ## Usage 📝

HEMU currently supports running RISCV64IM binaries. To run a binary using HEMU, you can use the following command:

```sh
cargo run --release --bin
```

Where `<filename>` is the path to the binary file that you want to execute.
 -->
## Why rust ❓

There are several potential benefits to using Rust to rewrite Nemus's logic:

1. **Memory safety**: Rust is designed to prevent memory-related bugs such as buffer overflows, null pointer dereferences, and use-after-free errors. By using Rust instead of C, the risk of these types of bugs is greatly reduced.

2. **Concurrency**: Rust has excellent support for concurrency, which means that HEMU could potentially support running multiple threads or processes simultaneously.

3. **Performance**: Rust is known for its performance, and code written in Rust can often be faster than equivalent code written in other languages. This could make HEMU faster than Nemus in some cases.

4. **Ecosystem**: Rust has a growing ecosystem of libraries and tools that can be used to develop HEMU. Taking advantage of this ecosystem could make development faster and easier.

5. **Maintainability**: Rust's syntax and features promote code that is easy to read, understand, and maintain. Writing HEMU in Rust may make it easier for future developers to modify and build upon the codebase.

## Progress 📈

- [ ] RISCV64IM instruction set architecture
- [ ] MIPS32 instruction set architecture
- [ ] Added support for peripherals.
- [ ] Added interrupt handling mechanisms.
- [ ] Added support for TLB (Translation Lookaside Buffer).
- [ ] Designed a cycle-level processor.
- [ ] Simulated a bus.
- [ ] Simulated cache functionality.

- [ ] Add some emoji to make it look better.
- [ ] Write document.
 
## Contributing 🤝

Contributions to HEMU are welcome! If you find a bug or have a feature request, please open an issue on Github. If you would like to contribute code, you can make a fork of the repository and submit a pull request.

Before submitting a pull request, please ensure that your code adheres to the project's coding standards and passes all tests.

## License 📜

HEMU is released under the MIT license. See [LICENSE](https://github.com/username/HEMU/blob/master/LICENSE) for more information.