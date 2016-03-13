#![no_std]
#![feature(asm)]

#[macro_use]
extern crate libtww;

use libtww::prelude::*;
use libtww::system;
use libtww::Link;
use libtww::game::Console;
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
    let mut lines = &mut console.lines;

    let exit = Warp::last_exit();

    let _ = write!(lines[0].begin(), "Stage:          {}", Entrance::last_entrance().stage_name());
    let _ = write!(lines[1].begin(), "Room:           {}", Entrance::last_entrance().room);
    let _ = write!(lines[2].begin(), "Rupees:         {}", link.rupees);
    let _ = write!(lines[3].begin(), "Sword ID:       {:02X}", link.sword_id);
    let _ = write!(lines[4].begin(), "Shield ID:      {:02X}", link.shield_id);
    let _ = write!(lines[5].begin(), "Max Magic:      {}", link.max_magic);
    let _ = write!(lines[6].begin(), "Magic:          {}", link.magic);
    
    if exit.entrance.stage_name() == stage::sea::SEA && Link::stage() == stage::other::NAME_SELECT && !flag::HAS_SEEN_INTRO.is_active()
    {
        let warp = Warp::new(stage::earth_temple::TEMPLE, 1, 1, exit.layer_override, exit.fadeout, exit.enabled);
        warp.execute();
    }
}

#[no_mangle]
pub extern "C" fn start() {
    game_loop();
    init();
}
