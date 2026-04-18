# Coding-Standards Hellpower Energy — Dev-Team

## Teamstruktur
- Koordination erfolgt über **dev_chef**
- Architekturentscheidungen trifft **dev_architektur**
- Bei Architekturunklarheiten → an dev_architektur wenden, nicht an den User
- Kein eigenständiges Architekturdesign ohne Freigabe

## Kommunikationsregeln
- Kein Smalltalk
- Keine Rückfragen an den User
- Kein unnötiger Text, keine Einleitungen
- 1 Satz technische Analyse — dann direkt Code
- Nur geänderte oder neue Dateien ausgeben

## Code-Qualität (alle Sprachen)
- Kein Pseudocode, kein Beispielcode — nur produktionsreifer Code
- Keine ungenutzten Imports
- Keine ungenutzten Variablen
- Keine Debug-Ausgaben (kein print / console.log / puts im Produktivcode)
- Logging-System der jeweiligen Sprache verwenden
- Defensive Programmierung
- Konfigurationswerte nicht hardcoden
- SOLID-Prinzipien einhalten
- Business-Logik nie im Einstiegspunkt (main / controller / route)

## Dateigröße
- Maximal 200 Zeilen pro Datei
- Bei Überschreitung → logisch aufteilen, nie künstlich kürzen

## Datei-Header (Pflicht, oben in jeder Datei)
Neueste Version immer zuerst. Format sprachspezifisch anpassen (# / // / /* */):

```
// Filename: src/<pfad/datei>.<ext>
// V 1.2 Was wurde geändert
// V 1.1 Was davor
// V 1.0 Initial
```

- Neue Datei startet mit V 1.0
- Jede inhaltliche Änderung → Version erhöhen
- Alte Einträge bleiben unverändert

## EOF-Marker
- Letzte Zeile jeder Datei: `// EOF` (sprachspezifisch anpassen: `# EOF` für Python/Shell)

## Konfiguration
- Konfiguration ausschließlich über Config-Dateien (z.B. `config.toml`, `config.json`)
- Keine Umgebungsvariablen zur Konfiguration — weder lesen noch dokumentieren
- Config-Datei liegt neben der ausführbaren Datei oder in einem definierten Config-Verzeichnis
- Sensitive Werte (Passwörter, Keys) kommen in eine separate Secrets-Datei, nie in die Haupt-Config

## Sicherheit (alle Sprachen)
- Keine SQL-Injection: Prepared Statements / parameterisierte Queries verwenden
- Input-Validierung an Systemgrenzen (User-Input, externe APIs)
- Keine hardcoded Credentials, Tokens oder Passwörter
- Keine unsichere Deserialisierung

## Funktionsdokumentation
- Jede Funktion erhält einen Kommentar (JSDoc / Docstring / Doxygen — je nach Sprache)
- Mindestens 1 Satz Kurzbeschreibung
- Kommentare nur wo die Logik nicht selbsterklärend ist
