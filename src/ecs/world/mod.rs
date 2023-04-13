use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

use self::query::Query;
use self::query::QueryState;

use super::component::Bundle;
use super::component::Component;
use super::component::ComponentId;
use super::component::Components;
use super::entity::Entities;
use super::entity::Entity;

pub mod query;

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

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> Entity {
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

        self.entities
            .set_components(entity.clone(), components_ids.clone());

        entity
    }

    pub fn query<Q: Query>(&mut self) -> QueryState<Q> {
        QueryState::new(self)
    }
}

#[derive(Debug, Default)]
pub struct Storages {
    pub hashmaps: HashMap<ComponentId, HashMap<Entity, Box<RefCell<dyn Any>>>>,
}

impl Storages {
    pub fn init_component(&mut self, component_id: ComponentId) {
        self.hashmaps
            .entry(component_id)
            .or_insert_with(|| HashMap::new());
    }

    pub fn push_component<C: Component + Clone>(
        &mut self,
        entity: Entity,
        component_id: ComponentId,
        component: C,
    ) {
        let component = Box::new(RefCell::new(component));

        self.hashmaps
            .entry(component_id)
            .and_modify(|comp| {
                comp.entry(entity.clone()).or_insert(component.clone());
            })
            .or_insert_with(|| {
                let mut inner: HashMap<_, Box<RefCell<dyn Any>>> = HashMap::new();
                inner.insert(entity, component);
                inner
            });
    }
}
