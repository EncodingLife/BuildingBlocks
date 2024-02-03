use bevy::prelude::*;

use crate::mods::{cell::stem::Stem, map::map_position::MapPosition, organism::Organism};

pub struct SimulationInfoPlugin;

impl Plugin for SimulationInfoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationDiagnosticInfo::default())
            .add_systems(Startup, setup_simulation_info)
            .add_systems(Update, (count_organisms, count_instructions))
            .add_systems(PostUpdate, (update_org_text, update_inst_text));
    }
}

#[derive(Resource, Default)]
pub struct SimulationDiagnosticInfo {
    org_count: usize,
    instruction_count: usize,
}

#[derive(Component)]
struct SimulationInfo;

#[derive(Component)]
struct OrgCount;

#[derive(Component)]
struct InstructionCount;

pub(super) fn setup_simulation_info(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            SimulationInfo,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Auto,
                    top: Val::Percent(1.),
                    bottom: Val::Auto,
                    left: Val::Percent(1.),
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let org_count_text = commands
        .spawn((
            OrgCount,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "Organisms: \t".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::PURPLE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let instruction_count_text = commands
        .spawn((
            InstructionCount,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "Instructions: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands
        .entity(root)
        .push_children(&[org_count_text, instruction_count_text]);
}

pub(super) fn update_org_text(
    diagnostics: Res<SimulationDiagnosticInfo>,
    mut query: Query<&mut Text, With<OrgCount>>,
) {
    for mut text in &mut query {
        // Format the number as to leave space for 4 digits, just in case,
        // right-aligned and rounded. This helps readability when the
        // number changes rapidly.
        text.sections[1].value = format!("{:>4.0}", diagnostics.org_count);
    }
}

pub(super) fn update_inst_text(
    diagnostics: Res<SimulationDiagnosticInfo>,
    mut query: Query<&mut Text, With<InstructionCount>>,
) {
    for mut text in &mut query {
        // Format the number as to leave space for 4 digits, just in case,
        // right-aligned and rounded. This helps readability when the
        // number changes rapidly.
        text.sections[1].value = format!("{:>4.0}", diagnostics.instruction_count);
    }
}

pub fn count_instructions(
    mut query: Query<&Stem>,
    mut simulation_diagnostics: ResMut<SimulationDiagnosticInfo>,
) {
    // if !query.is_empty() {
    //     println!("Executing instructions for {} stem cells", query.iter().count())
    // }
    simulation_diagnostics.instruction_count = query.iter().count();
}

pub fn count_organisms(
    mut query: Query<&Organism>,
    mut simulation_diagnostics: ResMut<SimulationDiagnosticInfo>,
) {
    // if !query.is_empty() {
    //     println!("Executing instructions for {} stem cells", query.iter().count())
    // }
    simulation_diagnostics.org_count = query.iter().count();
}
