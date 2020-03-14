use crate::domain::Hero;
use mysql::*;
use mysql::prelude::*;

pub struct HeroManager {
    pub heroes: Vec<Hero>
}

impl Default for HeroManager {
    fn default() -> Self {
        HeroManager {
            heroes: Vec::new()
        }
    }
}

impl HeroManager {
    pub fn init(mut self) -> Self {
        let url = "mysql://root:vagrant@localhost:3306/main";
        let pool = Pool::new(url).unwrap();
        let mut conn = pool.get_conn().unwrap();
        self.heroes = conn.query_map("SELECT * FROM heroes", |(id, name, sub_title, strength, weakness, hero_call)| Hero {
            id,
            name,
            sub_title,
            strength,
            weakness,
            hero_call
        }).unwrap();
        self
    }
}