# Filename: sync_claude.py
# V 1.0 Initial
"""
Synchronisiert ausgewählte Dateien aus ~/.claude/ nach C:/data/agent-prompt/.
Überschreibt nur neuere Quelldateien (mtime-Vergleich).
"""

import logging
import shutil
from pathlib import Path

logging.basicConfig(level=logging.INFO, format="%(message)s")
logger = logging.getLogger(__name__)

SOURCE_BASE = Path(r"C:\Users\mmade\.claude")
DEST_BASE = Path(r"C:\data\agent-prompt")

SYNC_RULES: list[tuple[str | None, str, str]] = [
    ("agents", "*.md", "agents"),
    ("rules", "*.md", "rules"),
    (None, "CLAUDE.md", "."),
]


def sync_file(src: Path, dest: Path) -> bool:
    """Kopiert src nach dest wenn src neuer ist. Gibt True zurück wenn kopiert."""
    if dest.exists() and dest.stat().st_mtime >= src.stat().st_mtime:
        return False
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dest)
    return True


def sync_entry(src_subdir: str | None, pattern: str, dest_subdir: str) -> int:
    """Synchronisiert alle Dateien eines Eintrags. Gibt Anzahl aktualisierter Dateien zurück."""
    src_dir = SOURCE_BASE / src_subdir if src_subdir else SOURCE_BASE
    dest_dir = DEST_BASE / dest_subdir if dest_subdir != "." else DEST_BASE

    updated = 0
    for src_file in src_dir.glob(pattern):
        dest_file = dest_dir / src_file.name
        if sync_file(src_file, dest_file):
            updated += 1
    return updated


def main() -> None:
    """Startet die Synchronisation und gibt eine Zusammenfassung aus."""
    results: list[tuple[str, int]] = []

    for src_subdir, pattern, dest_subdir in SYNC_RULES:
        label = src_subdir if src_subdir else "CLAUDE.md"
        count = sync_entry(src_subdir, pattern, dest_subdir)
        results.append((label, count))

    logger.info("Synchronisation abgeschlossen:")
    for label, count in results:
        logger.info("  %-12s %d Datei(en) aktualisiert", label, count)


if __name__ == "__main__":
    main()

# EOF
