use std::ffi::OsString;

pub struct Recipe {
    pub name: String,
    pub path: OsString,
    pub recipe: String,
}

impl Recipe {
    pub fn new(name: String, path: OsString, recipe: String) -> Self {
        Self { name, path, recipe }
    }
}

pub fn get_recipes() -> Vec<Recipe> {
    vec![
        Recipe::new(
            "Coconut Butter Cookies".to_string(),
            OsString::new(),
            "Step 1".to_string(),
        ),
        Recipe::new(
            "Babaganouch".into(),
            OsString::new(),
            "First, ...".into()
        ),
        Recipe::new(
            "Bar Snack nuts".into(),
            OsString::new(),
            "Just prepare...".into()
        )

    ]
}