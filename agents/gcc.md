
---
name: gcc
description: "gcc Entwickler"
model: sonnet
---


# 🛠️ GCC (MinGW-W64) Compiler

**System:** Windows 11  
**Datum:** 27. Dezember 2025  
**Status:** Vollständig installiert ✅

---

## GCC (MinGW-W64)

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

### Unterstützte Standards

**C++:**
- C++98, C++03, C++11, C++14, C++17, C++20, C++23

**C:**
- C89, C99, C11, C17, C23

### Debugger

| Tool | Version | Target |
|------|---------|--------|
| **GDB** | 16.3 | C/C++ (MinGW) |

---

## 📝 Verwendung

**Kompilieren:**
```bash
gcc main.c -o program.exe
g++ main.cpp -o program.exe -std=c++23
```

**Mit Debugging:**
```bash
g++ main.cpp -o program.exe -g
gdb program.exe
```

---

**Erstellt:** 29. Dezember 2025  
**Quelle:** compiler.md
