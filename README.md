# sync-claude — Claude Agenten installieren

## Windows 11

**1. Rust installieren**
PowerShell als Administrator:
```powershell
winget install Rustlang.Rustup
```
Danach PowerShell neu starten.

**2. Repo klonen & kompilieren**
```powershell
git clone https://github.com/michipriv/agent-prompt C:\data\agent-prompt
cd C:\data\agent-prompt
cargo build --release
```

**3. Agenten installieren**
```powershell
.\target\release\sync-claude.exe install
```
Agenten landen in `%USERPROFILE%\.claude\agents\`.
Backup vorher automatisch in `%TEMP%\sync-claude-backup\`.

---

## Linux (Debian / Ubuntu)

**1. Rust installieren**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**2. Repo klonen & kompilieren**
```bash
git clone https://github.com/michipriv/agent-prompt /data/agent-prompt
cd /data/agent-prompt
cargo build --release
```

**3. Agenten installieren**
```bash
./target/release/sync-claude install
```
Agenten landen in `~/.claude/agents/`.
Backup vorher automatisch in `/tmp/sync-claude-backup/`.

---

## Befehle

| Befehl | Funktion |
|--------|----------|
| `sync-claude install` | Repo → `.claude/` (auf diesem PC installieren) |
| `sync-claude sync`    | `.claude/` → Repo (Agenten einsammeln) |
