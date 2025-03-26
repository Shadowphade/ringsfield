use bevy::prelude::*;
use std::f32::consts::PI;
use ringsfield::player::PlayerPlugin;
use noise::{BasicMulti, NoiseFn, Perlin};

use bevy::{
    pbr::CascadeShadowConfigBuilder, prelude::*, scene::SceneInstanceReady, render::mesh::VertexAttributeValues,
};

const MODEL_PATH: &str = "robot.glb";

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_mesh)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup_env)
        .add_systems(Update, toggle_terrain)


        .run();
}

#[derive(Component)]
struct AnimationToPlay {
    graph_handle: Handle<AnimationGraph>,
    index: AnimationNodeIndex,
}


fn setup_mesh(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let (graph, index) = AnimationGraph::from_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(MODEL_PATH)),
    );

    let graph_handle = graphs.add(graph);

    let animation_to_play = AnimationToPlay {
        graph_handle,
        index,
    };

    let mesh_scene = SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(MODEL_PATH)));

    commands
        .spawn((animation_to_play, mesh_scene))
        .observe(play_animation_when_ready);
}

fn play_animation_when_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animations_to_play: Query<&AnimationToPlay>,
    mut players: Query<&mut AnimationPlayer>,
) {
    if let Ok(animation_to_play) = animations_to_play.get(trigger.entity()) {
        for child in children.iter_descendants(trigger.entity()) {
            if let Ok(mut player) = players.get_mut(child) {
                player.play(animation_to_play.index).repeat();

                commands.entity(child).insert(AnimationGraphHandle(animation_to_play.graph_handle.clone()));
            }
        }
    }
}

// should prob refactor the code and create a env file?!?
#[derive(Component)]
struct Terrain;
#[derive(Component)]
struct FlatGround;

fn setup_env (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    
    let grass_texture = asset_server.load("textures/grass.png");

    let terrain_material = materials.add(StandardMaterial {
        base_color_texture: Some(grass_texture),
        perceptual_roughness: 0.9,
        reflectance: 0.2,
        ..default()
    });

    // for now im keeping the flat plane setup and adding procedural terrain
    // you can toggle between them pressing 'T'
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5000.0, 5000.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 0.0))),
        FlatGround,
    ));

    
    let terrain_height = 70.;
    let noise = BasicMulti::<Perlin>::new(900);
    let mut terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(1000.0, 1000.0)
            .subdivisions(200),
    );

    if let Some(VertexAttributeValues::Float32x3(positions)) = terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        for pos in positions.iter_mut() {
            let val = noise.get([
                pos[0] as f64 / 300.,
                pos[2] as f64 / 300.,
            ]);
            pos[1] = val as f32 * terrain_height;
        }
    }

    terrain.compute_normals();

    commands.spawn((
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(terrain_material),
        Terrain,
    ));

    
    commands.spawn((
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .build(),
    ));
}

fn toggle_terrain(
    mut commands: Commands,
    flat_ground: Query<Entity, With<FlatGround>>,
    terrain: Query<Entity, With<Terrain>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyT) {
        for entity in &flat_ground {
            commands.entity(entity).despawn();
        }
        for entity in &terrain {
            commands.entity(entity).despawn();
        }
    }
}
