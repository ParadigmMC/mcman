use anyhow::Result;
use crate::app::App;

#[derive(clap::Args)]
pub struct Args {
    /// The world to pack - packs every world if not present
    world: Option<String>,
}

pub fn run(app: &mut App, args: Args) -> Result<()> {
    let world_name = if let Some(s) = args.world {
        s
    } else {
        app.select_world("Pack...")?
    };

    app.worlds().pack(&world_name)?;

    Ok(())
}
