#![no_std]
#![feature(asm)]

#[macro_use]
extern crate libtww;

use libtww::prelude::*;
use libtww::system;
use libtww::link;
use libtww::Link;
use libtww::link::inventory::Inventory;
use libtww::link::item;
use libtww::game::Console;
use libtww::game::controller;
use libtww::game::flag;
use libtww::warping::Entrance;
use libtww::warping::Warp;
use libtww::warping::stage;

#[no_mangle]
#[inline(never)]
pub extern "C" fn init() {
    // Call overriden instruction
    system::cdyl_init_async();

    Console::get().setup();
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn game_loop() {
    let link = Link::get();
    let console = Console::get();
    let inventory = Inventory::get();
    let mut lines = &mut console.lines;

    let exit = Warp::last_exit();
    
    if exit.entrance.stage_name() == stage::sea::SEA && Link::stage() == stage::other::NAME_SELECT && !flag::HAS_SEEN_INTRO.is_active()
    {
        system::memory::write(0x803B81BC, 0x01000000); //hero's sword
        let warp = Warp::new(stage::earth_temple::TEMPLE, 1, 1, exit.layer_override, exit.fadeout, exit.enabled);
        warp.execute();
    }
    if Link::stage() == stage::earth_temple::TEMPLE && exit.entrance.room == Entrance::last_entrance().room
    {
        let _ = write!(lines[0].begin(), "Puzzle 1: Backwards is Forwards");

        if Link::room() == 0 && exit.entrance.room == Entrance::last_entrance().room
        {
            system::memory::write(0x803B81BC, 0x00000000); //hero's sword off
            let warp = Warp::new(stage::earth_temple::TEMPLE, 1, 1, exit.layer_override, exit.fadeout, true);
            warp.execute();
        }
        else if Link::room() == 1
        {
            let warp = Warp::new(stage::earth_temple::TEMPLE, exit.entrance.entrance, exit.entrance.room, exit.layer_override, exit.fadeout, false);
            warp.execute();
        }
    }

    if Link::stage() == stage::forsaken_fortress::FF1_INTERIOR
    {

        if !flag::GRABBED_FIRST_ROPE_IN_FF1.is_active()
        {
            let _ = write!(lines[0].begin(), "Puzzle 2: On the Ropes Without That Button");
            Link::set_collision(link::CollisionType::DoorCancel);
        }
        else
        {
            let _ = write!(lines[0].begin(), "Puzzle 3: No Pain, No Gain");
            if link.heart_quarters < 12
            {
                Link::set_collision(link::CollisionType::ChestStorage);
            }
            else
            {
                Link::set_collision(link::CollisionType::Default);
            }
        }
    }

}

#[no_mangle]
#[inline(never)]
pub extern "C" fn set_controller() {
    if Link::stage() == stage::forsaken_fortress::FF1_INTERIOR
    {
        controller::mask_all_buttons(!controller::A);
    }
}

#[no_mangle]
pub extern "C" fn start() {
    game_loop();
    init();
    set_controller();
}
