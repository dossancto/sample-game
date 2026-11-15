use bevy::prelude::*;
use bevy_ahoy::prelude::*;
use bevy_enhanced_input::prelude::*;

use interaction_gateway::actions::InteractionAction;
use player_gateway::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<Player>();
    app.add_observer(setup_player_actions);
}

fn setup_player_actions(on: On<Add, Player>, mut commands: Commands) {
    let player = on.entity;

    commands.entity(player).insert(actions!(Player[
        (
            Action::<Movement>::new(),
            DeadZone::default(),
            Bindings::spawn((
                Cardinal::wasd_keys(),
                Axial::left_stick()
            ))
        ),
        (
            Action::<Jump>::new(),
            bindings![KeyCode::Space,  GamepadButton::South],
        ),
        (
            Action::<Crouch>::new(),
            bindings![KeyCode::ControlLeft, GamepadButton::LeftTrigger],
        ),
        (
            Action::<RotateCamera>::new(),
            Scale::splat(0.04),
            Bindings::spawn((
                Spawn(Binding::mouse_motion()),
                Axial::right_stick()
            ))
        ),
        (
            Action::<InteractionAction>::new(),
            Hold::new(999f32).one_shot(false),
            bindings![KeyCode::KeyE, GamepadButton::West]
        ),
    ]));
}
