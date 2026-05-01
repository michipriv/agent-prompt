---
name: compiler-pfade
description: "Vollständige Compiler-Toolchain-Referenz für Windows 11 — GCC, Clang/LLVM, Rust, Arduino, CMake, Ninja, Conan, Cross-Compilation"
model: sonnet
---

# Compiler-Toolchain-Referenz Windows 11

Dieses Referenz-Dokument enthält alle installierten Compiler, Build-Tools und Entwicklungswerkzeuge auf diesem System.

---

## C/C++ Toolchain

### GCC (MinGW-W64)

| Tool | Version |
|---|---|
| gcc | 15.2.0 (x86_64-ucrt-posix-seh) |
| g++ | 15.2.0 |
| gdb | 16.3 |

Standards: C++98 bis C++23, C89 bis C23

### LLVM/Clang

| Tool | Version |
|---|---|
| clang | 21.1.8 |
| clang++ | 21.1.8 |
| clang-cl | 21.1.8 (MSVC-kompatibel) |
| clangd | 21.1.8 (Language Server) |
| clang-tidy | 21.1.8 (Linter) |
| clang-format | 21.1.8 (Formatter) |
| lldb | 21.1.8 (Debugger) |
| llvm-cov | 21.1.8 (Coverage) |

---

## Rust Toolchain

| Tool | Version |
|---|---|
| rustc | 1.92.0 |
| cargo | 1.92.0 |
| rustup | 1.28.2 |
| clippy | via rustup |
| rustfmt | via rustup |
| rust-analyzer | via rustup |

**Cross-Compilation:** cargo-zigbuild v0.20.1, zig v0.15.2, Target: aarch64-unknown-linux-gnu

---

## Build-Systeme

| Tool | Version |
|---|---|
| CMake | 4.2.1 |
| Ninja | 1.13.2 |
| Make | 3.81 (GnuWin32) |

---

## Static Analysis & Formatierung

| Tool | Version | Zweck |
|---|---|---|
| cppcheck | 2.18.0 | C/C++ Analyzer |
| clang-tidy | 21.1.8 | Linter |
| clippy | Rust 1.92.0 | Rust Linter |
| clang-format | 21.1.8 | C/C++ Formatter |
| rustfmt | Rust 1.92.0 | Rust Formatter |
| ccache | 4.12.2 | Compiler Cache |

---

## Package Manager

| Tool | Version | Sprache |
|---|---|---|
| Conan | 2.24.0 | C/C++ |
| Cargo | 1.92.0 | Rust |

---

## Arduino

| Tool | Version |
|---|---|
| Arduino IDE | 2.3.6 (MS Store) |

Enthält: AVR-GCC, ARM-GCC, AVRDUDE, Board Manager, Library Manager

---

## PATH-Refresh (PowerShell)

```powershell
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
```

---

# ERFOLGSDEFINITION
Dieses Dokument ist eine reine Referenz. Es werden keine Ausgaben generiert — nur Nachschlagen von Versionen und Pfaden.

# SCOPE-BOUNDARY
Für aktive Entwicklung: gcc | clang | rust | esp32_idf | dev_cpp

# SELF-CHECK
- [ ] Richtige Toolchain nachgeschlagen?
- [ ] Echte Umlaute (ü, ä, ö, ß)?
