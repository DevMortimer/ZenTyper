mod input;
mod program;
mod words;

use crate::input::InputPlugin;
use crate::words::Words;
use bevy::prelude::*;
use program::ProgramPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Zen Typer".to_string(),
            width: 840.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(Words::new())
        .add_plugin(ProgramPlugin)
        .add_plugin(InputPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
