# Claude Code — Installation Windows 11

## Voraussetzungen

Git for Windows installieren: https://git-scm.com/downloads/win
→ Installer starten, alles auf Standard lassen, durchklicken.

---

## CMD öffnen



---

## Claude installieren

Diesen Befehl ins CMD-Fenster kopieren und Enter drücken:
Windows-Taste → cmd eintippen → Enter
curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd

powershell -Command "[Environment]::SetEnvironmentVariable('PATH', $env:USERPROFILE + '\.local\bin;' + [Environment]::GetEnvironmentVariable('PATH','User'), 'User')"

Fenster schließen und ein neues öffnen - sonst wird claude nicht gefunden

---

## Testen und login 


claude  hiwr wir dein code ode reine mail an m.mader abgesendet. deiser code muss zum verbinden angegebne wrden.

Aufruf ohne nachfrage von Claude
claude --dangerously-skip-permissions

/modell sonet4.6

https://status.claude.com/
https://claude.ai/settings/usage

---

## Agenten installieren

Diese Befehle nacheinander ins CMD-Fenster eingeben:

curl -L https://github.com/michipriv/agent-prompt/archive/refs/heads/master.zip -o agents.zip
Invoke-WebRequest -Uri "https://github.com/michipriv/agent-prompt/archive/refs/heads/master.zip" -OutFile agents.zip

tar -xf agents.zip

agent-prompt-master\bin\sync-claude.exe install

del agents.zip
rmdir /s /q agent-prompt-master

Agenten landen automatisch in %USERPROFILE%\.claude\agents\
