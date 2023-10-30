use std::env;

fn main() {
    if let Ok(version) = env::var("DEP_CARES_VERSION_NUMBER") {
        let version = u64::from_str_radix(&version, 16).unwrap();

        if version >= 0x1_0f_00 {
            // 1.15.0
            println!("cargo:rustc-cfg=cares115");
        }

        if version >= 0x1_11_00 {
            // 1.17.0
            println!("cargo:rustc-cfg=cares117");
        }

        if version >= 0x1_13_00 {
            // 1.19.0
            println!("cargo:rustc-cfg=cares119");
        }

        if version >= 0x1_14_00 {
            // 1.20.0
            println!("cargo:rustc-cfg=cares120");
        }
    }
}
