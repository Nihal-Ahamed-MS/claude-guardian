use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn install() -> Result<()> {
    let binary = current_binary()?;
    platform_install(&binary)
}

pub fn uninstall() -> Result<()> {
    platform_uninstall()
}

fn current_binary() -> Result<PathBuf> {
    std::env::current_exe().context("could not determine binary path")
}

// ─── macOS ───────────────────────────────────────────────────────────────────

#[cfg(target_os = "macos")]
fn platform_install(binary: &std::path::Path) -> Result<()> {
    let plist_path = launchd_plist_path()?;
    let plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
            "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
            <plist version="1.0">
            <dict>
                <key>Label</key>
                <string>com.llm-observer.daemon</string>
                <key>ProgramArguments</key>
                <array>
                    <string>{binary}</string>
                    <string>run</string>
                </array>
                <key>RunAtLoad</key>
                <true/>
                <key>KeepAlive</key>
                <true/>
                <key>StandardOutPath</key>
                <string>/tmp/llm-observer.log</string>
                <key>StandardErrorPath</key>
                <string>/tmp/llm-observer.log</string>
            </dict>
            </plist>
            "#,
        binary = binary.display()
    );

    if let Some(parent) = plist_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&plist_path, plist)?;

    Command::new("launchctl")
        .args(["load", "-w", &plist_path.to_string_lossy()])
        .status()
        .context("launchctl load failed")?;

    tracing::info!("launchd agent installed at {}", plist_path.display());
    Ok(())
}

#[cfg(target_os = "macos")]
fn platform_uninstall() -> Result<()> {
    let plist_path = launchd_plist_path()?;
    if plist_path.exists() {
        Command::new("launchctl")
            .args(["unload", "-w", &plist_path.to_string_lossy()])
            .status()
            .context("launchctl unload failed")?;
        fs::remove_file(&plist_path)?;
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn launchd_plist_path() -> Result<PathBuf> {
    let home = std::env::var("HOME").context("HOME not set")?;
    Ok(PathBuf::from(home)
        .join("Library/LaunchAgents/com.llm-observer.daemon.plist"))
}