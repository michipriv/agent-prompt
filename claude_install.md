# Claude Code — Installation Windows 11

## Voraussetzungen

Git for Windows installieren: https://git-scm.com/downloads/win
→ Installer starten, alles auf Standard lassen, durchklicken.

---

## CMD öffnen

Windows-Taste → cmd eintippen → Enter

---

## Claude installieren

Diesen Befehl ins CMD-Fenster kopieren und Enter drücken:

curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd

Warten bis fertig.s

Fenster schließen und ein neues öffnen - sonst wird claude nicht gefunden

---

## Testen

claude --version

Zeigt eine Versionsnummer → fertig.

---

claude --dangerously-skip-permissions

---

## Agenten installieren

Diese Befehle nacheinander ins CMD-Fenster eingeben:

    curl -L https://github.com/michipriv/agent-prompt/archive/refs/heads/master.zip -o agents.zip

    tar -xf agents.zip

    agent-prompt-master\bin\sync-claude.exe install

    del agents.zip

    rmdir /s /q agent-prompt-master

Agenten landen automatisch in %USERPROFILE%\.claude\agents\
