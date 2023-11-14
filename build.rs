use std::env;

fn main() {
    if let Ok(version) = env::var("DEP_CARES_VERSION_NUMBER") {
        let version = u64::from_str_radix(&version, 16).unwrap();

        if version >= 0x1_0f_00 {
            println!("cargo:rustc-cfg=cares1_15");
        }

        if version >= 0x1_11_00 {
            println!("cargo:rustc-cfg=cares1_17");
        }

        if version >= 0x1_13_00 {
            println!("cargo:rustc-cfg=cares1_19");
        }

        if version >= 0x1_14_00 {
            println!("cargo:rustc-cfg=cares1_20");
        }

        if version >= 0x1_16_00 {
            println!("cargo:rustc-cfg=cares1_22");
        }
    }
}
