use std::process::Command;

fn main() {
    // https://stackoverflow.com/a/44407625
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    // Get compilation date / time
    let dt_local = chrono::Local::now();
    let naive_utc = dt_local.naive_utc();
    let formatted = naive_utc.format("%Y-%m-%d %H:%M:%S");
    println!("cargo:rustc-env=NAMIDA_COMPILE_DT={} UTC", formatted);
}
