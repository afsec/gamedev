#[derive(Debug)]
pub struct GlobalEnvironment {
    pub quadrant: Vec<Quadrant>,
}

#[derive(Debug)]
pub struct Quadrant {
    pub x: i8,
    pub y: i8,
    pub entity: Option<Entity>
}

#[derive(Debug)]
pub struct Entity {
    pub id: u8,
    pub entity_type: EntityType,
    pub name: String
}

#[derive(Debug)]
pub enum EntityType {
    Player,
    Bot
}
