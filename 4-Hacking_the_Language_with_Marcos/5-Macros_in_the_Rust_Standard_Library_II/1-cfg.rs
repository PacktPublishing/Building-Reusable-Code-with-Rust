fn main() {
    if cfg!(target_os = "macos") {
        println!("Run the MacOS-specific code");
    }
    else if cfg!(target_os = "linux"){
        println!("Run the Linux-specific code");
        LinuxCode {};
    }
    else {
        println!("We don't support this architecture");
    }
}

#[cfg(target_os = "linux")]
struct LinuxCode;
