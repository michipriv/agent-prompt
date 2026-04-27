# Claude Agenten installieren

## Windows 11

1. Repo herunterladen: [ZIP Download](https://github.com/michipriv/agent-prompt/archive/refs/heads/master.zip)
2. ZIP entpacken
3. PowerShell im entpackten Ordner öffnen:
```powershell
.\bin\sync-claude.exe install
```

Agenten werden nach `%USERPROFILE%\.claude\agents\` installiert.

---

## Linux (Debian / Ubuntu)

1. Repo herunterladen:
```bash
wget https://github.com/michipriv/agent-prompt/archive/refs/heads/master.zip
unzip master.zip
cd agent-prompt-master
```

2. Binary ausführbar machen und installieren:
```bash
chmod +x bin/sync-claude
./bin/sync-claude install
```

Agenten werden nach `~/.claude/agents/` installiert.
