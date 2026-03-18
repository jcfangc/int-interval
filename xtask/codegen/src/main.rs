mod signed;
mod unsigned;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

const UNSIGNED_TYPES: &[(&str, &str)] = &[
    ("u16", "U16CO"),
    ("u32", "U32CO"),
    ("u64", "U64CO"),
    ("u128", "U128CO"),
    ("usize", "UsizeCO"),
];

const SIGNED_TYPES: &[(&str, &str)] = &[
    ("i16", "I16CO"),
    ("i32", "I32CO"),
    ("i64", "I64CO"),
    ("i128", "I128CO"),
    ("isize", "IsizeCO"),
];

fn main() {
    let mode = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("missing mode, expected one of: `signed`, `unsigned`, `all`"));

    let root = workspace_root();
    let src = root.join("src");

    match mode.as_str() {
        "signed" => signed::generate(&src),
        "unsigned" => unsigned::generate(&src),
        "all" => {
            signed::generate(&src);
            unsigned::generate(&src);
        }
        other => panic!("unsupported mode: {other}, expected one of: `signed`, `unsigned`, `all`"),
    }

    write_lib(&src);
}

/// workspace root
fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent() // xtask
        .unwrap()
        .parent() // workspace root
        .unwrap()
        .to_path_buf()
}

fn write_lib(src: &Path) {
    let mut s = String::new();

    s.push_str("#![no_std]\n");
    s.push_str("#[cfg(test)]\n");
    s.push_str("extern crate std;\n\n");

    s.push_str("mod res;\n");

    s.push_str("mod u8;\n");
    s.push_str("mod i8;\n");

    for (ty, _) in UNSIGNED_TYPES {
        s.push_str(&format!("mod {ty};\n"));
    }
    for (ty, _) in SIGNED_TYPES {
        s.push_str(&format!("mod {ty};\n"));
    }

    s.push('\n');

    s.push_str("pub use res::{OneTwo, ZeroOneTwo};\n");
    s.push_str("pub use u8::U8CO;\n");
    s.push_str("pub use i8::I8CO;\n");

    for (ty, name) in UNSIGNED_TYPES {
        s.push_str(&format!("pub use {ty}::{name};\n"));
    }
    for (ty, name) in SIGNED_TYPES {
        s.push_str(&format!("pub use {ty}::{name};\n"));
    }

    fs::write(src.join("lib.rs"), s).unwrap();
}
