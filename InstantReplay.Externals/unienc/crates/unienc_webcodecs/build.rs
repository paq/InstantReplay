use std::fs;

fn main() {
    let _ = std::process::Command::new("tsc")
        .current_dir(fs::canonicalize("./src/js").unwrap())
        .status()
        .expect("failed to execute tsc");
}
