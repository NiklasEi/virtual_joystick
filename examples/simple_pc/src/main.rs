use bevy::prelude::*;

use virtual_joystick::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VirtualJoystickPlugin::<String>::default())
        .add_systems(Startup, create_scene)
        .add_systems(Update, update_joystick)
        .run();
}

#[derive(Component)]
// Player with velocity
struct Player(pub f32);

fn create_scene(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 5.0),
        ..default()
    });
    cmd.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            ..default()
        },
        texture: asset_server.load("Knob.png"),
        sprite: Sprite {
            color: Color::PURPLE,
            custom_size: Some(Vec2::new(50., 50.)),
            ..default()
        },
        ..default()
    })
    .insert(Player(50.));
    // Spawn Virtual Joystick at horizontal center
    cmd.spawn(
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: asset_server.load("Outline.png"),
            knob_image: asset_server.load("Knob.png"),
            knob_size: Vec2::new(80., 80.),
            dead_zone: 0.,
            id: "UniqueJoystick".to_string(),
            axis: VirtualJoystickAxis::Both,
            behaviour: VirtualJoystickType::Floating,
        })
        .set_color(TintColor(Color::WHITE))
        .set_style(Style {
            width: Val::Px(150.),
            height: Val::Px(150.),
            position_type: PositionType::Absolute,
                left: Val::Percent(50.),
                bottom: Val::Percent(15.),
            ..default()
        }),
    )
    .insert(BackgroundColor(Color::ORANGE_RED.with_a(0.3)))
    .insert(VirtualJoystickInteractionArea);
}

fn update_joystick(
    mut joystick: EventReader<VirtualJoystickEvent<String>>,
    mut player: Query<(&mut Transform, &Player)>,
    time_step: Res<FixedTime>,
) {
    let (mut player, player_data) = player.single_mut();

    for j in joystick.iter() {
        println!("{:?}", j.value());
        let Vec2 { x, y } = j.axis();
        player.translation.x += x * player_data.0 * time_step.period.as_secs_f32();
        player.translation.y += y * player_data.0 * time_step.period.as_secs_f32();
    }
}
