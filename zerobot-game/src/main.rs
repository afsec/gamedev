mod model;
use model::{Entity, EntityType, GlobalEnvironment, Quadrant};

fn empty_quadrant() -> Quadrant {
    Quadrant {
        x: 0,
        y: 0,
        entity: None,
    }
}

fn new_game() -> GlobalEnvironment {
    let mut quadrants: Vec<Quadrant> = Vec::new();
    quadrants.push(empty_quadrant());
    let world: GlobalEnvironment = GlobalEnvironment {
        quadrant: quadrants,
    };
    world
}

fn new_player() -> Entity {
    Entity {
        id: 1,
        entity_type: EntityType::Player,
        name: "User".to_string(),
    }
}

fn new_bot() -> Entity {
    Entity {
        id: 2,
        entity_type: EntityType::Bot,
        name: "Tron".to_string(),
    }
}

fn spawn_entity(world: &mut model::GlobalEnvironment, entity: Entity) {
    let last_quadrant = world.quadrant.last().clone().unwrap();
    dbg!(last_quadrant);
    let mut new_quadrant = empty_quadrant();
      new_quadrant.y = last_quadrant.y + 1;
    
    new_quadrant.entity = Some(entity);
    world.quadrant.push(new_quadrant);
    
}

fn main() {
    
    println!("ZeroBot!");
    println!("========");
    println!("");
    let mut world = new_game();
    dbg!(&world);
    spawn_entity(&mut world, new_bot());
    dbg!(&world);
    spawn_entity(&mut world, new_player());
    dbg!(&world);
    
}
