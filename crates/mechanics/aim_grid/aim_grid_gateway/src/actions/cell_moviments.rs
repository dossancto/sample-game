use bevy_enhanced_input::prelude::InputAction;

/// Left Hand

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct MoveLeftHandToLeft;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct MoveLeftHandToRight;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct MoveLeftHandToDown;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct MoveLeftHandToUp;

/// Right Hand

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct MoveRightHandToLeft;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct MoveRightHandToRight;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct MoveRightHandToDown;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct MoveRightHandToUp;
