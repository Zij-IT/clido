use super::{clido_dir, ArgMatches, Context, Database, Result};

pub fn del(sub_args: &ArgMatches<'_>) -> Result<()> {
    let id = sub_args
        .value_of("id")
        .with_context(|| "DEL_ID was not provided")?
        .parse::<usize>()
        .with_context(|| "Unable to parse DEL_ID")?;

    Database::from_path(clido_dir()?)?.delete(id).save();

    Ok(())
}
