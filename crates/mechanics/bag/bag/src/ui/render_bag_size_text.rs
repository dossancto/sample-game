use bag_gateway::components::bag::Bag;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_can_bag_content);
    app.add_observer(clear_can_bag_content);

    app.add_systems(Update, update_content);
}

#[derive(Component)]
struct BagContentVolumeText;

#[derive(Component)]
struct BagContentMassText;

fn setup_can_bag_content(_on: On<Add, Bag>, mut commands: Commands) {
    commands.spawn((
        Name::new("Bag Content volume Text"),
        Text::new(""),
        BagContentVolumeText,
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));

    commands.spawn(( Name::new("Bag Content mass Text"),
        Text::new(""),
        BagContentMassText,
        Node {
            position_type: PositionType::Absolute,
            top: px(48),
            left: px(12),
            ..default()
        },
    ));
}

fn clear_can_bag_content(
    _on: On<Remove, Bag>,
    bag_volume_text_q: Query<Entity, With<BagContentVolumeText>>,
    bag_mass_text_q: Query<Entity, With<BagContentMassText>>,
    mut commands: Commands,
) {
    for visibility in bag_volume_text_q.iter() {
        commands.entity(visibility).despawn();
    }

    for visibility in bag_mass_text_q.iter() {
        commands.entity(visibility).despawn();
    }
}

fn update_content(
    bag: Single<&Bag>,
    volume_text: Single<&mut Text, (With<BagContentVolumeText>, Without<BagContentMassText>)>,
    mass_text: Single<&mut Text, (With<BagContentMassText>, Without<BagContentVolumeText>)>,
) {
    let volume = format!(
        "Volume: {:.2}/{:.2}",
        bag.current_volume, bag.max_volume
    );

    let mass = format!("Mass: {:.2}/{:.2}", bag.current_mass, bag.max_mass);

    let mut volume_text = volume_text.into_inner();
    let mut mass_text = mass_text.into_inner();

    volume_text.0 = volume;
    mass_text.0 = mass;
}
