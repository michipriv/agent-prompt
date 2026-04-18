# Design-Standards Hellpower Energy

## Grundprinzip
Immer helles Design. Niemals dunkles Design ohne explizite Anfrage des Users.

## Firmenfarben
- Grün (Primary):  #79a342  — RGB 121/163/66  — CMYK 26/0/60/36
- Blau (Secondary): #3ca3cb  — RGB 60/163/203  — CMYK 70/20/0/20
- Hintergrund: #ffffff
- Text: #1a1a1a
- Grau (Muted): #6b7280

## Framework
- Tailwind CSS v4 — für alle HTML-Ausgaben verwenden
- Kein Bootstrap, kein Pico, kein Custom-CSS wenn Tailwind ausreicht

## Layout & Responsive
- Mobile-first, responsive für alle Bildschirmgrößen
- Breakpoints: sm (640px), md (768px), lg (1024px), xl (1280px)

## Grafiken & Icons
- SVG bevorzugen für Icons, Diagramme und Grafiken
- Keine Raster-Grafiken (PNG/JPG) wenn SVG möglich

## Barrierefreiheit
- Semantisches HTML verwenden (nav, main, section, article, header, footer)
- Alt-Text für alle Bilder
- Kontrast mindestens 4.5:1 (WCAG AA)
- Fokus-States nicht entfernen
- Barrierefreiheit ist wichtig aber kein Blocker — Note 3 Priorität
