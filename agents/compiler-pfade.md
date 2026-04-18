---
name: compiler-pfade
description: "compiler-pfade"
model: sonnet
---



# 🛠️ Installierte Compiler & Entwicklungstools

**System:** Windows 11  
**Datum:** 27. Dezember 2025  
**Status:** Vollständig installiert ✅

---

## 📋 Inhaltsverzeichnis

- [C/C++ Toolchain](#cc-toolchain)
- [Rust Toolchain](#rust-toolchain)
- [Arduino Entwicklung](#arduino-entwicklung)
- [Build-Systeme](#build-systeme)
- [Code-Qualität & Analyse](#code-qualität--analyse)
- [Debugger](#debugger)
- [Package Manager](#package-manager)
- [Zusätzliche Tools](#zusätzliche-tools)

---

## C/C++ Toolchain

### GCC (MinGW-W64)

| Tool | Version | Pfad |
|------|---------|------|
| **g++** | 15.2.0 | MinGW-W64 x86_64-ucrt-posix-seh |
| **gcc** | 15.2.0 | MinGW-W64 |
| **gdb** | 16.3 | GDB for MinGW-W64 |

**Features:**
- ✅ C++23 Standard Support
- ✅ POSIX Threads
- ✅ UCRT Runtime
- ✅ SEH Exception Handling

### LLVM/Clang Suite

| Tool | Version | Beschreibung |
|------|---------|--------------|
| **LLVM** | 21.1.8 | LLVM Core |
| **clang** | 21.1.8 | C Compiler |
| **clang++** | 21.1.8 | C++ Compiler |
| **clang-cl** | 21.1.8 | MSVC-kompatibel |

#### LLVM Komponenten

**Compiler & Linker:**
- `clang.exe` - C Compiler
- `clang++.exe` - C++ Compiler
- `clang-cl.exe` - MSVC-compatible mode
- `clang-cpp.exe` - C Preprocessor
- `lld.exe` - LLVM Linker
- `ld.lld.exe` - ELF Linker
- `ld64.lld.exe` - Mach-O Linker
- `lld-link.exe` - MSVC-compatible linker
- `wasm-ld.exe` - WebAssembly Linker

**Code-Analyse:**
- `clang-tidy.exe` - Code Linter
- `clang-format.exe` - Code Formatter
- `clang-check.exe` - Syntax Checker
- `clang-query.exe` - AST Query Tool
- `scan-build` - Static Analyzer

**Refactoring:**
- `clang-refactor.exe` - Auto-Refactoring
- `clang-move.exe` - Code Movement
- `clang-include-fixer.exe` - Missing Includes
- `clang-include-cleaner.exe` - Cleanup Includes
- `clang-rename.exe` - Symbol Renaming
- `clang-reorder-fields.exe` - Field Reordering

**Debugging:**
- `lldb.exe` - LLVM Debugger
- `lldb-dap.exe` - Debug Adapter Protocol
- `lldb-server.exe` - Remote Debugging

**Build & Archive:**
- `llvm-ar.exe` - Archive Tool
- `llvm-ranlib.exe` - Archive Index
- `llvm-lib.exe` - Library Tool
- `llvm-dlltool.exe` - DLL Import Libraries

**Binary Analysis:**
- `llvm-objdump.exe` - Disassembly
- `llvm-nm.exe` - Symbol Listing
- `llvm-objcopy.exe` - Object Manipulation
- `llvm-strip.exe` - Symbol Stripper
- `llvm-size.exe` - Section Sizes
- `llvm-readobj.exe` - Object Reader

**Profiling & Coverage:**
- `llvm-cov.exe` - Code Coverage
- `llvm-profdata.exe` - Profile Data
- `llvm-profgen.exe` - Profile Generator
- `llvm-mca.exe` - Machine Code Analyzer

**Special Tools:**
- `clang-repl.exe` - C++ REPL (Interactive)
- `clangd.exe` - Language Server (IDE)
- `clang-doc.exe` - Documentation Generator
- `modularize.exe` - Module Checker

---

## Rust Toolchain

| Tool | Version | Beschreibung |
|------|---------|--------------|
| **rustc** | 1.92.0 (ded5c06cf 2025-12-08) | Rust Compiler |
| **cargo** | 1.92.0 (344c4567c 2025-10-21) | Build & Package Manager |
| **rustup** | 1.28.2 (e4f3ad6f8 2025-04-28) | Toolchain Manager |

### Installierte Rust-Komponenten

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

## Arduino Entwicklung

| Tool | Version | Status |
|------|---------|--------|
| **Arduino IDE** | 2.3.6 | ✅ Installiert (MS Store) |

**Hinweis:** Die Arduino IDE enthält:
- AVR-GCC Compiler (für AVR-basierte Boards)
- ARM-GCC Compiler (für ARM-basierte Boards)
- AVRDUDE (Programmer)
- Board Manager
- Library Manager

---

## Build-Systeme

| Tool | Version | Beschreibung |
|------|---------|--------------|
| **CMake** | 4.2.1 | Cross-Platform Build System |
| **Ninja** | 1.13.2 | Fast Build Tool |
| **Make** | 3.81 | GNU Make (GnuWin32) |

**Verfügbare Build-Generatoren:**
- Ninja (empfohlen für Speed)
- Unix Makefiles
- Visual Studio Solutions
- NMake Makefiles

---

## Code-Qualität & Analyse

### Static Analysis

| Tool | Version | Zweck |
|------|---------|-------|
| **cppcheck** | 2.18.0 | C/C++ Static Analyzer |
| **clang-tidy** | 21.1.8 | LLVM C++ Linter |
| **clippy** | (Rust 1.92.0) | Rust Linter |

### Code Formatting

| Tool | Version | Zweck |
|------|---------|-------|
| **clang-format** | 21.1.8 | C/C++ Formatter |
| **rustfmt** | (Rust 1.92.0) | Rust Formatter |

### Performance

| Tool | Version | Zweck |
|------|---------|-------|
| **ccache** | 4.12.2 | Compiler Cache (Speed Boost) |

**ccache Features:**
- ✅ AVX2 Support
- ✅ File Storage
- ✅ HTTP Storage
- ✅ Redis Storage

---

## Debugger

| Tool | Version | Target |
|------|---------|--------|
| **GDB** | 16.3 | C/C++ (MinGW) |
| **LLDB** | 21.1.8 | C/C++ (LLVM) |

**LLDB Komponenten:**
- `lldb.exe` - Main Debugger
- `lldb-dap.exe` - Debug Adapter Protocol (VS Code)
- `lldb-server.exe` - Remote Debugging
- `lldb-argdumper.exe` - Argument Dumper
- `lldb-instr.exe` - Instrumentation

---

## Package Manager

### C/C++

| Tool | Version | Beschreibung |
|------|---------|--------------|
| **Conan** | 2.24.0 | C/C++ Package Manager |

**Verwendung:**
```bash
conan profile detect          # Profil erstellen
conan install <package>       # Package installieren
conan search <package>        # Package suchen
```

### Rust

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

## Zusätzliche Tools

### Profiling & Coverage

**LLVM Tools:**
- `llvm-profdata` - Profile Data Management
- `llvm-profgen` - Profile Generator
- `llvm-cov` - Code Coverage
- `llvm-mca` - Machine Code Analyzer

**Rust Tools:**
- llvm-tools-preview (via rustup)

### Cross-Compilation

**Rust ARM64 Target:**
- `aarch64-unknown-linux-gnu` - ARM64 Linux GNU/glibc

**Cross-Compilation Tools:**
- `cargo-zigbuild v0.20.1` - Cross-compilation via Zig
- `zig v0.15.2` - Cross-platform compiler/linker

### Binary Tools

**LLVM Suite:**
- `llvm-ar` - Archive Manager
- `llvm-nm` - Symbol Lister
- `llvm-objdump` - Object Dumper
- `llvm-objcopy` - Object Copier
- `llvm-strip` - Symbol Stripper
- `llvm-size` - Section Size Analyzer
- `llvm-readobj` - Object Reader
- `llvm-symbolizer` - Symbol Resolver

### Development Libraries

**DLLs (LLVM):**
- `LLVM-C.dll` - C API
- `liblldb.dll` - Debugger Library
- `libclang.dll` - Clang Library
- `libomp.dll` - OpenMP Runtime
- `LTO.dll` - Link-Time Optimization
- `Remarks.dll` - Optimization Remarks

---

## 🎯 Zusammenfassung

### Compiler-Matrix

| Sprache | Compiler | Version | Status |
|---------|----------|---------|--------|
| **C** | GCC | 15.2.0 | ✅ |
| **C** | Clang | 21.1.8 | ✅ |
| **C++** | g++ | 15.2.0 | ✅ |
| **C++** | clang++ | 21.1.8 | ✅ |
| **Rust** | rustc | 1.92.0 | ✅ |
| **Arduino** | AVR-GCC | (via IDE) | ✅ |

### Toolchain-Vollständigkeit

```
✅ Compiler:      GCC, Clang, Rust, Arduino
✅ Linker:        ld, lld, lld-link, wasm-ld
✅ Debugger:      GDB, LLDB
✅ Build-Tools:   Make, CMake, Ninja, Cargo
✅ Linter:        clang-tidy, Clippy, Cppcheck
✅ Formatter:     clang-format, rustfmt
✅ Package Mgr:   Conan, Cargo
✅ IDE Support:   clangd, rust-analyzer
✅ Performance:   ccache
```

### Unterstützte Standards

**C++:**
- C++98, C++03, C++11, C++14, C++17, C++20, C++23

**Rust:**
- Rust 2015, 2018, 2021, 2024 (Edition)

**C:**
- C89, C99, C11, C17, C23

---

## 📝 Notizen

### PATH-Konfiguration

Alle Tools sind nach Neustart des Terminals verfügbar. Bei Problemen:

```powershell
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
```

### Erste Schritte

**C++ (CMake + Ninja):**
```bash
mkdir build && cd build
cmake -G Ninja ..
ninja
```

**Rust:**
```bash
cargo new projekt
cd projekt
cargo build
cargo run
```

**Rust Cross-Compilation (ARM64 Linux):**
```bash
cargo zigbuild --target aarch64-unknown-linux-gnu --release
```

**Arduino:**
- Öffne Arduino IDE
- Tools → Board → Board Manager
- Wähle dein Board
- Sketch → Upload

---

## 🔗 Dokumentation

- **LLVM:** https://llvm.org/docs
- **Rust:** https://doc.rust-lang.org
- **CMake:** https://cmake.org/documentation
- **Conan:** https://docs.conan.io
- **Arduino:** https://docs.arduino.cc

---

**Erstellt:** 27. Dezember 2025  
**Letzte Aktualisierung:** 27. Dezember 2025 (Cross-Compilation hinzugefügt)  
**System:** Windows 11  
