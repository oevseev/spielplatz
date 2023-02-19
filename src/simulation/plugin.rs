use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::simulation::systems::run_when_physics_pending;

#[derive(StageLabel)]
enum SimulationStages {
    StepSimulation
}
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

        app.add_stage_after(
            CoreStage::Update,
            SimulationStages::StepSimulation,
            SystemStage::parallel().with_system_set(
                SystemSet::new()
                    .with_run_criteria(run_when_physics_pending)
            )
        );
    }
}
