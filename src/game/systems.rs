use bevy::prelude::*;

use super::SimulationState;

pub fn toogle_similation(
    mut commnds: Commands,
    keboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keboard_input.just_pressed(KeyCode::Space) {
        if *simulation_state.get() == SimulationState::Running {
            commnds.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Simulation Paused.")
        }
        if *simulation_state.get() == SimulationState::Paused {
            commnds.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Simulation Running.")
        }
    }
}
