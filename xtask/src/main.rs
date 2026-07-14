use std::{fs, path::Path, time::Instant};

use anyhow::bail;
use itertools::Itertools;

const MAIN_RS_CONTENTS: &str = r#"fn main() {
    println!("hello world");
}"#;

fn create_crate(name: &str) -> anyhow::Result<()> {
    let crate_dir = Path::new("crates").join(name);
    fs::create_dir(&crate_dir)?;

    let cargo_toml_contents = format!(
        r#"[package]
name = "{}"
license.workspace = true
edition.workspace = true
publish.workspace = true"#,
        name,
    );
    fs::write(crate_dir.join("Cargo.toml"), cargo_toml_contents)?;

    let src_dir = crate_dir.join("src");
    fs::create_dir(&src_dir)?;

    fs::write(src_dir.join("main.rs"), MAIN_RS_CONTENTS)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let Some(max_name_len) = std::env::args().nth(1) else {
        bail!("please specify max crate name length as argument!!");
    };

    let mut num_crates_generated = 0u64;

    let before = Instant::now();

    let _ = fs::remove_dir_all("crates");
    fs::create_dir("crates")?;

    for len in 1..=max_name_len.parse()? {
        for name_ascii_bytes in std::iter::repeat(b'a'..b'z')
            .take(len)
            .multi_cartesian_product()
        {
            let name = str::from_utf8(&name_ascii_bytes)?;
            create_crate(name)?;
            num_crates_generated += 1;
        }
    }

    println!(
        "generated {} crates in {:?}",
        num_crates_generated,
        before.elapsed()
    );

    Ok(())
}
