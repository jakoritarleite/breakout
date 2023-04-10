use std::collections::HashMap;

use super::component::ComponentId;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Entity(usize);

#[derive(Debug, Default)]
pub struct Entities {
    entities: Vec<Entity>,
    components: HashMap<Entity, Vec<ComponentId>>,
}

impl Entities {
    pub fn alloc(&mut self) -> Entity {
        let entity: Entity = (self.entities.len()).into();

        self.entities.push(entity.clone());

        entity
    }

    pub fn set_components(&mut self, entity: Entity, components_ids: Vec<ComponentId>) {
        self.components
            .entry(entity)
            .and_modify(|comps| comps.extend(components_ids.clone().into_iter()))
            .or_insert(components_ids);
    }
}

impl From<usize> for Entity {
    fn from(value: usize) -> Self {
        Entity(value)
    }
}
