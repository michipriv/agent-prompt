// Filename: src/main.rs
// V 1.2 Backup vor Install; claude_dir auto per USERPROFILE
// V 1.1 config.toml-Suche im CWD
// V 1.0 Initial — sync/install zwischen .claude/ und Repo

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    paths: Paths,
    rules: Vec<Rule>,
}

#[derive(Deserialize)]
struct Paths {
    claude_dir: String,
    repo_dir:   String,
}

#[derive(Deserialize)]
struct Rule {
    subdir:      String,
    dest_subdir: String,
    extension:   String,
}

/// Löst `claude_dir` auf — ersetzt %USERPROFILE% oder auto-erkennt den User.
fn resolve_claude_dir(raw: &str) -> PathBuf {
    let userprofile = env::var("USERPROFILE").unwrap_or_else(|_| {
        let username = env::var("USERNAME").unwrap_or_else(|_| "Unknown".into());
        format!("C:\\Users\\{}", username)
    });
    PathBuf::from(raw.replace("%USERPROFILE%", &userprofile))
}

/// Gibt true zurück wenn src neuer als dest ist oder dest nicht existiert.
fn src_is_newer(src: &Path, dest: &Path) -> bool {
    if !dest.exists() {
        return true;
    }
    let mtime = |p: &Path| -> SystemTime {
        fs::metadata(p)
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH)
    };
    mtime(src) > mtime(dest)
}

/// Kopiert src → dest (immer, ohne mtime-Check). Gibt true bei Erfolg.
fn copy_file(src: &Path, dest: &Path) -> bool {
    if let Some(parent) = dest.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("  Fehler Verzeichnis erstellen {}: {}", parent.display(), e);
            return false;
        }
    }
    match fs::copy(src, dest) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("  Fehler Kopieren {}: {}", src.display(), e);
            false
        }
    }
}

/// Kopiert src → dest wenn src neuer. Gibt true zurück wenn kopiert wurde.
fn copy_if_newer(src: &Path, dest: &Path) -> bool {
    if !src_is_newer(src, dest) {
        return false;
    }
    copy_file(src, dest)
}

/// Kopiert alle Dateien mit gegebener Extension aus src_dir nach dest_dir.
fn sync_dir(src_dir: &Path, dest_dir: &Path, ext: &str) -> usize {
    let entries = match fs::read_dir(src_dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("  Verzeichnis nicht lesbar {}: {}", src_dir.display(), e);
            return 0;
        }
    };
    let mut count = 0;
    for entry in entries.flatten() {
        let src = entry.path();
        if src.extension().and_then(|e| e.to_str()) != Some(ext) {
            continue;
        }
        let filename = src.file_name().unwrap();
        let dest = dest_dir.join(filename);
        if copy_if_newer(&src, &dest) {
            println!("    + {}", filename.to_string_lossy());
            count += 1;
        }
    }
    count
}

/// Erstellt Backup aller vorhandenen Zieldateien nach C:\tmp\backup\<timestamp>\.
fn backup_existing(claude: &Path, config: &Config) -> PathBuf {
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let backup_root = PathBuf::from(format!("C:\\tmp\\backup\\{}", ts));
    let mut count = 0;

    for rule in &config.rules {
        let src_dir = claude.join(&rule.dest_subdir);
        if !src_dir.exists() {
            continue;
        }
        if let Ok(entries) = fs::read_dir(&src_dir) {
            for entry in entries.flatten() {
                let src = entry.path();
                if src.extension().and_then(|e| e.to_str()) != Some(rule.extension.as_str()) {
                    continue;
                }
                let rel = src.strip_prefix(claude).unwrap_or(&src);
                let dest = backup_root.join(rel);
                if copy_file(&src, &dest) {
                    count += 1;
                }
            }
        }
    }

    // CLAUDE.md sichern
    let claude_md = claude.join("CLAUDE.md");
    if claude_md.exists() {
        let dest = backup_root.join("CLAUDE.md");
        if copy_file(&claude_md, &dest) {
            count += 1;
        }
    }

    if count > 0 {
        println!("Backup: {} Datei(en) → {}", count, backup_root.display());
    } else {
        println!("Backup: nichts vorhanden — kein Backup nötig");
    }
    backup_root
}

fn run_sync(config: &Config) {
    let claude = resolve_claude_dir(&config.paths.claude_dir);
    let repo   = PathBuf::from(&config.paths.repo_dir);
    let mut total = 0;

    for rule in &config.rules {
        let src  = claude.join(&rule.subdir);
        let dest = repo.join(&rule.dest_subdir);
        println!("{}:", rule.subdir);
        let n = sync_dir(&src, &dest, &rule.extension);
        println!("  {} Datei(en) aktualisiert", n);
        total += n;
    }

    let src  = claude.join("CLAUDE.md");
    let dest = repo.join("CLAUDE.md");
    if src.exists() {
        println!("CLAUDE.md:");
        if copy_if_newer(&src, &dest) {
            println!("    + CLAUDE.md");
            total += 1;
        } else {
            println!("  bereits aktuell");
        }
    }

    println!("\nSync abgeschlossen: {} Datei(en) aktualisiert", total);
}

fn run_install(config: &Config) {
    let userprofile = env::var("USERPROFILE").unwrap_or_else(|_| {
        let u = env::var("USERNAME").unwrap_or_else(|_| "Unknown".into());
        format!("C:\\Users\\{}", u)
    });
    let username = PathBuf::from(&userprofile)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "Unknown".into());

    let claude = PathBuf::from(&userprofile).join(".claude");
    let repo   = PathBuf::from(&config.paths.repo_dir);

    println!("Ziel-User: {} → {}", username, claude.display());

    // Backup vorhandener Dateien
    backup_existing(&claude, config);

    let mut total = 0;

    for rule in &config.rules {
        let src  = repo.join(&rule.subdir);
        let dest = claude.join(&rule.dest_subdir);
        println!("{}:", rule.subdir);
        let n = sync_dir(&src, &dest, &rule.extension);
        println!("  {} Datei(en) installiert", n);
        total += n;
    }

    let src  = repo.join("CLAUDE.md");
    let dest = claude.join("CLAUDE.md");
    if src.exists() {
        println!("CLAUDE.md:");
        if copy_file(&src, &dest) {
            println!("    + CLAUDE.md");
            total += 1;
        }
    }

    println!("\nInstall abgeschlossen: {} Datei(en) für User '{}' installiert", total, username);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Verwendung: sync-claude <sync|install>");
        eprintln!("  sync    — .claude/ → Repo (sammeln)");
        eprintln!("  install — Repo → .claude/ (verteilen, mit Backup)");
        std::process::exit(1);
    }

    let config_path = {
        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let cwd_config = cwd.join("config.toml");
        if cwd_config.exists() {
            cwd_config
        } else {
            PathBuf::from(&args[0])
                .parent()
                .map(|p| p.join("config.toml"))
                .unwrap_or_else(|| PathBuf::from("config.toml"))
        }
    };

    let config_str = fs::read_to_string(&config_path)
        .unwrap_or_else(|e| panic!("config.toml nicht gefunden ({}): {}", config_path.display(), e));

    let config: Config = toml::from_str(&config_str)
        .expect("config.toml ungültig");

    match args[1].as_str() {
        "sync"    => run_sync(&config),
        "install" => run_install(&config),
        mode => {
            eprintln!("Unbekannter Modus: {}", mode);
            std::process::exit(1);
        }
    }
}

// EOF
