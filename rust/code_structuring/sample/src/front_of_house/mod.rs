// Re-expose functions under the modules front_of_house
pub use self::hosting::seat_at_table;
pub use self::serving::take_order;

mod hosting;
mod serving;