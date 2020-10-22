#[cfg(target_os = "windows")]
use winres;
use std::path::Path;
use std::{env, fs};

const CONFIG_FILE: &str = "config.txt";

fn copy<S: AsRef<std::ffi::OsStr> + ?Sized, P: Copy + AsRef<Path>>(target_dir_path: &S, file_name: P) {
    let path = Path::new(&target_dir_path)
        .join(r"..\..\..\")
        .join(file_name);
    fs::copy(file_name, path).unwrap();
}

fn copy_local_files() {
    let target_dir_path = env::var("OUT_DIR").unwrap();
    copy(&target_dir_path, CONFIG_FILE);
}

fn append_windows_resources() {
    use std::io::Write;
    let mut res = winres::WindowsResource::new();
    res.set_manifest(
        r#"
        <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
        <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
            <security>
                <requestedPrivileges>
                    <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
                </requestedPrivileges>
            </security>
        </trustInfo>
        </assembly>
        "#);
    match res.compile() {
        Err(error) => {
            write!(std::io::stderr(), "{}", error).unwrap();
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}

#[cfg(target_os = "windows")]
fn main() {
    println!("Building! ");
    copy_local_files();
    append_windows_resources();
}

#[cfg(not(target_os = "windows"))]
fn main() {}