
#[get("/")]
pub fn get_profile() -> &'static str {
    "Profile!"
}

#[post("/")]
pub fn create_profile() -> &'static str {
    "New profile!"
}

#[put("/")]
pub fn update_profile() -> &'static str {
    "Updated profile!"
}

#[delete("/")]
pub fn delete_profile() -> &'static str {
    "Deleted profile!"
}
