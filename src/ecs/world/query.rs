use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::ecs::component::{Component, ComponentId};
use crate::ecs::entity::Entity;

use super::World;

pub trait Query {
    type Item<'a>;

    type Fetch<'a>;

    type State;

    fn init_state(world: &mut World) -> Self::State;

    fn init_fetch<'w>(world: &'w World, state: &Self::State) -> Self::Fetch<'w>;

    fn fetch<'w>(fetch: &mut Self::Fetch<'w>, entity: Entity) -> Self::Item<'w>;
}

pub struct QueryState<Q: Query> {
    pub fetch_state: Q::State,
}

impl<Q: Query> QueryState<Q> {
    pub fn new(world: &mut World) -> Self {
        let fetch_state = Q::init_state(world);

        Self { fetch_state }
    }

    pub fn get<'w>(&mut self, world: &'w mut World, entity: Entity) -> Result<Q::Item<'w>, String> {
        let mut fetch = Q::init_fetch(world, &self.fetch_state);

        Ok(Q::fetch(&mut fetch, entity))
    }
}

impl Query for Entity {
    type Item<'a> = Entity;

    type Fetch<'a> = ();

    type State = ();

    fn init_state(_world: &mut World) -> Self::State {}

    fn init_fetch<'w>(_world: &'w World, _state: &Self::State) -> Self::Fetch<'w> {}

    fn fetch<'w>(_fetch: &mut Self::Fetch<'w>, entity: Entity) -> Self::Item<'w> {
        entity
    }
}

pub struct ReadFetch<'a> {
    storage: Option<&'a HashMap<Entity, Box<dyn Any>>>,
}

impl<T: Component> Query for &T {
    type Item<'w> = Option<&'w T>;

    type Fetch<'w> = ReadFetch<'w>;

    type State = ComponentId;

    fn init_state(world: &mut World) -> Self::State {
        world.init_component::<T>()
    }

    fn init_fetch<'w>(world: &'w World, component_id: &ComponentId) -> Self::Fetch<'w> {
        ReadFetch {
            storage: world.storages.hashmaps.get(component_id),
        }
    }

    fn fetch<'w>(fetch: &mut Self::Fetch<'w>, entity: Entity) -> Self::Item<'w> {
        match fetch.storage {
            Some(storage) => match storage.get(&entity) {
                Some(component) => component.downcast_ref::<T>(),
                None => None,
            },
            None => None,
        }
    }
}

macro_rules! tuple_impls {
    ($head_ty:ident) => {
        tuple_impl!($head_ty);
    };
    ($head_ty:ident, $( $tail_ty:ident ),*) => {
        tuple_impl!($head_ty, $( $tail_ty ),*);
        tuple_impls!($( $tail_ty ),*);
    };
}

macro_rules! tuple_impl {
    ( $( $name:ident ),* ) => {
        impl<$($name: Query),*> Query for ($($name,)*) {
            #![allow(non_snake_case)]
            type Item<'w> = ($($name::Item<'w>,)*);
            type Fetch<'w> = ($($name::Fetch<'w>,)*);
            type State = ($($name::State,)*);

            fn init_state(world: &mut World) -> Self::State {
                ($($name::init_state(world),)*)
            }

            fn init_fetch<'w>(_world: &'w World, state: &Self::State) -> Self::Fetch<'w> {
                let ($($name,)*) = state;
                ($($name::init_fetch(_world, $name),)*)
            }

            fn fetch<'w>(_fetch: &mut Self::Fetch<'w>, _entity: Entity) -> Self::Item<'w> {
                let ($($name,)*) = _fetch;
                ($($name::fetch($name, _entity.clone()),)*)
            }
        }
    };
}

tuple_impls!(A, B, C, D, E, F, G, H, I, J, K, L);
