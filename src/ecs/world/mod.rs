use std::any::TypeId;
use std::collections::HashMap;

use super::component::Bundle;
use super::component::Component;
use super::component::ComponentId;
use super::component::Components;
use super::entity::Entities;
use super::entity::Entity;
use super::TypeIdMap;

#[derive(Debug, Default)]
pub struct World {
    entities: Entities,
    components: Components,
    storages: Storages,
}

impl World {
    #[inline]
    pub fn new() -> World {
        World::default()
    }

    pub fn init_component<T: Component>(&mut self) -> ComponentId {
        self.components.init_component::<T>(&mut self.storages)
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) {
        let entity = self.entities.alloc();

        let mut components_ids = Vec::new();

        bundle.components_ids(
            entity.clone(),
            &mut self.components,
            &mut self.storages,
            &mut |id| {
                components_ids.push(id);
            },
        );

        self.entities.set_components(entity, components_ids.clone());
    }

    pub fn query<Q>(&mut self) {}
}

#[derive(Debug, Default)]
pub struct Storages {
    pub hashmaps: TypeIdMap<HashMap<Entity, Box<dyn Component>>>,
}

impl Storages {
    pub fn init_component(&mut self, type_id: TypeId) {
        self.hashmaps
            .entry(type_id)
            .or_insert_with(|| HashMap::new());
    }

    pub fn push_component<C: Component + Clone>(
        &mut self,
        entity: Entity,
        type_id: TypeId,
        component: C,
    ) {
        let component = Box::new(component);

        self.hashmaps
            .entry(type_id)
            .and_modify(|comp| {
                comp.entry(entity.clone()).or_insert(component.clone());
            })
            .or_insert_with(|| {
                let mut inner: HashMap<_, Box<dyn Component>> = HashMap::new();
                inner.insert(entity, component);
                inner
            });
    }
}

#[cfg(test)]
mod test {
    use breakout_macros::Component;

    use super::World;

    #[derive(Debug, Clone, Component)]
    struct Position(u8, u8);

    #[derive(Debug, Clone, Component)]
    struct Velocity(u8);

    #[test]
    fn spawn_entities() {
        let mut world = World::new();

        world.spawn((Position(0, 0), Velocity(1)));
        world.spawn((Velocity(3), Position(1, 1)));

        dbg!(world);
    }

    #[test]
    fn query_components() {
        let mut world = World::new();

        world.spawn((Position(0, 0), Velocity(1)));

        world.query::<Position>();
    }
}
