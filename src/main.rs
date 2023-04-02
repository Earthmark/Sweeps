#![allow(clippy::type_complexity)]

use attractor::*;
use bevy::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::*;

mod attractor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_system(setup.on_startup())
        .add_system(setup_menu.in_schedule(OnEnter(GameState::MainMenu)))
        .add_system(cleanup_menu.in_schedule(OnExit(GameState::MainMenu)))
        .add_system(update_menu.in_set(OnUpdate(GameState::MainMenu)))
        .add_system(setup_sim.in_schedule(OnEnter(GameState::Simulation)))
        .add_system(cleanup_sim.in_schedule(OnExit(GameState::Simulation)))
        .add_system(
            update_node_positions::<LorenzAttractor>.in_set(OnUpdate(GameState::Simulation)),
        )
        .add_system(
            update_node_positions::<ThreeCellCnnAttractor>.in_set(OnUpdate(GameState::Simulation)),
        )
        .add_system(
            update_node_positions::<AizawaAttractor>.in_set(OnUpdate(GameState::Simulation)),
        )
        .add_system(
            update_node_positions::<BoualiAttractor>.in_set(OnUpdate(GameState::Simulation)),
        )
        .add_system(
            update_node_positions::<ChenLeeAttractor>.in_set(OnUpdate(GameState::Simulation)),
        )
        .add_system(
            update_node_positions::<HalvorsenAttractor>.in_set(OnUpdate(GameState::Simulation)),
        )
        .add_system(
            update_node_positions::<FinanceAttractor>.in_set(OnUpdate(GameState::Simulation)),
        )
        .add_system(
            update_node_positions::<NewtonLeipnikAttractor>.in_set(OnUpdate(GameState::Simulation)),
        )
        .add_system(
            update_node_positions::<NoseHooverAttractor>.in_set(OnUpdate(GameState::Simulation)),
        )
        .add_system(
            update_node_positions::<ThomasAttractor>.in_set(OnUpdate(GameState::Simulation)),
        )
        .run();
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    Simulation,
}

fn setup(mut commands: Commands) {
    //commands.spawn(Camera2dBundle::default());
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, -20.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });
}

#[derive(Resource)]
struct Menu {
    root: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .id();
    commands.insert_resource(Menu { root });
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<Menu>) {
    commands.entity(menu_data.root).despawn_recursive();
}

fn update_menu(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameState::Simulation);
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

#[derive(Resource)]
struct Sim {
    root: Entity,
}

fn setup_sim(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(shape::Icosphere::default().try_into().unwrap());
    let material = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    let mut rng = rand::thread_rng();
    let mut rand_gen = || -> f32 { rng.sample(Uniform::new(-20., 20.)) };
    let mut rand_v3 = || -> Vec3 {
        Vec3 {
            x: rand_gen(),
            y: rand_gen(),
            z: rand_gen(),
        }
    };
    let root = commands
        .spawn(SpatialBundle::default())
        .with_children(|parent| {
            parent.spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 1500.0,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_xyz(4.0, 8.0, 4.0),
                ..default()
            });
            for _ in 0..1000 {
                parent.spawn((
                    PbrBundle {
                        mesh: mesh.clone(),
                        material: material.clone(),
                        transform: Transform {
                            translation: rand_v3(),
                            scale: Vec3::splat(0.05),
                            ..default()
                        },
                        ..default()
                    },
                    ChenLeeAttractor::default(),
                ));
            }
        })
        .id();
    commands.insert_resource(Sim { root });
}

fn cleanup_sim(mut commands: Commands, sim: Res<Sim>) {
    commands.entity(sim.root).despawn_recursive();
}
