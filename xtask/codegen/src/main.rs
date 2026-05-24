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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum EmitMode {
    Write,
    Check,
}

impl EmitMode {
    #[inline]
    fn is_check(self) -> bool {
        matches!(self, Self::Check)
    }
}

fn main() {
    let mut args = env::args().skip(1);

    let mode = args.next().unwrap_or_else(|| {
        panic!("missing mode, expected one of: `signed`, `unsigned`, `all`, `check`")
    });

    let mut emit = EmitMode::Write;

    let target = match mode.as_str() {
        "signed" | "unsigned" | "all" => mode.as_str(),
        "check" => {
            emit = EmitMode::Check;
            "all"
        }
        other => panic!(
            "unsupported mode: {other}, expected one of: `signed`, `unsigned`, `all`, `check`"
        ),
    };

    for arg in args {
        match arg.as_str() {
            "--check" | "check" => emit = EmitMode::Check,
            other => panic!("unsupported arg: {other}, expected `--check`"),
        }
    }

    let root = workspace_root();
    let src = root.join("src");

    match target {
        "signed" => signed::generate(&src, emit),
        "unsigned" => unsigned::generate(&src, emit),
        "all" => {
            signed::generate(&src, emit);
            unsigned::generate(&src, emit);
        }
        _ => unreachable!(),
    }

    write_lib(&src, emit);

    if emit.is_check() {
        eprintln!("codegen check passed");
    }
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

pub(crate) fn emit_file(path: impl AsRef<Path>, expected: String, mode: EmitMode) {
    let path = path.as_ref();

    match mode {
        EmitMode::Write => {
            fs::write(path, expected).unwrap();
        }
        EmitMode::Check => {
            let actual = fs::read_to_string(path).unwrap_or_else(|err| {
                panic!(
                    "generated file is missing or unreadable: {}\nerror: {err}",
                    path.display()
                )
            });

            if actual != expected {
                panic!(
                    "generated file is stale: {}\nrun codegen and commit the updated output",
                    path.display()
                );
            }
        }
    }
}

fn write_lib(src: &Path, mode: EmitMode) {
    let mut s = String::new();

    s.push_str("#![no_std]\n");
    s.push_str("#[cfg(test)]\n");
    s.push_str("extern crate std;\n\n");

    s.push_str("pub mod traits;\n");
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

    emit_file(src.join("lib.rs"), s, mode);
}
