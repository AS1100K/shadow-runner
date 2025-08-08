use bevy::prelude::*;
use std::marker::PhantomData;

pub struct Maybe<B: Bundle>(pub Option<B>);

impl<B: Bundle> Maybe<B> {
    pub const NONE: Self = Self(None);

    pub fn new(bundle: B) -> Self {
        Self(Some(bundle))
    }

    pub fn into_inner(self) -> Option<B> {
        self.0
    }
}

impl<B: Bundle> Default for Maybe<B> {
    fn default() -> Self {
        Self::NONE
    }
}

impl<B: Bundle> Component for Maybe<B> {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::SparseSet;
    
    type Mutability = bevy::ecs::component::Mutable;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, hook_context| {
            // Component hooks can't perform structural changes, so we need to rely on commands.
            world.commands().queue(MaybeCommand {
                entity: hook_context.entity,
                _phantom: std::marker::PhantomData::<B>,
            });
        });
    }
}

struct MaybeCommand<B> {
    entity: Entity,
    _phantom: PhantomData<B>,
}

impl<B: Bundle> Command for MaybeCommand<B> {
    fn apply(self, world: &mut World) {
        let Ok(mut entity_mut) = world.get_entity_mut(self.entity) else {
            #[cfg(debug_assertions)]
            panic!("Entity with Maybe component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let Some(maybe_component) = entity_mut.take::<Maybe<B>>() else {
            #[cfg(debug_assertions)]
            panic!("Maybe component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        if let Some(bundle) = maybe_component.into_inner() {
            entity_mut.insert(bundle);
        }
    }
}
