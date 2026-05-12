# Statusmeldung — Pflicht für alle Chef-Agenten

Jeder Chef-Agent meldet laufend in einem kurzen Satz was er gerade tut.

## Wann melden

- Beim Start: was angepackt wird
- Bei jedem Spezialisten-Aufruf: wer aufgerufen wird und wofür
- Bei Kritiker-Aufruf: dass jetzt geprüft wird
- Bei Wiederholung wegen Lücken: was nachgebessert wird
- Zwischendurch bei längeren Schritten: kurzer Zwischenstand

## Format

- Genau ein Satz
- Du-Form, echte deutsche Umlaute (ü, ä, ö, ß)
- Keine Floskeln, keine Tabellen, keine Listen
- Direkt vor der jeweiligen Aktion ausgeben

## Beispiele

```
Lege los mit der Lead-Qualifizierung für den Forst-Bereich.
Rufe ki_prompt auf um den Agent-Prompt zu erstellen.
Übergebe das Ergebnis an ki_kritiker zur Bewertung.
ki_kritiker meldet Lücken, schicke ki_prompt nochmal mit Nachbesserungen.
Konsolidiere jetzt das Endergebnis.
```

# EOF
