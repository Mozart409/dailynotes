use loco_rs::prelude::*;

use crate::models::_entities::movies;

/// Render a list view of movies.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<movies::Model>) -> Result<Response> {
    format::render().view(v, "movie/list.html", serde_json::json!({"items": items}))
}

/// Render a single movie view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &movies::Model) -> Result<Response> {
    format::render().view(v, "movie/show.html", serde_json::json!({"item": item}))
}

/// Render a movie create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "movie/create.html", serde_json::json!({}))
}

/// Render a movie edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &movies::Model) -> Result<Response> {
    format::render().view(v, "movie/edit.html", serde_json::json!({"item": item}))
}
