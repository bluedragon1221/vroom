use serde_derive::{Deserialize, Serialize};

use crate::error::VroomError;

#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    value: String,
}

impl Item {
    fn new(name: &str, value: &str) -> Item {
        Item {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct List {
    name: String,
    pub contents: Vec<Item>,
}

impl List {
    pub fn new(name: &str) -> List {
        List {
            name: name.into(),
            contents: Vec::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_item(&mut self, name: &str, value: &str) {
        self.contents.push(Item::new(name, value))
    }

    pub fn rm_item(&mut self, name: &str) {
        self.contents.retain(|x| x.name != name)
    }

    #[must_use]
    pub fn get_mut_item(&mut self, item_name: &str) -> Result<&mut Item, VroomError> {
        self.contents
            .iter_mut()
            .find(|x| x.name == item_name)
            .ok_or(VroomError::NoSuchItem(item_name.into(), self.name.clone()))
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Vroomfile {
    pub contents: Vec<List>,
}

impl Vroomfile {
    pub fn add_list(&mut self, list_name: &str) {
        self.contents.push(List::new(list_name))
    }

    #[must_use]
    pub fn get_mut_list(&mut self, list_name: &str) -> Result<&mut List, VroomError> {
        self.contents
            .iter_mut()
            .find(|x| x.name == list_name)
            .ok_or(VroomError::NoSuchList(list_name.into()))
    }

    pub fn rm_list(&mut self, list_name: &str) -> Result<(), VroomError> {
        let index: usize = self
            .contents
            .iter()
            .position(|x| x.name == list_name)
            .ok_or(VroomError::NoSuchList(list_name.into()))?;

        self.contents.remove(index);
        Ok(())
    }

    fn flat_items(&mut self) -> Vec<&mut Item> {
        let mut res: Vec<Vec<&mut Item>> = Vec::new();
        for list in self.contents.iter_mut() {
            res.push(list.contents.iter_mut().collect())
        }

        res.into_iter().flatten().collect()
    }

    #[must_use]
    pub fn get_mut_item(&mut self, item_name: &str) -> Result<&mut Item, VroomError> {
        self.flat_items()
            .into_iter()
            .find(|x| x.name == item_name)
            .ok_or(VroomError::NoSuchItemAny(item_name.into()))
    }
}
