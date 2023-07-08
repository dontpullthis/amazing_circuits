/// General systems, like setup
pub mod toolbar;

use bevy::prelude::*;

use crate::components::{FONT, NORMAL_BUTTON};
use crate::components::toolbar as toolbar_components;

/// Sets up components in the main window
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        }, toolbar_components::Toolbar{}))
        .with_children(|parent| {
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                }, toolbar_components::ButtonAddResistor{}))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Res+",
                        TextStyle {
                            font: asset_server.load(FONT),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                }, toolbar_components::ButtonAddCapacitor{}))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Cap+",
                        TextStyle {
                            font: asset_server.load(FONT),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
