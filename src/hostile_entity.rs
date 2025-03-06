use crate::{
    assets::AudioAssets,
    colliders::ColliderBundle,
    player::{Blinded, ContinueTakingDamage, HealthBar, PlayerEntity},
    sprite_animation::Animation,
    utils::Maybe,
    GameState, GRID_SIZE,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};
use bevy_rapier2d::prelude::{ActiveEvents, CollisionEvent, Velocity};

pub struct HostilePlugin;

impl Plugin for HostilePlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<Hostile>("Sand_Ghoul")
            .register_ldtk_entity::<Hostile>("Grave_Revenant")
            .register_ldtk_entity::<Hostile>("Mutilated_Stumbler")
            .register_ldtk_entity::<Hostile>("Adept_Necromancer")
            .add_systems(
                Update,
                (patrol, damage_player).run_if(in_state(GameState::PlayingScreen)),
            )
            .add_systems(
                FixedUpdate,
                blinding_power.run_if(in_state(GameState::PlayingScreen)),
            );
    }
}

#[derive(better_default::Default, Bundle, LdtkEntity)]
#[default(
    active_events: ActiveEvents::COLLISION_EVENTS,
    animation: Animation::new(0, 3, Timer::from_seconds(0.25, TimerMode::Repeating))
)]
pub struct Hostile {
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    #[ldtk_entity]
    pub patrol: Patrol,
    pub active_events: ActiveEvents,
    hostile_entity: HostileEntity,
    pub animation: Animation,
    #[from_entity_instance]
    pub damage_count: DamageCount,
    #[from_entity_instance]
    pub blindness_power: Maybe<BlindnessPower>,
}

#[derive(Default, Component)]
pub struct HostileEntity;

#[derive(Default, Component)]
pub struct BlindnessPower;

impl From<&EntityInstance> for Maybe<BlindnessPower> {
    fn from(value: &EntityInstance) -> Self {
        if value.identifier == "Adept_Necromancer" {
            return Self::new(BlindnessPower);
        }
        Self::NONE
    }
}

#[derive(Default, Component)]
pub struct DamageCount(pub u8);

impl From<&EntityInstance> for DamageCount {
    fn from(value: &EntityInstance) -> Self {
        match value.identifier.as_str() {
            "Sand_Ghoul" | "Adept_Necromancer" => DamageCount(1),
            "Grave_Revenant" => DamageCount(2),
            "Mutilated_Stumbler" => DamageCount(3),
            _ => DamageCount(0),
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Patrol {
    pub points: Vec<Vec2>,
    pub index: usize,
    pub forward: bool,
}

impl LdtkEntity for Patrol {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        let mut points = Vec::new();
        points.push(ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        ));

        let ldtk_patrol_points = entity_instance
            .iter_points_field("patrol")
            .expect("patrol field should be correctrly typed for hostile entities");

        for ldtk_point in ldtk_patrol_points {
            let pixel_coords = (ldtk_point.as_vec2() + Vec2::new(0.5, 0.5))
                * Vec2::splat(layer_instance.grid_size as f32);

            points.push(ldtk_pixel_coords_to_translation_pivoted(
                pixel_coords.as_ivec2(),
                layer_instance.c_hei * layer_instance.grid_size,
                IVec2::new(entity_instance.width, entity_instance.height),
                entity_instance.pivot,
            ));
        }

        Self {
            points,
            index: 1,
            forward: true,
        }
    }
}

pub fn patrol(mut query: Query<(&mut Transform, &mut Velocity, &mut Patrol, &mut Sprite)>) {
    for (mut transform, mut velocity, mut patrol, mut sprite) in &mut query {
        if patrol.points.len() <= 1 {
            continue;
        }

        let mut new_velocity =
            (patrol.points[patrol.index] - transform.translation.truncate()).normalize() * 50.;

        if new_velocity.dot(velocity.linvel) < 0. {
            if patrol.index == 0 {
                patrol.forward = true;
                sprite.flip_x = false;
            } else if patrol.index == patrol.points.len() - 1 {
                patrol.forward = false;
                sprite.flip_x = true;
            }

            transform.translation.x = patrol.points[patrol.index].x;
            transform.translation.y = patrol.points[patrol.index].y;

            if patrol.forward {
                patrol.index += 1;
            } else {
                patrol.index -= 1;
            }

            new_velocity =
                (patrol.points[patrol.index] - transform.translation.truncate()).normalize() * 75.;
        }

        velocity.linvel = new_velocity;
    }
}

fn damage_player(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut HealthBar, Option<&AudioPlayer>), With<PlayerEntity>>,
    hostile_query: Query<(Entity, &DamageCount), With<HostileEntity>>,
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
) {
    for collision in collision_events.read() {
        let (player_entity, mut player_healtbar, audio_player) = player_query.single_mut();
        match *collision {
            CollisionEvent::Started(entity_one, entity_two, ..) => {
                if entity_one == player_entity || entity_two == player_entity {
                    for (hostile_entity, damage_count) in &hostile_query {
                        if entity_two == hostile_entity || entity_one == hostile_entity {
                            log::info!("Got damage from hostile entity");

                            if player_healtbar.health > damage_count.0 {
                                player_healtbar.health -= damage_count.0;

                                let mut entity_commands = commands.entity(player_entity);
                                entity_commands.insert(ContinueTakingDamage(damage_count.0));

                                if audio_player.is_none() {
                                    entity_commands.insert((
                                        AudioPlayer(audio_assets.damage.clone()),
                                        PlaybackSettings::REMOVE,
                                    ));
                                }
                            } else {
                                player_healtbar.health = 0;
                            }

                            return;
                        }
                    }
                }
            }
            CollisionEvent::Stopped(entity_one, entity_two, ..) => {
                if entity_one == player_entity || entity_two == player_entity {
                    for (hostile_entity, ..) in &hostile_query {
                        if entity_two == hostile_entity || entity_one == hostile_entity {
                            log::info!("Removing Continuous Damage");
                            commands
                                .entity(player_entity)
                                .remove::<ContinueTakingDamage>();
                            return;
                        }
                    }
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn blinding_power(
    player_query: Query<(Entity, &Transform), (Without<Blinded>, With<PlayerEntity>)>,
    blinding_power_entity: Query<&Transform, (With<BlindnessPower>, With<HostileEntity>)>,
    mut commands: Commands,
) {
    for (entity, player_transform) in &player_query {
        for blinding_power_transform in &blinding_power_entity {
            if player_transform
                .translation
                .truncate()
                .distance(blinding_power_transform.translation.truncate())
                <= (GRID_SIZE as f32 * 5.)
            {
                log::info!("Detected Player, Effecting with Blindness");
                commands
                    .entity(entity)
                    .insert(Blinded(Timer::from_seconds(15., TimerMode::Once)));
            }
        }
    }
}
