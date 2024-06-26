use crate::{
    components::{Damage, EntityState, Health, Player, Spammer},
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
const HEALTH: i32 = 2;
const DAMAGE: i32 = 2;

#[derive(Component)]
struct Hammer;

pub struct HammerPlugin;

impl Plugin for HammerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_button_input)
            .add_systems(Update, collision)
            .add_systems(Update, death)
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
            Damage(DAMAGE),
            Health(HEALTH),
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
    mut hammers: Query<(Entity, &mut Health, &Damage), (With<Hammer>, Without<Spammer>)>,
    mut spammers: Query<(Entity, &mut Health), (With<Spammer>, Without<Hammer>)>,
    rapier_context: Res<RapierContext>,
) {
    for (h_id, mut h_health, h_dmg) in hammers.iter_mut() {
        for (s_id, mut s_health) in spammers.iter_mut() {
            if rapier_context.contact_pair(h_id, s_id).is_some() {
                s_health.0 -= h_dmg.0;
                h_health.0 -= 1;
            }
        }
    }
}

fn death(mut commands: Commands, hammers: Query<(Entity, &Health), With<Hammer>>) {
    for (id, health) in hammers.iter() {
        if health.0 <= 0 {
            commands.entity(id).despawn();
        }
    }
}
