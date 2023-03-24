use std::{env, process::Command};

fn main() {
    env::set_var("LD_PRELOAD", "./sloppy/target/debug/libsloppy.so");

    let out = Command::new("ping")
        .arg("google.com")
        .spawn()
        .expect("could not spawn cmd");

    println!("hey");
}
