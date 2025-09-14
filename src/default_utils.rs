use crate::{config::CONFIG, program::Program};

pub fn matcher(program: Program) -> anyhow::Result<Vec<u8>> {
    let default_name_and_args = program.get_default_name_and_args()?;
    match default_name_and_args[0].as_str() {
        "ive" => ive(default_name_and_args),
        _ => unreachable!(),
    }
}

fn ive(args: Vec<String>) -> anyhow::Result<Vec<u8>> {
    if args.len() != 2 {
        anyhow::bail!(format!("Incorrect number of arguments: {}", args.join(" ")))
    }

    match args[1].as_str() {
        "on" => CONFIG.set_interactive(true)?,
        "off" => CONFIG.set_interactive(false)?,
        _ => anyhow::bail!(format!("Wrong argument: {}", args.join(" "))),
    }

    Ok(vec![])
}
