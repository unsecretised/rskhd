use std::{fs, path::PathBuf, process::Command};

const SERVICE_PLIST: &str = include_str!("../rskhd_service.plist");
const LABEL: &str = "com.umangsurana.rskhd";

fn plist_path() -> PathBuf {
    std::env::home_dir()
        .expect("Unable to get home dir")
        .join("Library/LaunchAgents")
        .join(format!("{LABEL}.plist"))
}

fn domain() -> String {
    format!("gui/{}", unsafe { libc::getuid() })
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn launchctl(args: &[&str]) -> bool {
    Command::new("launchctl")
        .args(args)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn install() {
    let path = plist_path();
    let bin = std::env::current_exe().expect("Unable to get self binary path");

    fs::create_dir_all(path.parent().unwrap()).expect("Unable to create LaunchAgents dir");

    let plist = SERVICE_PLIST
        .replace("{{LABEL}}", LABEL)
        .replace("{{BIN}}", &xml_escape(&bin.to_string_lossy()));
    fs::write(&path, plist).expect("Unable to write launch agent");

    println!("Installed and started launch agent at {}", path.display());
}

pub fn uninstall() {
    let path = plist_path();

    launchctl(&["bootout", &format!("{}/{LABEL}", domain())]);

    if path.exists() {
        fs::remove_file(&path).expect("File deletion error");
        println!("Uninstalled service successfully");
    } else {
        eprintln!("Launch agent file doesn't exist");
    }
}

pub fn start() {
    let path = plist_path();
    if !path.exists() {
        eprintln!("Launch agent not installed. Run \"rskhd install\" first.");
        return;
    }
    if !launchctl(&["bootstrap", &domain(), &path.to_string_lossy()]) {
        eprintln!("Failed to start (it may already be running)");
    }
}

pub fn stop() {
    if !launchctl(&["bootout", &format!("{}/{LABEL}", domain())]) {
        eprintln!("Failed to stop (it may not be running)");
    }
}
