use avian2d::prelude::CollisionStarted;
use bevy::prelude::*;

use crate::common_components::{Damage, Enemy, Friendly, Health};

pub struct CollisionsHandlerPlugin;

impl Plugin for CollisionsHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hit.run_if(on_event::<CollisionStarted>));
    }
}

fn hit(
    mut enemies: Query<(Option<&mut Health>, Option<&Damage>), (With<Enemy>, Without<Friendly>)>,
    mut allies: Query<(Option<&mut Health>, Option<&Damage>), (With<Friendly>, Without<Enemy>)>,
    mut collisions: EventReader<CollisionStarted>,
) {
    for CollisionStarted(entity1, entity2) in collisions.read() {
        if let (Ok(friendly), Ok(enemy)) = (allies.get_mut(*entity1), enemies.get_mut(*entity2)) {
            if let Some(mut hp) = friendly.0 {
                if let Some(dmg) = enemy.1 {
                    hp.0 -= dmg.0;
                }
            }

            if let Some(mut hp) = enemy.0 {
                if let Some(dmg) = friendly.1 {
                    hp.0 -= dmg.0;
                }
            }
        } else if let (Ok(friendly), Ok(enemy)) =
            (allies.get_mut(*entity2), enemies.get_mut(*entity1))
        {
            if let Some(mut hp) = friendly.0 {
                if let Some(dmg) = enemy.1 {
                    hp.0 -= dmg.0;
                }
            }

            if let Some(mut hp) = enemy.0 {
                if let Some(dmg) = friendly.1 {
                    hp.0 -= dmg.0;
                }
            }
        }

        println!("{entity1}, {entity2}");
    }
}
