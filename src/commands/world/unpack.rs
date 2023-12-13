use anyhow::Result;

use crate::app::App;

#[derive(clap::Args)]
pub struct Args {
    /// The world to unpack
    world: Option<String>,
}

pub fn run(app: &mut App, args: Args) -> Result<()> {
    let world_name = if let Some(s) = args.world {
        s
    } else {
        app.select_world("Unpack...")?
    };

    app.worlds().unpack(&world_name)?;

    Ok(())
}
