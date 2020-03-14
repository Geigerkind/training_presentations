use crate::domain::{HeroSearchFilter, SearchResult, Hero, HeroManager};
use std::cmp::Ordering;

pub trait GetHeroes {
    fn get_heroes(&self, filter: HeroSearchFilter) -> SearchResult<Hero>;
}

impl GetHeroes for HeroManager {
    fn get_heroes(&self, filter: HeroSearchFilter) -> SearchResult<Hero> {
        let mut result = self.heroes.iter()
            .filter(|hero| filter.id.filter.is_none() || hero.id == *filter.id.filter.as_ref().unwrap())
            .filter(|hero| filter.name.filter.is_none() || hero.name.to_lowercase().contains(&filter.name.filter.as_ref().unwrap().to_lowercase()))
            .filter(|hero| filter.strength.filter.is_none() || hero.strength.to_lowercase().contains(&filter.strength.filter.as_ref().unwrap().to_lowercase()))
            .filter(|hero| filter.sub_title.filter.is_none() || (hero.sub_title.is_some() && hero.sub_title.as_ref().unwrap().to_lowercase().contains(&filter.sub_title.filter.as_ref().unwrap().to_lowercase())))
            .filter(|hero| filter.weakness.filter.is_none() || (hero.weakness.is_some() && hero.weakness.as_ref().unwrap().to_lowercase().contains(&filter.weakness.filter.as_ref().unwrap().to_lowercase())))
            .filter(|hero| filter.hero_call.filter.is_none() || (hero.hero_call.is_some() && hero.hero_call.as_ref().unwrap().to_lowercase().contains(&filter.hero_call.filter.as_ref().unwrap().to_lowercase())))
            .map(|hero| hero.clone())
            .collect::<Vec<Hero>>();
        let num_items = result.len();
        result.sort_by(|left, right| {
            if let Some(sorting) = filter.id.sorting {
                let ordering = left.id
                    .cmp(&right.id);
                if ordering != Ordering::Equal {
                    return negate_ordering(ordering, sorting);
                }
            }
            if let Some(sorting) = filter.name.sorting {
                let ordering = left.name
                    .cmp(&right.name);
                if ordering != Ordering::Equal {
                    return negate_ordering(ordering, sorting);
                }
            }
            if let Some(sorting) = filter.sub_title.sorting {
                if left.sub_title.is_some() && right.sub_title.is_some() {
                    let ordering = left.sub_title.as_ref().unwrap()
                        .cmp(&right.sub_title.as_ref().unwrap());
                    if ordering != Ordering::Equal {
                        return negate_ordering(ordering, sorting);
                    }
                }
            }
            if let Some(sorting) = filter.strength.sorting {
                let ordering = left.strength
                    .cmp(&right.strength);
                if ordering != Ordering::Equal {
                    return negate_ordering(ordering, sorting);
                }
            }
            if let Some(sorting) = filter.weakness.sorting {
                if left.weakness.is_some() && right.weakness.is_some() {
                    let ordering = left.weakness.as_ref().unwrap()
                        .cmp(&right.weakness.as_ref().unwrap());
                    if ordering != Ordering::Equal {
                        return negate_ordering(ordering, sorting);
                    }
                }
            }
            if let Some(sorting) = filter.hero_call.sorting {
                if left.hero_call.is_some() && right.hero_call.is_some() {
                    let ordering = left.hero_call.as_ref().unwrap()
                        .cmp(&right.hero_call.as_ref().unwrap());
                    if ordering != Ordering::Equal {
                        return negate_ordering(ordering, sorting);
                    }
                }
            }
            return Ordering::Equal;
        });

        SearchResult {
            result: result.iter()
                .skip((filter.page * 10) as usize)
                .take(10)
                .map(|hero| hero.to_owned())
                .collect(),
            num_items
        }
    }
}

fn negate_ordering(ordering: Ordering, sorting: bool) -> Ordering {
    if ordering == Ordering::Less {
        if sorting {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }
    if sorting {
        return Ordering::Greater;
    }
    return Ordering::Less;
}