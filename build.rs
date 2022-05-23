use cc::windows_registry;
use std::{env, path::Path};

fn main() {
    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    add_spectre_link_search();
}

/// Windows requires additional steps to find the spectre-mitigated CRT libs.
/// More info: https://docs.microsoft.com/en-us/cpp/build/reference/qspectre
fn add_spectre_link_search() {
    let target = env::var("TARGET").expect("missing TARGET");
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("missing CARGO_CFG_TARGET_ARCH");
    let arch = match arch.as_str() {
        "x86_64" => "x64",
        "x86" => "x86",
        "aarch64" => "arm64",
        "arm" => "arm32",
        _ => panic!("unsupported arch: {arch}"),
    };

    let tool = windows_registry::find_tool(&target, "cl.exe").expect("couldn't find cl.exe");
    let spectre_libs = tool.path().join(format!(r"..\..\..\..\lib\spectre\{arch}"));

    if Path::new(&spectre_libs).exists() {
        println!(
            "cargo:rustc-link-search=native={}",
            spectre_libs.into_os_string().into_string().unwrap()
        );
    } else {
        println!("cargo:warning=No spectre-mitigated libs were found. Please modify the VS Installation to add these.");

        #[cfg(feature = "error")]
        {
            panic!("No spectre-mitigated libs were found. Please modify the VS Installation to add these.");
        }
    }
}
