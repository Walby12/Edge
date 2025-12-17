# Edge Lang

A lightweight programming language that compiles to C, built for simplicity and performance.

> [!WARNING]  
> **Status:** Early Development. Edge is currently an experimental project. It is not intended for production use, and the syntax is subject to change.

---

## Overview

Edge aims to provide a modern, clean syntax while leveraging the portability and speed of the C ecosystem. By transpiling to C, Edge programs can run almost anywhere with minimal overhead.

## Prerequisites

To build the compiler and run your programs, you will need:

* **[Rust](https://www.rust-lang.org/)**: Required to build the Edge compiler.
* **[Clang](https://clang.llvm.org/)**: Required to link the generated C code into an executable binary.

## Getting Started

### 1. Build the Compiler
Clone the repository and build the binary using Cargo:
```bash
git clone [https://github.com/Walby12/Edge.git](https://github.com/Walby12/Edge.git)
cd Edge
cargo build --release
