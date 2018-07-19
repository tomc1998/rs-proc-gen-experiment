//! Module for controller systems - either AI or input controlled

mod slime_ai;

pub use self::slime_ai::SlimeAISys;

use DeltaTime;
use input;
use specs::*;
use comp::*;
use renderer::TextureKey;
use fpa::*;
use fpavec::*;

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
                    vel.vel.y = -pc.move_speed.to_fx32();
                    anim_change = Some(TextureKey::Human00WalkUp);
                }
                else if *input_state.down.get(&input::Command::MoveDown).unwrap() {
                    vel.vel.y = pc.move_speed.to_fx32();
                    anim_change = Some(TextureKey::Human00WalkDown);
                }
                else {
                    vel.vel.y = Fx32::new(0.0);
                }
                if *input_state.down.get(&input::Command::MoveLeft).unwrap() {
                    vel.vel.x = -pc.move_speed.to_fx32();
                    anim_change = Some(TextureKey::Human00WalkLeft);
                }
                else if *input_state.down.get(&input::Command::MoveRight).unwrap() {
                    vel.vel.x = pc.move_speed.to_fx32();
                    anim_change = Some(TextureKey::Human00WalkRight);
                }
                else {
                    vel.vel.x = Fx32::new(0.0);
                }
                // Attack
                if *input_state.pressed.get(&input::Command::Primary).unwrap() {
                    vel.vel.x = Fx32::new(0.0);
                    vel.vel.y = Fx32::new(0.0);
                    pc.state = PlayerState::Attacking;
                    pc.attack_time = Fx16::new(125.0);
                    // Figure out attack dir
                    let vec = input_state.mouse - pos.pos;
                    let dir = if vec.x.abs() > vec.y.abs() {
                        if vec.x.0 > 0 {
                            Direction::Right
                        } else {
                            Direction::Left
                        }
                    } else {
                        if vec.y.0 > 0 {
                            Direction::Down
                        } else {
                            Direction::Up
                        }
                    };

                    // Change anim
                    anim_change = None;
                    match dir {
                        Direction::Left => anim.set_anim(TextureKey::Human00AttackLeft,
                                                         1, Fx32::new(1000.0)),
                        Direction::Right => anim.set_anim(TextureKey::Human00AttackRight,
                                                         1, Fx32::new(1000.0)),
                        Direction::Up => anim.set_anim(TextureKey::Human00AttackUp,
                                                         1, Fx32::new(1000.0)),
                        Direction::Down => anim.set_anim(TextureKey::Human00AttackDown,
                                                         1, Fx32::new(1000.0)),
                    }

                    // Spawn attack
                    lazy_update.create_entity(&*entities_s)
                        .with(Pos {
                            pos: pos.pos + match dir {
                                Direction::Down   => Vec32::new(Fx32::new(0.0),   Fx32::new(20.0)),
                                Direction::Left   => Vec32::new(Fx32::new(-16.0), Fx32::new(8.0)),
                                Direction::Up     => Vec32::new(Fx32::new(0.0),   Fx32::new(8.0)),
                                Direction::Right  => Vec32::new(Fx32::new(16.0),  Fx32::new(8.0)),
                            }
                        })
                        .with(Hurt { damage: 2,
                                     mask: Hitmask::default_player_attack(),
                                     flags: 0 })
                        .with(CollCircle {
                            r: Fx16::new(40.0),
                            off: Vec16::zero(),
                            flags: 0,
                        })
                        .with(HurtKnockbackDir {
                            knockback: match dir {
                                Direction::Down   => Vec16::new(Fx16::new(0.0),   Fx16::new(500.0)),
                                Direction::Left   => Vec16::new(Fx16::new(-500.0), Fx16::new(0.0)),
                                Direction::Up     => Vec16::new(Fx16::new(0.0),   Fx16::new(-500.0)),
                                Direction::Right  => Vec16::new(Fx16::new(500.0),  Fx16::new(0.0)),
                            },
                            duration: Fx32::new(96.0)
                        })
                        .with(Lifetime { lifetime: Fx32::new(125.0) })
                        .with(AnimSprite::new(64.0, 64.0, Fx32::new(25.0), 5,
                                              match dir {
                                                  Direction::Down  => TextureKey::Slice00Down,
                                                  Direction::Left  => TextureKey::Slice00Left,
                                                  Direction::Up    => TextureKey::Slice00Up,
                                                  Direction::Right => TextureKey::Slice00Right,
                                              })
                              .with_flags(ANIM_SPRITE_NO_LOOP))
                        .build();
                }
                if let Some(anim_change) = anim_change {
                    anim.set_anim(anim_change, 4, Fx32::new(150.0));
                } else if vel.vel.x == Fx32::new(0.0) && vel.vel.y == Fx32::new(0.0)
                    && pc.state == PlayerState::Default {
                    match anim.anim {
                        TextureKey::Human00WalkLeft | TextureKey::Human00AttackLeft =>
                            anim.set_anim(TextureKey::Human00IdleLeft, 1, Fx32::new(1000.0)),
                        TextureKey::Human00WalkRight | TextureKey::Human00AttackRight =>
                            anim.set_anim(TextureKey::Human00IdleRight, 1, Fx32::new(1000.0)),
                        TextureKey::Human00WalkUp | TextureKey::Human00AttackUp =>
                            anim.set_anim(TextureKey::Human00IdleUp, 1, Fx32::new(1000.0)),
                        TextureKey::Human00WalkDown | TextureKey::Human00AttackDown =>
                            anim.set_anim(TextureKey::Human00IdleDown, 1, Fx32::new(1000.0)),
                        _ => ()
                    }
                }
            } else if pc.state == PlayerState::Attacking {
                // Count down attack timer
                pc.attack_time -= delta.0 * 1000.0;
                if pc.attack_time.0 <= 0 {
                    pc.state = PlayerState::Default;
                }
            }
        }
    }
}
