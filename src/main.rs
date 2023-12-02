use itertools::Itertools;
use std::{error::Error, fs, iter, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    let days = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/"))?
        .filter_map(|p| p.ok()?.path().file_stem()?.to_str().map(str::to_string))
        .sorted()
        .collect::<Vec<_>>();

    for day in &days {
        let mut cmd = Command::new("cargo");
        cmd.args(["run", "--release", "--bin", day]);
        println!(
            "Running {:?}",
            iter::once(cmd.get_program()).chain(cmd.get_args())
        );

        let result = cmd.output()?;
        let output = String::from_utf8(result.stdout)?;
        println!("result {}:\n{}", day, output);
    }

    Ok(())
}
