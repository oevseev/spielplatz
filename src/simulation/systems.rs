use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::simulation::plugin::SimulationState;

pub(crate) fn run_if_physics_pending(simulation_state: Res<SimulationState>) -> ShouldRun {
    if simulation_state.physics_pending {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub(crate) fn unset_physics_pending(mut simulation_state: ResMut<SimulationState>) {
    simulation_state.physics_pending = false
}

pub(crate) fn step_simulation(mut simulation_state: ResMut<SimulationState>) {
    simulation_state.physics_pending = true
}
