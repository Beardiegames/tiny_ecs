use std::fmt::Debug;
use std::convert::TryInto;

use super::{
    MAX_OBJECTS,
    ObjectIndex,
    NameTag,
    BitFlags,
};

pub struct Entities {
    pub(crate) pool: [BitFlags; MAX_OBJECTS],
    pub(crate) active: Vec<ObjectIndex>,
    pub(crate) free: Vec<ObjectIndex>,
}

impl Entities {
    pub(crate) fn new() -> Self {
        let mut create_entities = Vec::<BitFlags>::with_capacity(MAX_OBJECTS);
        create_entities.resize_with(MAX_OBJECTS, Default::default);

        let mut free = Vec::with_capacity(MAX_OBJECTS);
        for i in 0..MAX_OBJECTS { free.push(i); }

        Entities { 
            pool: create_entities.try_into().unwrap(),
            active: Vec::with_capacity(MAX_OBJECTS),
            free,
        }
    }
}

pub struct Objects<T: Default> {
    pub(crate) pool: [T; MAX_OBJECTS],
    pub(crate) active: Vec<(ObjectIndex, NameTag)>,
}

impl<T: Default + Debug> Objects<T> {

    pub(crate) fn new() -> Self {
        let mut create_objects = Vec::<T>::with_capacity(MAX_OBJECTS);
        create_objects.resize_with(MAX_OBJECTS, Default::default);

        Objects { 
            pool: create_objects.try_into().unwrap(),
            active: Vec::new(),
        }
    }

    pub fn get_mut(&mut self, target: &ObjectIndex) -> &mut T {
        &mut self.pool[*target]
    }

    pub fn get_ref(&self, target: &ObjectIndex) -> &T {
        &self.pool[*target]
    }

    pub fn find(&self, name: &str) -> Option<ObjectIndex> {
        let tag = NameTag::from_str(name);
        self.active.iter().find(|x| x.1 == tag).map(|a| a.0)
    }
}
