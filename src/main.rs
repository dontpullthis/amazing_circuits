mod components;
mod systems;

use bevy::{prelude::*, winit::WinitSettings};

use crate::systems::setup;
use crate::systems::toolbar as toolbar_systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            toolbar_systems::btn_add_capacitor_handler,
            toolbar_systems::btn_add_resistor_handler
        ))
        .run();
}