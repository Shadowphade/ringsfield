use bevy::prelude::*;
use crate::player_controller::*;


//Uncomment when the Player component is needed
//#[derive(Component)]
//pub struct Player {
//    pub health: f32,
//}

pub fn setup_player(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Controller,
        Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseSettings>()
            .init_resource::<KeyBinds>()
            .add_systems(Startup, setup_player)
            .add_systems(Startup, init_cursor_grab)
            .add_systems(Update, player_move)
            .add_systems(Update, player_look)
            .add_systems(Update, grab_cursor);
    }
}
