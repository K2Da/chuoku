use super::Profile;
use crate::error::Error::*;
use crate::error::*;
use std::collections::hash_map::IterMut;
use std::collections::HashMap;

pub struct ProfileMap {
    map: HashMap<String, Profile>,
}

impl ProfileMap {
    pub fn new() -> Self {
        ProfileMap {
            map: HashMap::new(),
        }
    }

    pub fn get_mut(&mut self, name: &str) -> Result<&mut Profile> {
        Ok(self
            .map
            .get_mut(name)
            .ok_or(ProfileNotFound(name.to_owned()))?)
    }

    pub fn get(&self, name: &str) -> Result<&Profile> {
        Ok(self.map.get(name).ok_or(ProfileNotFound(name.to_owned()))?)
    }

    pub fn insert(&mut self, name: &str, profile: Profile) {
        self.map.insert(name.to_string(), profile);
    }

    pub fn iter_mut(&mut self) -> IterMut<String, Profile> {
        self.map.iter_mut()
    }

    pub fn profiles(&self) -> Vec<&Profile> {
        self.map.values().collect::<Vec<&Profile>>()
    }

    pub fn print_export(&self, profile_name: &str) -> Result<()> {
        println!("{}", self.get(profile_name)?.export()?);
        Ok(())
    }
}