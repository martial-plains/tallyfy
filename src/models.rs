use enum_iterator::Sequence;
use uuid::Uuid;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Sequence, Hash)]
pub enum Color {
    #[default]
    System,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Counter {
    pub id: Uuid,
    pub title: String,
    pub count: usize,
    pub color: Color,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::from("Untitled"),
            count: Default::default(),
            color: Color::default(),
        }
    }
}
