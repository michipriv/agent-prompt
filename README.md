# sync-claude

Synchronisiert Claude-Agenten zwischen `.claude/` und diesem Repo.

## Voraussetzungen

Rust installieren (einmalig):
```
# Windows / Linux / Mac
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Kompilieren

```
cargo build --release
```

Binary liegt danach unter:
- Windows: `target\release\sync-claude.exe`
- Linux:   `target/release/sync-claude`

## Verwendung

```
sync-claude sync      # .claude/ → Repo  (Agenten ins Repo einsammeln)
sync-claude install   # Repo → .claude/  (Agenten auf diesem PC installieren)
```

`install` erstellt automatisch ein Backup unter `<tmp>/sync-claude-backup/<timestamp>/`.

## config.toml

```toml
[paths]
# Windows:
claude_dir = "%USERPROFILE%\\.claude"
repo_dir   = "C:\\data\\agent-prompt"

# Linux:
# claude_dir = "$HOME/.claude"
# repo_dir   = "/data/agent-prompt"

[[rules]]
subdir      = "agents"
dest_subdir = "agents"
extension   = "md"

[[rules]]
subdir      = "rules"
dest_subdir = "rules"
extension   = "md"
```

## Typischer Workflow

```
# Agenten vom lokalen .claude/ ins Repo holen
sync-claude sync

# Repo committen & pushen (git)

# Auf anderem PC: Repo klonen, kompilieren, installieren
git clone https://github.com/michipriv/agent-prompt
cd agent-prompt
cargo build --release
./target/release/sync-claude install
```
