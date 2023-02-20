use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_scripting::prelude::*;
use bevy_rapier3d::prelude::*;
use big_brain::prelude::*;

use spielplatz::view_3d::camera::{move_camera, CameraState};

fn main() {
    App::new()
        // Set up Bevy
        .add_plugins(DefaultPlugins)
        // Add Lua scripting support
        .add_plugin(ScriptingPlugin)
        .add_script_host::<LuaScriptHost<()>, _>(CoreStage::PostUpdate)
        .add_script_handler_stage::<LuaScriptHost<()>, _, 0, 0>(CoreStage::PostUpdate)
        // Add utility AI support (big-brain)
        .add_plugin(BigBrainPlugin)
        // Add physics support (Rapier)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(configure_rapier_physics_plugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        // Add utility plugins for development
        .add_plugin(WorldInspectorPlugin)
        // Add game logic
        .init_resource::<CameraState>()
        .add_system(move_camera)
        // Run the game
        .run();
}

fn configure_rapier_physics_plugin(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.timestep_mode = TimestepMode::Interpolated {
        dt: 1.0 / 60.0,
        time_scale: 1.0,
        substeps: 1,
    };
}
