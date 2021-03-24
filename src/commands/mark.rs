use super::{clido_dir, ArgMatches, Context, Database, Result};

pub fn mark(sub_args: &ArgMatches<'_>) -> Result<()> {
    let id = sub_args
        .value_of("id")
        .with_context(|| "ID was not provided")?
        .parse::<usize>()
        .with_context(|| "Unable to parse ID")?;

    Database::from_path(clido_dir()?)?.mark_complete(id).save();

    Ok(())
}
