use bevy::{prelude::*, ecs::schedule::ShouldRun};
use bevy_rapier3d::prelude::*;

pub(crate) fn run_when_physics_pending(
    time: Res<Time>,
    rapier_configuration: Res<RapierConfiguration>,
    sim_to_render_time: Res<SimulationToRenderTime>,
) -> ShouldRun {
    if let TimestepMode::Interpolated { dt, .. } = rapier_configuration.timestep_mode {
        // Two assumptions here:
        // - time is updated in CoreStage::Update and thus will be the same in the later stages,
        // including SimulationStages::StepSimulation and the PhysicsStages::StepSimulation
        // - the relative ordering of the stages is CoreStage::Update ->
        // SimulationStages::StepSimulation -> PhysicsStages::StepSimulation
        //
        // For more information take a look at step_simulation in bevy_rapier/src/plugin/context.rs

        if sim_to_render_time.diff + time.delta_seconds() > dt {
            ShouldRun::Yes
        } else {
            ShouldRun::No
        }
    } else {
        panic!("can't use run_when_system_pending with timestep_mode other than TimestepMode::Interpolated");
    }
}
