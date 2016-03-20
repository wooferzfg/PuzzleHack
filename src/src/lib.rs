#![no_std]
#![feature(asm)]

#[macro_use]
extern crate libtww;

use libtww::prelude::*;
use libtww::{system, link, Link};
use libtww::link::inventory::Inventory;
use libtww::link::item;
use libtww::link::equips::Equips;
use libtww::link::quest_items::Sword;
use libtww::game::{Console, controller, flag};
use libtww::warping::{Entrance, Warp, stage};

#[no_mangle]
#[inline(never)]
pub extern "C" fn init() {
    // Call overriden instruction
    system::cdyl_init_async();

    Console::get().setup();
}

pub static mut hasSword : bool = true;
pub static mut adjusted_index : u8 = 0;

#[no_mangle]
#[inline(never)]
pub extern "C" fn game_loop() {
    let link = Link::get();
    let console = Console::get();
    let inventory = Inventory::get();
    let equips = Equips::get();

    let mut lines = &mut console.lines;

    let exit = Warp::last_exit();

    if exit.entrance.stage_name() == stage::sea::SEA && Entrance::last_entrance().stage_name() == stage::other::NAME_SELECT && !flag::HAS_SEEN_INTRO.is_active()
    {
        link.set_sword(Sword::HerosSword);
        let warp = Warp::new(stage::earth_temple::TEMPLE, 1, 1, exit.layer_override, exit.fadeout, exit.enabled);
        warp.execute();
    }
    else if exit.entrance.stage_name() == stage::sea::SEA && exit.entrance.room == 44 //game over or save warp
    {
        let _ = write!(lines[0].begin(), "you tried :^)");
        let warp = Warp::new(stage::dev::LARGE_EMPTY_ROOM, 0, 0, exit.layer_override, exit.fadeout, true);
        warp.execute();
    }
    else if Entrance::last_entrance().stage_name() == stage::earth_temple::TEMPLE && exit.entrance.room == Entrance::last_entrance().room
    {
        if unsafe {hasSword}
        {
            let _ = write!(lines[0].begin(), "- Backwards is Forwards -");
        }
        flag::HAS_SEEN_INTRO.activate();
        if Link::room() == 0 && exit.entrance.room == Entrance::last_entrance().room
        {
            unsafe {hasSword = false;}
            let _ = write!(lines[0].begin(), "- Do Not Pass Go, Do Not Collect 200 Rupees -");
            link.set_sword(Sword::None);
            let warp = Warp::new(stage::earth_temple::TEMPLE, 1, 1, exit.layer_override, exit.fadeout, true);
            warp.execute();
        }
        else if Link::room() == 1
        {
            let warp = Warp::new(stage::earth_temple::TEMPLE, exit.entrance.entrance, exit.entrance.room, exit.layer_override, exit.fadeout, false);
            warp.execute();
        }
    }
    else if Entrance::last_entrance().stage_name() == stage::forsaken_fortress::FF1_INTERIOR
    {
        if !flag::GRABBED_FIRST_ROPE_IN_FF1.is_active()
        {
            let _ = write!(lines[0].begin(), "- On the Ropes Without That Button -");
            link.heart_quarters = 12;
            Link::set_collision(link::CollisionType::DoorCancel);
        }
        else
        {
            if link.heart_quarters < 12
            {
                let _ = write!(lines[0].begin(), "- Fuck Doors, Get Paid -");
                Link::set_collision(link::CollisionType::ChestStorage);
            }
            else
            {
                let _ = write!(lines[0].begin(), "- No Pain, No Gain -");
                Link::set_collision(link::CollisionType::Default);
            }
        }
    }
    else if Entrance::last_entrance().stage_name() == stage::forbidden_woods::BOSS
    {
        let _ = write!(lines[0].begin(), "- Unconventional Methods -");
        Link::set_collision(link::CollisionType::ChestStorage);
        inventory.deku_leaf_slot = item::DEKU_LEAF;
        link.magic = 16;
        link.max_magic = 16;
        link.set_sword(Sword::UnchargedMasterSword);
    }
    else if Entrance::last_entrance().stage_name() == stage::outset::UNDER_LINKS_HOUSE
    {
        let _ = write!(lines[0].begin(), "- Bombs = The Answer to Life -");
        link.set_sword(Sword::None);
        inventory.deku_leaf_slot = item::EMPTY;
        link.magic = 0;
        link.max_magic = 0;
        inventory.bombs_slot = item::BOMBS;
        inventory.tingle_tuner_slot = item::TINGLE_TUNER;
    }
    else if Entrance::last_entrance().stage_name() == stage::dragon_roost_island::POND
    {
        let _ = write!(lines[0].begin(), "- X Controls The Items -");
        
        link.magic = 16;
        link.max_magic = 16;
        inventory.bomb_count = 99;

        inventory.telescope_slot = item::TELESCOPE;
        inventory.sail_slot = item::SAIL;
        inventory.wind_waker_slot = item::WIND_WAKER;
        inventory.grappling_hook_slot = item::GRAPPLING_HOOK;
        inventory.tingle_tuner_slot = item::TINGLE_TUNER;
        inventory.picto_box_slot = item::PICTO_BOX;
        inventory.iron_boots_slot = item::IRON_BOOTS;
        inventory.magic_armor_slot = item::MAGIC_ARMOR;

        let new_adjusted_index = adjust_right_index(adjust_left_index(equips.x_index));

        if new_adjusted_index != unsafe {adjusted_index}
        {
            Inventory::set_by_slot_id(unsafe{adjusted_index} as usize, item::EMPTY);
            Inventory::set_by_slot_id(new_adjusted_index as usize, item_id_for_slot(new_adjusted_index));
            unsafe{adjusted_index = new_adjusted_index;}
        }
    }
    else 
    {
        lines[0].clear();
    }
}

//adjust the inventory index for left side of the inventory so that it ranges from 1 to 8
pub fn adjust_left_index(index: u8) -> u8 {
    let mut adjusted = 0;
    if index != 255
    {
        if index < 4
        {
            adjusted = index + 1;
        }
        else if index >= 7 && index < 11
        {
            adjusted = index - 2;
        }
    }
    adjusted
}

//adjust the 0 to 8 index so it corresponds to an index on the right side of the inventory
pub fn adjust_right_index(index: u8) -> u8 {
    if index < 3
    {
        return index + 4;
    }
    else if index < 6
    {
        return index + 8;
    }
    else 
    {
        return index + 12;
    }
}

pub fn item_id_for_slot(index: u8) -> u8 {
    match index {
        4 => item::SPOILS_BAG,
        5 => item::BOOMERANG,
        6 => item::DEKU_LEAF,
        11 => item::BAIT_BAG,
        12 => item::BOW,
        13 => item::BOMBS,
        18 => item::DELIVERY_BAG,
        19 => item::HOOKSHOT,
        20 => item::SKULL_HAMMER,
        _ => item::EMPTY,
    }
}

pub fn mask_all_buttons(mask: u16) {
    use libtww::system::memory::{read, write};

    let m1: u16 = read(0x803E0D2A);
    let m2: u16 = read(0x803E0D2E);
    let m3: u16 = read(0x803E0CF8);
    let m4: u16 = read(0x803E0D42);

    write(0x803E0D2A, m1 & mask);
    write(0x803E0D2E, m2 & mask);
    write(0x803E0CF8, m3 & mask);
    write(0x803E0D42, m4 & mask);
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn set_control_stuff() {
    //modify controller inputs
    if Entrance::last_entrance().stage_name() == stage::forsaken_fortress::FF1_INTERIOR
    {
        mask_all_buttons(!controller::A);
    }
    //replace warps
    let inventory = Inventory::get();
    let exit = Warp::last_exit();
    if exit.entrance.stage_name() == stage::forsaken_fortress::FF1
    {
        let warp = Warp::new(stage::forbidden_woods::BOSS, 0, 0, exit.layer_override, exit.fadeout, true);
        warp.execute();
    }
    else if exit.entrance.stage_name() == stage::forest_haven::FOREST_HAVEN
    {
        inventory.bomb_count = 45;
        inventory.bomb_capacity = 99;
        let warp = Warp::new(stage::outset::UNDER_LINKS_HOUSE, 0, 0, exit.layer_override, exit.fadeout, true);
        warp.execute();
    }
    else if exit.entrance.stage_name() == stage::sea::SEA && Entrance::last_entrance().stage_name() == stage::outset::UNDER_LINKS_HOUSE
    {
        if inventory.bomb_count == 42
        {
            inventory.bombs_slot = item::EMPTY;
            let warp = Warp::new(stage::dragon_roost_island::POND, 1, 0, exit.layer_override, exit.fadeout, true);
            warp.execute();
        }
        else 
        {
            inventory.bomb_count = 45;
            let warp = Warp::new(stage::outset::UNDER_LINKS_HOUSE, 0, 0, exit.layer_override, exit.fadeout, true);
            warp.execute();
        }
    }
    else if exit.entrance.stage_name() == stage::dragon_roost_island::POSTAL_SERVICE
    {
        let warp = Warp::new(stage::dragon_roost_island::POND, 1, 0, exit.layer_override, exit.fadeout, true);
        warp.execute();
    }
}

#[no_mangle]
pub extern "C" fn start() {
    game_loop();
    init();
    set_control_stuff();
}
