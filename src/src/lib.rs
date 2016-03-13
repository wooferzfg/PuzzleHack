#![no_std]
#![feature(asm)]

#[macro_use]
extern crate libtww;

use libtww::prelude::*;
use libtww::system;
use libtww::Link;
use libtww::game::Console;
use libtww::link::song;
use libtww::game::flag;
use libtww::game::controller;
use libtww::warping::Warp;

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

    let _ = write!(lines[0].begin(), "Stage:          {}", Link::stage());
    let _ = write!(lines[1].begin(), "Room:           {}", Link::room());
    let _ = write!(lines[2].begin(), "Rupees:         {}", link.rupees);
    let _ = write!(lines[3].begin(), "Sword ID:       {:02X}", link.sword_id);
    let _ = write!(lines[4].begin(), "Shield ID:      {:02X}", link.shield_id);
    let _ = write!(lines[5].begin(), "Max Magic:      {}", link.max_magic);
    let _ = write!(lines[6].begin(), "Magic:          {}", link.magic);
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn init_save_file() {
}

#[no_mangle]
pub extern "C" fn start() {
    game_loop();
    init();
    init_save_file();
}
