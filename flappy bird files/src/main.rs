mod components;
mod constants;
mod run;
mod update;
mod events;
mod render;
use crate::run::init_and_start_run;

fn main() -> Result<(), String> {
    init_and_start_run()?;
    Ok(())
}