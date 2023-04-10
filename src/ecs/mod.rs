use std::any::TypeId;

pub mod component;
pub mod entity;
pub mod world;

pub type TypeIdMap<V> = rustc_hash::FxHashMap<TypeId, V>;
