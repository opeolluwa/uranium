// import the command module form the standard library
use std::process::Command;

fn main() {
    shell("cd ../nitride-ui &&  npm run build:ssr")
}

fn shell(command: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect(format!("Failed to run {cmd}", cmd = command).as_str());

    println!("{:#?}", output);
}
