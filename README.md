# Claude Agenten installieren


## Windows 11

PowerShell:
```powershell
$TOKEN = 
Invoke-WebRequest -Uri "https://$TOKEN@github.com/michipriv/agent-prompt/archive/refs/heads/master.zip" -OutFile "$env:TEMP\agents.zip"
Expand-Archive "$env:TEMP\agents.zip" -DestinationPath "$env:TEMP\agents" -Force
cd "$env:TEMP\agents\agent-prompt-master"
.\bin\sync-claude.exe install
```

Agenten landen in `%USERPROFILE%\.claude\agents\`.

---

## Linux (Debian / Ubuntu)

```bash
TOKEN=
wget "https://$TOKEN@github.com/michipriv/agent-prompt/archive/refs/heads/master.zip"
unzip master.zip
cd agent-prompt-master
chmod +x bin/sync-claude
./bin/sync-claude install
```

Agenten landen in `~/.claude/agents/`.
