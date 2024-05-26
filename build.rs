use std::env;

fn main() {
    if let Ok(version) = env::var("DEP_CARES_VERSION_NUMBER") {
        let version = u64::from_str_radix(&version, 16).unwrap();

        println!("cargo::rustc-check-cfg=cfg(cares1_15)");
        if version >= 0x1_0f_00 {
            println!("cargo:rustc-cfg=cares1_15");
        }

        println!("cargo::rustc-check-cfg=cfg(cares1_17)");
        if version >= 0x1_11_00 {
            println!("cargo:rustc-cfg=cares1_17");
        }

        println!("cargo::rustc-check-cfg=cfg(cares1_19)");
        if version >= 0x1_13_00 {
            println!("cargo:rustc-cfg=cares1_19");
        }

        println!("cargo::rustc-check-cfg=cfg(cares1_20)");
        if version >= 0x1_14_00 {
            println!("cargo:rustc-cfg=cares1_20");
        }

        println!("cargo::rustc-check-cfg=cfg(cares1_22)");
        if version >= 0x1_16_00 {
            println!("cargo:rustc-cfg=cares1_22");
        }

        println!("cargo::rustc-check-cfg=cfg(cares1_23)");
        if version >= 0x1_17_00 {
            println!("cargo:rustc-cfg=cares1_23");
        }

        println!("cargo::rustc-check-cfg=cfg(cares1_24)");
        if version >= 0x1_18_00 {
            println!("cargo:rustc-cfg=cares1_24");
        }

        println!("cargo::rustc-check-cfg=cfg(cares1_29)");
        if version >= 0x1_1d_00 {
            println!("cargo:rustc-cfg=cares1_29");
        }
    }
}
