use bevy::{prelude::*, time::FixedTimestep};
use bevy_rapier3d::prelude::*;

use crate::simulation::systems::{run_if_physics_pending, step_simulation, unset_physics_pending};

#[derive(Default, Resource)]
pub struct SimulationState {
    pub(crate) physics_pending: bool,
}

#[derive(StageLabel)]
enum SimulationStages {
    StepSimulation,
    PostPhysics,
}

pub struct SimulationPlugin {
    pub dt: f32,
}

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationState>();

        app.add_plugin(
            RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false),
        );

        // TODO: Implement interpolation (https://gafferongames.com/post/fix_your_timestep/)
        if let Some(mut rapier_configuration) = app.world.get_resource_mut::<RapierConfiguration>()
        {
            rapier_configuration.timestep_mode = TimestepMode::Fixed {
                dt: self.dt,
                substeps: 1,
            };
        }

        // CoreStage::Update -> SimulationStages::StepSimulation -> SimulationStages::SyncBackend -> SimulationStages::Writeback
        app.add_stage_after(
            CoreStage::Update,
            SimulationStages::StepSimulation,
            SystemStage::parallel().with_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(self.dt.into()))
                    .with_system(step_simulation),
            ),
        );
        app.add_stage_after(
            SimulationStages::StepSimulation,
            PhysicsStages::SyncBackend,
            SystemStage::parallel()
                .with_run_criteria(run_if_physics_pending)
                .with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
                    PhysicsStages::SyncBackend,
                )),
        );
        app.add_stage_after(
            PhysicsStages::SyncBackend,
            PhysicsStages::StepSimulation,
            SystemStage::parallel()
                .with_run_criteria(run_if_physics_pending)
                .with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
                    PhysicsStages::StepSimulation,
                )),
        );
        app.add_stage_after(
            PhysicsStages::StepSimulation,
            PhysicsStages::Writeback,
            SystemStage::parallel()
                .with_run_criteria(run_if_physics_pending)
                .with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
                    PhysicsStages::Writeback,
                )),
        );

        // PhysicsStages::DetectDespawn -> SimulationStages::PostPhysics -> CoreStage::Last
        app.add_stage_before(
            CoreStage::Last,
            SimulationStages::PostPhysics,
            SystemStage::parallel()
                .with_run_criteria(run_if_physics_pending)
                .with_system(unset_physics_pending),
        );
        app.add_stage_before(
            SimulationStages::PostPhysics,
            PhysicsStages::DetectDespawn,
            SystemStage::parallel()
                .with_run_criteria(run_if_physics_pending)
                .with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
                    PhysicsStages::DetectDespawn,
                )),
        );

        app.add_plugin(RapierDebugRenderPlugin::default());
    }
}
