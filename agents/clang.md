
---
name: LLVM/Clang
description: "compiler-pfade"
model: sonnet
---

# 🛠️ LLVM/Clang Compiler Suite

**System:** Windows 11  
**Datum:** 27. Dezember 2025  
**Status:** Vollständig installiert ✅

---

## LLVM/Clang Suite

| Tool | Version | Beschreibung |
|------|---------|--------------|
| **LLVM** | 21.1.8 | LLVM Core |
| **clang** | 21.1.8 | C Compiler |
| **clang++** | 21.1.8 | C++ Compiler |
| **clang-cl** | 21.1.8 | MSVC-kompatibel |

---

## LLVM Komponenten

### Compiler & Linker

- `clang.exe` - C Compiler
- `clang++.exe` - C++ Compiler
- `clang-cl.exe` - MSVC-compatible mode
- `clang-cpp.exe` - C Preprocessor
- `lld.exe` - LLVM Linker
- `ld.lld.exe` - ELF Linker
- `ld64.lld.exe` - Mach-O Linker
- `lld-link.exe` - MSVC-compatible linker
- `wasm-ld.exe` - WebAssembly Linker

### Code-Analyse

- `clang-tidy.exe` - Code Linter
- `clang-format.exe` - Code Formatter
- `clang-check.exe` - Syntax Checker
- `clang-query.exe` - AST Query Tool
- `scan-build` - Static Analyzer

### Refactoring

- `clang-refactor.exe` - Auto-Refactoring
- `clang-move.exe` - Code Movement
- `clang-include-fixer.exe` - Missing Includes
- `clang-include-cleaner.exe` - Cleanup Includes
- `clang-rename.exe` - Symbol Renaming
- `clang-reorder-fields.exe` - Field Reordering

### Debugging

- `lldb.exe` - LLVM Debugger
- `lldb-dap.exe` - Debug Adapter Protocol
- `lldb-server.exe` - Remote Debugging

**LLDB Komponenten:**
- `lldb.exe` - Main Debugger
- `lldb-dap.exe` - Debug Adapter Protocol (VS Code)
- `lldb-server.exe` - Remote Debugging
- `lldb-argdumper.exe` - Argument Dumper
- `lldb-instr.exe` - Instrumentation

### Build & Archive

- `llvm-ar.exe` - Archive Tool
- `llvm-ranlib.exe` - Archive Index
- `llvm-lib.exe` - Library Tool
- `llvm-dlltool.exe` - DLL Import Libraries

### Binary Analysis

- `llvm-objdump.exe` - Disassembly
- `llvm-nm.exe` - Symbol Listing
- `llvm-objcopy.exe` - Object Manipulation
- `llvm-strip.exe` - Symbol Stripper
- `llvm-size.exe` - Section Sizes
- `llvm-readobj.exe` - Object Reader

### Profiling & Coverage

- `llvm-cov.exe` - Code Coverage
- `llvm-profdata.exe` - Profile Data
- `llvm-profgen.exe` - Profile Generator
- `llvm-mca.exe` - Machine Code Analyzer

### Special Tools

- `clang-repl.exe` - C++ REPL (Interactive)
- `clangd.exe` - Language Server (IDE)
- `clang-doc.exe` - Documentation Generator
- `modularize.exe` - Module Checker

### Development Libraries (DLLs)

- `LLVM-C.dll` - C API
- `liblldb.dll` - Debugger Library
- `libclang.dll` - Clang Library
- `libomp.dll` - OpenMP Runtime
- `LTO.dll` - Link-Time Optimization
- `Remarks.dll` - Optimization Remarks

---

## 📝 Verwendung

**Kompilieren:**
```bash
clang main.c -o program.exe
clang++ main.cpp -o program.exe -std=c++23
```

**Mit MSVC-Kompatibilität:**
```bash
clang-cl /EHsc main.cpp /Fe:program.exe
```

**Code-Analyse:**
```bash
clang-tidy main.cpp -- -std=c++23
```

**Code-Formatierung:**
```bash
clang-format -i main.cpp
```

**Debugging:**
```bash
clang++ main.cpp -o program.exe -g
lldb program.exe
```

---

## 🔗 Dokumentation

- **LLVM:** https://llvm.org/docs

---

**Erstellt:** 29. Dezember 2025  
**Quelle:** compiler.md
