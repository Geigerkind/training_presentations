use crate::domain::{HeroManager, Hero, SearchResult, HeroSearchFilter};
use rocket_contrib::json::Json;
use rocket::State;
use crate::tools::GetHeroes;

#[post("/heroes", data = "<filter>")]
pub fn get_heroes(
    me: State<HeroManager>,
    filter: Json<HeroSearchFilter>,
) -> Json<SearchResult<Hero>> {
    Json(me.get_heroes(filter.into_inner()))
}