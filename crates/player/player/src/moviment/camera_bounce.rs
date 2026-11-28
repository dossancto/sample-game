use bevy::prelude::*;
use player_gateway::{Player, components::PlayerMoving};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, while_walking);
}

fn while_walking(
    player: Single<(Option<&PlayerMoving>,), (With<Player>,)>,
    time: Res<Time>,
    mut bounce_controll: Single<&mut CameraBounce>,
) {
    let (player_moving,) = player.into_inner();

    let is_moving = player_moving.is_some();

    if is_moving {
        bounce_controll.active = true;
    }

    if bounce_controll.active == false {
        return;
    }

    let height = bounce_controll.intensity;

    let frequency = bounce_controll.frequency;

    let offset = 0f32;
    let range = 0f32;

    let x = time.elapsed_secs();

    let content = (x - offset) / frequency;

    let y = height * content.sin() + range;

    if is_moving == false && bounce_controll.is_peak(y) {
        bounce_controll.active = false;
        return;
    }

    info!("{}", y);

    bounce_controll.last_y = y;
}

#[derive(Component, Default)]
pub struct CameraBounce {
    pub last_y: f32,
    pub active: bool,

    pub intensity: f32,
    pub frequency: f32,
}

impl CameraBounce {
    pub fn is_peak(self: &Self, current_y: f32) -> bool {
        let was_positive = self.last_y.is_sign_positive();
        let still_positive = current_y.is_sign_positive();

        return was_positive != still_positive;
    }

    pub fn new(intensity: f32, frequency: f32) -> Self {
        Self {
            intensity: intensity,
            frequency: frequency,
            ..Default::default()
        }
    }
}
