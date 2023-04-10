use std::any::TypeId;

use super::{entity::Entity, world::Storages, TypeIdMap};

pub trait Component: std::any::Any + std::fmt::Debug {}

pub trait Bundle {
    fn components_ids(
        &self,
        entity: Entity,
        components: &mut Components,
        storages: &mut Storages,
        ids: &mut impl FnMut(ComponentId),
    );
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ComponentId(usize);

#[derive(Debug, Clone)]
pub struct ComponentInfo {
    pub id: ComponentId,
}

impl ComponentInfo {
    pub fn new(id: ComponentId) -> ComponentInfo {
        ComponentInfo { id }
    }
}

#[derive(Debug, Default)]
pub struct Components {
    components: Vec<ComponentInfo>,
    indices: TypeIdMap<usize>,
}

impl Components {
    pub fn init_component<T: Component>(&mut self, storages: &mut Storages) -> ComponentId {
        let type_id = TypeId::of::<T>();

        let Components {
            components,
            indices,
            ..
        } = self;

        let index = indices
            .entry(type_id)
            .or_insert_with(|| Components::init_component_inner(components));

        storages.init_component(type_id);

        ComponentId(*index)
    }

    #[inline]
    pub fn init_component_inner(components: &mut Vec<ComponentInfo>) -> usize {
        let index = components.len();
        let info = ComponentInfo::new(ComponentId(index));

        components.push(info);

        index
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
        impl<$($name: Bundle),*> Bundle for ($($name,)*) {
            #![allow(non_snake_case)]
            fn components_ids(
                &self,
                entity: Entity,
                components: &mut Components,
                storages: &mut Storages,
                ids: &mut impl FnMut(ComponentId)
            ) {
                let ($($name,)*) = self;

                $(
                    $name.components_ids(entity.clone(), components, storages, ids);
                )*
            }
        }
    };
}

tuple_impls!(A, B, C, D, E, F, G, H, I, J, K, L);
