//! Module for controller systems - either AI or input controlled

mod slime_ai;

pub use self::slime_ai::SlimeAISys;

use DeltaTime;
use input;
use specs::*;
use comp::*;
use renderer::{TextureKey, frame_sets::*};
use vec::*;
use std::f32::consts::PI;

pub struct PlayerControllerSys;

pub enum Direction {
    Left, Right, Up, Down
}

impl<'a> System<'a> for PlayerControllerSys {
    type SystemData = (
        Read<'a, LazyUpdate>,
        Read<'a, input::InputState>,
        ReadExpect<'a, DeltaTime>,
        Entities<'a>,
        ReadStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, AnimSprite>,
        WriteStorage<'a, PlayerControlled>);

    fn run(&mut self, (lazy_update, input_state, delta, entities_s,
                       pos_s, mut vel_s, mut anim_s, mut pc_s): Self::SystemData) {
        use specs::Join;
        for (pos, vel, anim, pc) in (&pos_s, &mut vel_s, &mut anim_s, &mut pc_s).join() {
            if pc.state == PlayerState::Default {
                let mut anim_change = None;
                if *input_state.down.get(&input::Command::MoveUp).unwrap() {
                    vel.vel.y = -pc.move_speed;
                    anim_change = Some(FS_HUMAN_WALK_UP);
                }
                else if *input_state.down.get(&input::Command::MoveDown).unwrap() {
                    vel.vel.y = pc.move_speed;
                    anim_change = Some(FS_HUMAN_WALK_DOWN);
                }
                else {
                    vel.vel.y = 0.0;
                }
                if *input_state.down.get(&input::Command::MoveLeft).unwrap() {
                    vel.vel.x = -pc.move_speed;
                    anim_change = Some(FS_HUMAN_WALK_LEFT);
                }
                else if *input_state.down.get(&input::Command::MoveRight).unwrap() {
                    vel.vel.x = pc.move_speed;
                    anim_change = Some(FS_HUMAN_WALK_RIGHT);
                }
                else {
                    vel.vel.x = 0.0;
                }
                // Attack
                if *input_state.pressed.get(&input::Command::Primary).unwrap() {
                    vel.vel.x = 0.0;
                    vel.vel.y = 0.0;
                    pc.state = PlayerState::Attacking;
                    pc.attack_time = 125.0;
                    // Figure out attack dir
                    let vec = input_state.world_mouse - pos.pos;
                    let dir = if vec.x.abs() > vec.y.abs() {
                        if vec.x > 0.0 {
                            Direction::Right
                        } else {
                            Direction::Left
                        }
                    } else {
                        if vec.y > 0.0 {
                            Direction::Down
                        } else {
                            Direction::Up
                        }
                    };

                    // Change anim
                    anim_change = None;
                    match dir {
                        Direction::Left => anim.set_anim(FS_HUMAN_ATTACK_LEFT, 1, 1000.0),
                        Direction::Right => anim.set_anim(FS_HUMAN_ATTACK_RIGHT, 1, 1000.0),
                        Direction::Up => anim.set_anim(FS_HUMAN_ATTACK_UP, 1, 1000.0),
                        Direction::Down => anim.set_anim(FS_HUMAN_ATTACK_DOWN, 1, 1000.0),
                    }

                    // Spawn attack
                    lazy_update.create_entity(&*entities_s)
                        .with(Pos {
                            pos: pos.pos + (vec.nor() * 16.0)
                                // Add a constant offset
                                + Vec32::new(0.0, 16.0)
                        })
                        .with(Hurt { damage: 2,
                                     mask: Hitmask::default_player_attack(),
                                     flags: 0 })
                        .with(CollCircle {
                            r: 40.0,
                            off: Vec32::zero(),
                            flags: 0,
                        })
                        .with(HurtKnockbackDir {
                            knockback: match dir {
                                Direction::Down   => Vec32::new(0.0,   500.0),
                                Direction::Left   => Vec32::new(-500.0, 0.0),
                                Direction::Up     => Vec32::new(0.0,   -500.0),
                                Direction::Right  => Vec32::new(500.0,  0.0),
                            },
                            duration: 96.0
                        })
                        .with(Lifetime { lifetime: 125.0 })
                        .with(AnimSprite::new(64.0, 64.0, 25.0, 5,
                                              TextureKey::SliceAnim)
                              .with_flags(ANIM_SPRITE_NO_LOOP))
                        .with(Rot { angle: vec.angle() - PI / 2.0 })
                        .build();
                }
                if let Some(anim_change) = anim_change {
                    anim.set_anim(anim_change, 4, 150.0);
                } else if vel.vel.x == 0.0 && vel.vel.y == 0.0
                    && pc.state == PlayerState::Default {
                    match anim.anim {
                        FS_HUMAN_WALK_LEFT | FS_HUMAN_ATTACK_LEFT =>
                            anim.set_anim(FS_HUMAN_IDLE_LEFT, 1, 1000.0),
                        FS_HUMAN_WALK_RIGHT | FS_HUMAN_ATTACK_RIGHT =>
                            anim.set_anim(FS_HUMAN_IDLE_RIGHT, 1, 1000.0),
                        FS_HUMAN_WALK_UP | FS_HUMAN_ATTACK_UP =>
                            anim.set_anim(FS_HUMAN_IDLE_UP, 1, 1000.0),
                        FS_HUMAN_WALK_DOWN | FS_HUMAN_ATTACK_DOWN =>
                            anim.set_anim(FS_HUMAN_IDLE_DOWN, 1, 1000.0),
                        _ => ()
                    }
                }
            } else if pc.state == PlayerState::Attacking {
                // Count down attack timer
                pc.attack_time -= delta.0 * 1000.0;
                if pc.attack_time <= 0.0 {
                    pc.state = PlayerState::Default;
                }
            }
        }
    }
}
