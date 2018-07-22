use specs::*;
use comp::*;
use fpa::*;
use fpavec::*;
use drop_tables::*;
use rand::*;
use rand::rngs::StdRng;
use inventory;
use renderer;
use KilledEntities;

/// System for processing drops on entity death
pub struct OnDeathDropSys {
    rng: StdRng,
}

impl OnDeathDropSys {
    pub fn new(rng: StdRng) -> OnDeathDropSys {
        OnDeathDropSys {
            rng: rng,
        }
    }
}

impl<'a> System<'a> for OnDeathDropSys {
    type SystemData = (
        Read<'a, LazyUpdate>,
        ReadExpect<'a, KilledEntities>,
        ReadExpect<'a, DropTableMap>,
        Entities<'a>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, OnDeathDrop>);

    fn run(&mut self, (lazy_update, killed, drop_table_map, entities, pos_s,
                       on_death_drop_s): Self::SystemData) {
        // Loop over all dead entities that have a deathdrop component
        for e in &killed.0 {
            println!("HELLO");
            // Check if we can process drops on these entities
            let pos = pos_s.get(*e);
            let dd = on_death_drop_s.get(*e);
            if pos.is_none() || dd.is_none() { continue; }
            let pos = pos.unwrap();
            let dd = dd.unwrap();

            // Query the drop table
            let drop_table = drop_table_map.get(&dd.drop_table)
                .expect(&format!("Drop table {:?} not found", dd.drop_table));
            // First, how many things to drop?
            let rng = &mut self.rng;
            let num_drops = rng.gen_range(dd.min_drops, dd.max_drops);
            (0..num_drops).filter_map(|_| {
                let mut rng = rng.clone();
                // Query the drop table
                let probability = Fx32(rng.gen_range(Fx32::new(0.0).0, Fx32::new(10000.0).0));
                drop_table.get_drop(probability)
            }).for_each(|d| {
                let mut rng = rng.clone();
                // Spawn the drops in-world - first choose the amount to drop
                let num = rng.gen_range(d.min_num, d.max_num);

                // Spawn
                let mut builder = lazy_update.create_entity(&*entities)
                    .with(pos.clone())
                    .with(Vel { vel: Vec32::zero() })
                    .with(Pickup { item: inventory::InventoryItem::new(d.item, num) })
                    .with(CollCircle { r: Fx16::new(8.0), off: Vec16::zero(), flags: 0})
                    .with(AnimSprite::new(16.0, 16.0, Fx32::new(40.0), 6, renderer::TextureKey::Coin));
                match d.item.get_in_world_drawable().expect("In-world drawable not found for item") {
                    DrawableComponent::Static(c) => builder = builder.with(c),
                    DrawableComponent::Anim(c) => builder = builder.with(c),
                }
                builder.build();
            })
        }
    }
}
