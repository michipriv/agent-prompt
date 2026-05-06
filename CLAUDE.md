# Globale Regeln

## Antworten — Stil

- Kurz und direkt. Kein Herumreden.
- Ergebnisse in 2–5 Sätzen oder ASCII-Chart.
- Keine Dateinamen, Pfade, Code-Details in Antworten — außer explizit gefragt.
- Details nur auf Nachfrage.

## Screenshot-Pfad
- Standard-Screenshot-Ordner: `C:\Users\mmade\Pictures\Screenshots`
- Wenn User Screenshots analysieren will → zuerst dort suchen

## Git — mcp-git ist PFLICHT

### Ein Git-Tool — ausnahmslos

**mcp-git MCP-Tools** (`mcp__mcp-git__*`) sind das EINZIGE erlaubte Git-Tool.
- Gilt für alle Kontexte: Haupt-Session, EDV/Infra, Entwicklung
- Pflicht-Reihenfolge: credential_status → git_remote_list → dann handeln
- Bash-Git (`git` via Bash/Shell) NIEMALS verwenden — auch nicht als Fallback

**dev_git** ist kein zweites Git-System — es ist ein Spezialist-Agent, der intern mcp-git verwendet.
- dev_chef delegiert alle Git-Ops an dev_git, führt sie nie selbst aus
- dev_git verwendet ausschließlich mcp__mcp-git__* Tools

### Verbote
- Bash-Git (`git` via Shell) ist komplett verboten — für alle Agenten
- Den User NIEMALS nach GitHub-Zugangsdaten, Token oder Repo-URLs fragen
- GitHub-Username ist NICHT aus credential_status oder git_log ermittelbar

## Schätzungen — verboten

- Keine Zeitschätzungen (Stunden, Tage, Minuten) — diese Angaben beziehen sich auf Menschen, nicht auf KI
- Keine Kostenschätzungen (EUR, USD, Serverkosten) — Preise ändern sich, Schätzungen sind falsch
- Gilt für alle Antworten, Tabellen, YAML, HTML und Dokumente

## MCP-Connector registrieren — PFLICHT

- **Niemals** `claude_desktop_config.json` oder andere App-Config-Dateien anfassen
- MCP-Server ausschließlich über diesen Befehl registrieren (global, User-Scope):

```
claude mcp add --scope user <name> <pfad-zur-exe> -- --config <pfad-zum-projektverzeichnis>
```

Beispiel mcp-proxmox:
```
claude mcp add --scope user mcp-proxmox "C:\data\mcp-proxmox\target\release\mcp-proxmox.exe" -- --config "C:\data\mcp-proxmox"
```

- Nur diesen Befehl dem User ausgeben — keine JSON-Blöcke, keine Dateipfade zu Config-Dateien

## Sprache
- Immer echte deutsche Umlaute verwenden: ü, ä, ö, ß
- NICHT: ue, ae, oe, ss
- Gilt für alle Texte: Posts, DB-Einträge, Kundengeschichten, E-Mails, Dokumente

