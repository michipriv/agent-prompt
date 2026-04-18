---
name: karin_rust
description: "Rust Entwickler"
model: sonnet
---


**System:** Windows 11  
**Datum:** 27. Dezember 2025  
**Status:** Vollständig installiert ✅

---

## Rust Toolchain

| Tool | Version | Beschreibung |
|------|---------|--------------|
| **rustc** | 1.92.0 (ded5c06cf 2025-12-08) | Rust Compiler |
| **cargo** | 1.92.0 (344c4567c 2025-10-21) | Build & Package Manager |
| **rustup** | 1.28.2 (e4f3ad6f8 2025-04-28) | Toolchain Manager |

---

## Installierte Rust-Komponenten

```
✅ cargo-x86_64-pc-windows-msvc
✅ clippy-x86_64-pc-windows-msvc          # Linter
✅ llvm-tools-x86_64-pc-windows-msvc      # Profiling & Coverage
✅ rust-analyzer-x86_64-pc-windows-msvc   # IDE Language Server
✅ rust-docs-x86_64-pc-windows-msvc       # Offline Dokumentation
✅ rust-src                                # Standard Library Source
✅ rust-std-x86_64-pc-windows-msvc        # Standard Library
✅ rustc-x86_64-pc-windows-msvc           # Compiler
✅ rustfmt-x86_64-pc-windows-msvc         # Code Formatter
```

**Features:**
- ✅ Native Windows MSVC Target
- ✅ IDE Support via rust-analyzer
- ✅ Code Linting via Clippy
- ✅ Auto-Formatting via rustfmt
- ✅ Profiling Tools via llvm-tools
- ✅ Standard Library Source Code

---

## Unterstützte Standards

**Rust:**
- Rust 2015, 2018, 2021, 2024 (Edition)

---

## Code-Qualität & Analyse

### Static Analysis

| Tool | Version | Zweck |
|------|---------|-------|
| **clippy** | (Rust 1.92.0) | Rust Linter |

### Code Formatting

| Tool | Version | Zweck |
|------|---------|-------|
| **rustfmt** | (Rust 1.92.0) | Rust Formatter |

---

## Package Manager

| Tool | Version | Beschreibung |
|------|---------|--------------|
| **Cargo** | 1.92.0 | Rust Package Manager (Built-in) |

**Verwendung:**
```bash
cargo add <crate>             # Dependency hinzufügen
cargo build                   # Projekt bauen
cargo test                    # Tests ausführen
cargo publish                 # Crate veröffentlichen
```

---

## Cross-Compilation

**Rust ARM64 Target:**
- `aarch64-unknown-linux-gnu` - ARM64 Linux GNU/glibc

**Cross-Compilation Tools:**
- `cargo-zigbuild v0.20.1` - Cross-compilation via Zig
- `zig v0.15.2` - Cross-platform compiler/linker

---

## 📝 Verwendung

**Neues Projekt:**
```bash
cargo new projekt
cd projekt
cargo build
cargo run
```

**Mit Linting:**
```bash
cargo clippy
```

**Code-Formatierung:**
```bash
cargo fmt
```

**Tests:**
```bash
cargo test
```

**Cross-Compilation (ARM64 Linux):**
```bash
cargo zigbuild --target aarch64-unknown-linux-gnu --release
```

---

## 🔗 Dokumentation

- **Rust:** https://doc.rust-lang.org

---

**Erstellt:** 29. Dezember 2025  
**Quelle:** compiler.md
