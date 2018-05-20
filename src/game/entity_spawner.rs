/*
Copyright <2018> <River Bartz, Daniel Dupriest, Brandon Goldbeck>

Permission is hereby granted, free of charge, to any person obtaining a copy of this 
software and associated documentation files (the "Software"), to deal in the Software 
without restriction, including without limitation the rights to use, copy, modify, 
merge, publish, distribute, sublicense, and/or sell copies of the Software, and to 
permit persons to whom the Software is furnished to do so, subject to the following 
conditions:
The above copyright notice and this permission notice shall be included in all copies
or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, 
INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR 
PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE 
FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR 
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER 
DEALINGS IN THE SOFTWARE.
*/

extern crate ggez;
extern crate rand;
use ggez::{Context,graphics};
use self::rand::Rng;
use game::entity::{Lifetime, Movement, Entity, EntityType};
use game::DEFAULT_FONT;
use game::BULLET_SPEED;
use std;

const ENEMY_FONT_SIZE: u32 = 18;
const ENEMY_COOLDOWN: i64 = 1_000;
const POWERUP_COOLDOWN: i64 = 10_000;
const ENEMY_NAMES: [&str;4] = [
	"NULL POINTER",
	"DANGLING REF",
	"SEGFAULT",
	"DOUBLE FREE",
];

pub struct EntitySpawner {
    //pub max_cooldown: i64,
    //pub current_cooldown: i64,
    pub text: graphics::Text,
    pub rng: rand::ThreadRng,
    pub cooldowns: std::collections::HashMap<EntityType, i64>,
}

impl EntitySpawner {
    pub fn new(ctx: &mut Context) -> EntitySpawner {
        
        let font = graphics::Font::new(ctx, DEFAULT_FONT, 48);
        let text = graphics::Text::new(ctx, "", &font.unwrap()).unwrap();

        let mut p = EntitySpawner {
            //max_cooldown: cooldown,
            //current_cooldown: cooldown,
            text,
            rng: rand::thread_rng(),
            cooldowns: std::collections::HashMap::new(),
        };

        p.cooldowns.insert(EntityType::Enemy, ENEMY_COOLDOWN );
        p.cooldowns.insert(EntityType::Powerup, POWERUP_COOLDOWN );

        p
    }

    // Spawns bullets for the player
    pub fn player_bullet_spawner(&self, x: f32, y: f32) -> Entity {
        let bullet = Entity {
            text: self.text.clone(),
            entity_type: EntityType::PlayerBullet,
            x: x,
            y: y,
            hp: 1,
            dam: 1,
            vel: 10.0,
            bounds: graphics::Rect {
                x: 0.0,
                y: 0.0,
                w: 50.0,
                h: 50.0,
            },
            movement: Movement::Linear(0.0, -BULLET_SPEED),
            //movement: Movement::Linear(0.0, -10_000.0),
            lifetime: Lifetime::Milliseconds(2_000),
            seed: 0.0,
            timer: 0,
            bullet_cooldown: 0,
            angle: 0.0,
        };
        bullet
    }


    // Spawns bullets for the enemy
    pub fn spawn_enemy_bullet(&self, x: f32, y: f32) -> Entity {
        let bullet = Entity {
            text: self.text.clone(),
            entity_type: EntityType::EnemyBullet,
            x,
            y,
            hp: 1,
            dam: 1,
            vel: 1000.0,
            bounds: graphics::Rect {
                x: 0.0,
                y: 0.0,
                w: 25.0,
                h: 25.0,
            },
            //movement: Movement::Linear(0.0, 7_000.0),
            movement: Movement::Linear(0.0, BULLET_SPEED),
            lifetime: Lifetime::Milliseconds(8_000),
            seed: 0.0,
            timer: 0,
            bullet_cooldown: 0,
            angle: 0.0,
        };
        //state.entities.push(bullet);
        bullet
    }

    pub fn spawn_enemy(&self, ctx: &mut Context, seed: f64, name: &str) -> Entity {
        let font = graphics::Font::new(ctx, DEFAULT_FONT, ENEMY_FONT_SIZE);
        //let name = ENEMY_NAMES[self.rng.gen::<usize>() % ENEMY_NAMES.len()].clone();
		let text = graphics::Text::new(ctx, name, &font.unwrap()).unwrap();
		let e = Entity {
            text: text,
            entity_type: EntityType::Enemy,
            x: 0.0,
            y: 0.0,
            hp: 3,
            dam: 1,
            vel: 0.0,
        	bounds: graphics::Rect {
				x: 18.0,
				y: 5.0,
				w: 44.0,
				h: 60.0,
			},
			movement: Movement::Generated(
				|t,r,s|{
 					(
						( ( (t as f64) / 1000.0 + s * 1000.0 ).sin() + r.gen_range(-3.0, 3.0) ) as f32 * 60_f32,
 						(1.0 + ( (t as f64) / 900.0 + s * 100.0).sin() ) as f32 * 60_f32
 					)
 				}
			),
			lifetime: Lifetime::Milliseconds(100_000),
			seed,
			timer: 0,
			bullet_cooldown: 0,
			angle: 0.0,
        };
        // Return powerup entity option type.
        e
    }

    pub fn spawn_powerup(&self) -> Entity {

        let e = Entity {
            text: self.text.clone(),
            entity_type: EntityType::Powerup,
            x: 0.0,
            y: 0.0,
            hp: 1,
            dam: 1,
            vel: 10.0,
        	bounds: graphics::Rect {
				x: 0.0,
				y: 0.0,
				w: 32.0,
				h: 32.0,
			},
			movement: Movement::Linear(0.0, 50.0),
			lifetime: Lifetime::Milliseconds(100_000),
			seed: 0.0,
			timer: 0,
			bullet_cooldown: 0,
			angle: 0.0,
        };
        // Return powerup entity option type.
        e
    }

    pub fn update(&mut self, delta_ms: u64, ctx: &mut Context) -> Option<Entity> {
        
        // We dont really care about matching the player type, so we use that as a dummy.
        let mut entity_type: EntityType = EntityType::Player;

        for (k, v) in self.cooldowns.iter_mut() {
            *v -= delta_ms as i64;
            if *v <= 0 {
                entity_type = k.clone();
            }
        }

        match entity_type {
            EntityType::Enemy => {
                // Reset cooldown.
                self.cooldowns.insert(entity_type, ENEMY_COOLDOWN);
                
                // Create enemy name and seed.
                let name = ENEMY_NAMES[self.rng.gen::<usize>() % ENEMY_NAMES.len()].clone();
                let seed: f64 = self.rng.gen_range(-1.0, 1.0);
                
                // Create enemy.
                let mut entity = self.spawn_enemy(ctx, seed, name);
                entity.x = self.rng.gen_range(0.0, ctx.conf.window_mode.width as f32);
                entity.y = -45.0;
                return Some(entity);
            }
            EntityType::Powerup => {
                // Reset cooldown.
                self.cooldowns.insert(entity_type, POWERUP_COOLDOWN);

                // Create powerup.
                let mut powerup = self.spawn_powerup();
                powerup.x = self.rng.gen_range(0.0, ctx.conf.window_mode.width as f32);
                powerup.y = -45.0;
                return Some(powerup);
            },
            _ => (),
            }                

        None
    }
}
