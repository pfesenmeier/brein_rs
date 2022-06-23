use crate::recipe;
use recipe::RecipeModel;
use std::ffi::OsString;

pub struct RecipeDatabase {
    recipes: Vec<RecipeModel>,
}

impl RecipeDatabase {
    pub fn new(recipes: Vec<RecipeModel>) -> Self {
        Self { recipes }
    }
}

pub fn load_recipes() -> Vec<RecipeModel> {
    vec![
        RecipeModel::new(
            0,
            "Coconut Butter Cookies".to_string(),
            OsString::new(),
            "Step 1".to_string(),
        ),
        RecipeModel::new(
            1,
            "Babaganouch".into(),
            OsString::new(),
            "First, ...".into(),
        ),
        RecipeModel::new(
            2,
            "Bar Snack nuts".into(),
            OsString::new(),
            "Just prepare...".into(),
        ),
    ]
}

impl RecipeDatabase {
    pub fn get_recipe(&self, id: u32) -> Option<RecipeModel> {
        if let Some(recipe) = self.recipes.iter().find(|recipe| recipe.id == id) {
            Some((*recipe).clone())
        } else {
            None
        }
    }

    pub fn get_recipes(&self) -> Vec<RecipeModel> {
        self.recipes.clone()
    }
}
