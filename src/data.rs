use crate::recipe;
use dioxus::prelude::*;
use recipe::RecipeProps;
use std::ffi::OsString;

pub struct RecipeDatabase {
    recipes: Vec<RecipeProps>,
}

impl RecipeDatabase {
    pub fn new(recipes: Vec<RecipeProps>) -> Self {
        Self { recipes }
    }
}

pub fn load_recipes() -> Vec<RecipeProps> {
    vec![
        RecipeProps::new(
            0,
            "Coconut Butter Cookies".to_string(),
            OsString::new(),
            "Step 1".to_string(),
        ),
        RecipeProps::new(
            1,
            "Babaganouch".into(),
            OsString::new(),
            "First, ...".into(),
        ),
        RecipeProps::new(
            2,
            "Bar Snack nuts".into(),
            OsString::new(),
            "Just prepare...".into(),
        ),
    ]
}

impl RecipeDatabase {
    pub fn get_recipe(&self, id: u32) -> Option<RecipeProps> {
        if let Some(recipe) = self.recipes.iter().find(|recipe| recipe.id == id) {
            Some((*recipe).clone())
        } else {
            None
        }
    }

    pub fn get_recipes(&self) -> Vec<RecipeProps> {
        self.recipes.clone()
    }
}
