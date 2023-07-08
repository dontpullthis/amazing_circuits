use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::components::{NORMAL_BUTTON, HOVERED_BUTTON};
use crate::components::toolbar as toolbar_components;
use crate::components::desktop as desktop_components;

pub fn btn_add_resistor_handler(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<toolbar_components::ButtonAddResistor>),
    >,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let window = window_query.get_single().unwrap();
                let mut rng = rand::thread_rng();
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            rng.gen_range(0.0..window.width()) - (window.width() / 2.0),
                            rng.gen_range(0.0..window.height()) - (window.height() / 2.0),
                            0.0),
                        texture: asset_server.load("sprites/components/passive/resistor.png"),
                        ..default()
                    },
                    desktop_components::Resistor{},
                ));
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn btn_add_capacitor_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<toolbar_components::ButtonAddCapacitor>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}