use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct SimulationPlugin {
    pub dt: f32,
}

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default());

        if let Some(mut rapier_configuration) = app.world.get_resource_mut::<RapierConfiguration>()
        {
            rapier_configuration.timestep_mode = TimestepMode::Interpolated {
                dt: self.dt,
                time_scale: 1.0,
                substeps: 1,
            };
        }
    }
}
