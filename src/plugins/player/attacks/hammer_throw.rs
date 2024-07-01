use crate::{
    components::{EntityState, Player, Spammer},
    plugins::{
        asset_loader::{AnimationKey, AnimationMap},
        default_plugins::CursorPosition,
    },
    AnimationTimer,
};
use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{RigidBody, Velocity},
    geometry::{Collider, CollisionGroups, Group},
    plugin::RapierContext,
};

const HAMMER_SPEED: f32 = 600.0;
const ROTATION_DIVIDER: f32 = 200.0;

#[derive(Component)]
struct Hammer;

pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_button_input)
            .add_systems(Update, collision)
            .add_systems(Update, animate);
    }
}

fn mouse_button_input(
    mut command: Commands,
    player: Query<&Transform, With<Player>>,
    cursor_position: Res<CursorPosition>,
    buttons: Res<ButtonInput<MouseButton>>,
    animation_map: Res<AnimationMap>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let animation = animation_map
            .0
            .get(&AnimationKey::Hammer)
            .expect("Player animation were not found");

        let p_transform = player.single();

        let mut sprite_sheet = SpriteSheetBundle {
            transform: *p_transform,
            ..default()
        };

        sprite_sheet.texture = animation.texture.clone();
        sprite_sheet.atlas = TextureAtlas {
            layout: animation.atlas.clone(),
            index: 1,
        };

        let p1 = p_transform.translation.truncate();
        let p2 = cursor_position.0;

        command.spawn((
            Hammer,
            EntityState::Idle,
            Collider::cuboid(14.0, 14.0),
            RigidBody::Dynamic,
            CollisionGroups::new(Group::GROUP_1, Group::GROUP_3 | Group::GROUP_2),
            Velocity::linear((p2 - p1).normalize() * HAMMER_SPEED),
            sprite_sheet,
        ));
    }
}

fn animate(
    mut sprite_query: Query<
        (&mut TextureAtlas, &mut Transform, &EntityState, &Velocity),
        With<Hammer>,
    >,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
    animation: Res<AnimationMap>,
) {
    for (mut atlas, mut transform, state, velocity) in sprite_query.iter_mut() {
        let hammer_animation = &animation
            .0
            .get(&AnimationKey::Hammer)
            .expect("Animation were not found");

        let frames = hammer_animation
            .indices
            .get(state)
            .unwrap_or(&hammer_animation.default);

        timer.0.tick(time.delta());
        if timer.0.finished() {
            let mut index = atlas.index + 1;

            if atlas.index >= frames.last_frame || atlas.index < frames.first_frame {
                index = frames.first_frame;
            }

            atlas.index = index;
        }

        transform.rotate_z(f32::to_radians(-f32::floor(
            velocity.linvel.x / ROTATION_DIVIDER,
        )));
    }
}

fn collision(
    mut commands: Commands,
    mut hammers: Query<Entity, (With<Hammer>, Without<Spammer>)>,
    spammers: Query<Entity, (With<Spammer>, Without<Hammer>)>,
    rapier_context: Res<RapierContext>,
) {
    for hammer in hammers.iter_mut() {
        for spammer in spammers.iter() {
            if rapier_context.contact_pair(hammer, spammer).is_some() {
                commands.entity(spammer).despawn();
            }
        }
    }
}
