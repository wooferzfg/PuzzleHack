#![no_std]
#![feature(asm)]

#[macro_use]
extern crate libtww;

use libtww::prelude::*;
use libtww::{system, link, Link, Addr};
use libtww::link::inventory::Inventory;
use libtww::link::item;
use libtww::link::equips::Equips;
use libtww::link::quest_items::Sword;
use libtww::game::windfall_flowers::WindfallFlowers;
use libtww::game::{Console, controller, flag};
use libtww::warping::{Entrance, Warp, stage};
use libtww::system::memory::{read, write};

#[no_mangle]
#[inline(never)]
pub extern "C" fn init() {
    // Call overriden instruction
    system::cdyl_init_async();

    let console = Console::get();
    console.setup();
    console.x = 5;
    console.y = 16;
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
    let stage_name = Entrance::last_entrance().stage_name();

    if exit.entrance.stage_name() == stage::sea::SEA && Entrance::last_entrance().stage_name() == stage::other::NAME_SELECT && !flag::HAS_SEEN_INTRO.is_active()
    {
        link.heart_pieces = 40;
        link.heart_quarters = 40;

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
    else if stage_name == stage::earth_temple::TEMPLE && exit.entrance.room == Entrance::last_entrance().room
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
    else if stage_name == stage::forsaken_fortress::FF1_INTERIOR
    {
        if !flag::GRABBED_FIRST_ROPE_IN_FF1.is_active()
        {
            let _ = write!(lines[0].begin(), "- On the Ropes Without That Button -");
            link.heart_quarters = 40;
            Link::set_collision(link::CollisionType::DoorCancel);
        }
        else
        {
            if link.heart_quarters < 40
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
    else if stage_name == stage::forbidden_woods::BOSS
    {
        let _ = write!(lines[0].begin(), "- Unconventional Methods -");
        Link::set_collision(link::CollisionType::ChestStorage);
        inventory.deku_leaf_slot = item::DEKU_LEAF;
        link.magic = 16;
        link.max_magic = 16;
        link.set_sword(Sword::UnchargedMasterSword);
    }
    else if stage_name == stage::outset::UNDER_LINKS_HOUSE
    {
        let _ = write!(lines[0].begin(), "- Bombs = The Answer to Life -");

        link.set_sword(Sword::None);
        inventory.deku_leaf_slot = item::EMPTY;

        link.magic = 0;
        link.max_magic = 0;
        inventory.bombs_slot = item::BOMBS;
        inventory.tingle_tuner_slot = item::TINGLE_TUNER;
    }
    else if stage_name == stage::cavern::PAWPRINT_ISLE_WIZZROBE
    {
        let _ = write!(lines[0].begin(), "- You Don't Need a Bow For This -");

        inventory.bombs_slot = item::EMPTY;
        inventory.tingle_tuner_slot = item::EMPTY;

        Link::set_collision(link::CollisionType::ChestStorage);
        link.set_sword(Sword::UnchargedMasterSword);
        inventory.deku_leaf_slot = item::DEKU_LEAF;
        link.magic = 16;
        link.max_magic = 16;

        let wizzrobe_memory = read::<u8>(0x803B88B7);
        if wizzrobe_memory == 0x3F
        {
            inventory.deku_leaf_slot = item::EMPTY;
            let warp = Warp::new(stage::dragon_roost_island::POND, 1, 0, exit.layer_override, exit.fadeout, true);
            warp.execute();
        }
    }
    else if stage_name == stage::dragon_roost_island::POND
    {
        let _ = write!(lines[0].begin(), "- X Controls The Items -");
        
        link.set_sword(Sword::None);
        link.magic = 0;
        link.max_magic = 0;

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
    else if stage_name == stage::sea::SEA && Entrance::last_entrance().room == 11 && exit.entrance.stage_name() != stage::other::ENDING
    {
        let _ = write!(lines[0].begin(), "- 100 Rupees -");

        for x in 0..19 {
            Inventory::set_by_slot_id(x, item::EMPTY);
        }
        WindfallFlowers::activate_pedestals();
        inventory.delivery_bag_slot = item::DELIVERY_BAG;
        write::<u8>(0x803B8888, 0x80); //set windfall intro cs to seen
        set_rupees_from_flowers();
        if link.rupees == 100
        {
            let warp = Warp::new(stage::other::ENDING, 0, 0, exit.layer_override, exit.fadeout, true);
            warp.execute();
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
    let m1: u16 = read(0x803E0D2A);
    let m2: u16 = read(0x803E0D2E);
    let m3: u16 = read(0x803E0CF8);
    let m4: u16 = read(0x803E0D42);

    write(0x803E0D2A, m1 & mask);
    write(0x803E0D2E, m2 & mask);
    write(0x803E0CF8, m3 & mask);
    write(0x803E0D42, m4 & mask);
}

pub fn reset_flowers() {
    let flowers = WindfallFlowers::get();
    flowers.shop_left = 0;
    flowers.bench_bush = 0;
    flowers.bench_tree = 0;
    flowers.bench_stone = 0;
    flowers.platform_right = 0;
    flowers.shop_right = 0;
    flowers.platform_left = 0;
    flowers.alley_tree = 0;
    flowers.gate_center_left = 0;
    flowers.gate_left_left = 0;
    flowers.gate_left_right = 0;
    flowers.gate_center_right = 0;
    flowers.gate_right_right = 0;
    flowers.gate_right_left = 0;

    let inventory = Inventory::get();
    for x in 0..8 {
        inventory.delivery_bag.items[x] = item::TOWN_FLOWER;
    }
}

pub fn set_rupees_from_flowers() {
    let flowers = WindfallFlowers::get();
    let mut rupee_count = 0;
    if flowers.bench_bush > 0
    {
        rupee_count += 2;
    }
    if flowers.bench_stone > 0
    {
        rupee_count += 3;
    }
    if flowers.bench_tree > 0
    {
        rupee_count += 5;
    }
    if flowers.shop_left > 0
    {
        rupee_count += 7;
    }
    if flowers.shop_right > 0
    {
        rupee_count += 11;
    }
    if flowers.platform_left > 0
    {
        rupee_count += 13;
    }
    if flowers.platform_right > 0
    {
        rupee_count += 17;
    }
    if flowers.alley_tree > 0
    {
        rupee_count += 19;
    }
    if flowers.gate_right_right > 0
    {
        rupee_count += 23;
    }
    if flowers.gate_right_left > 0
    {
        rupee_count += 29;
    }
    if flowers.gate_center_right > 0
    {
        rupee_count += 31;
    }
    if flowers.gate_center_left > 0
    {
        rupee_count += 37;
    }
    if flowers.gate_left_right > 0
    {
        rupee_count += 41;
    }
    if flowers.gate_left_left > 0
    {
        rupee_count += 43;
    }
    set_rupees(rupee_count);
}

pub fn set_rupees(rupees: u16) 
{
    let link = Link::get();
    let address = unsafe {rupees_left_addr};
    let previous_rupees_left = read::<i32>(address);
    write(address, previous_rupees_left + rupees as i32 - link.rupees as i32);
    link.rupees = rupees;
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
    let stage_name = exit.entrance.stage_name();
    if stage_name == stage::forsaken_fortress::FF1
    {
        let warp = Warp::new(stage::forbidden_woods::BOSS, 0, 0, exit.layer_override, exit.fadeout, true);
        warp.execute();
    }
    else if stage_name == stage::forest_haven::FOREST_HAVEN
    {
        inventory.bomb_count = 45;
        inventory.bomb_capacity = 99;
        let warp = Warp::new(stage::outset::UNDER_LINKS_HOUSE, 0, 0, exit.layer_override, exit.fadeout, true);
        warp.execute();
    }
    else if stage_name == stage::sea::SEA && Entrance::last_entrance().stage_name() == stage::outset::UNDER_LINKS_HOUSE
    {
        if inventory.bomb_count == 42
        {
            let warp = Warp::new(stage::cavern::PAWPRINT_ISLE_WIZZROBE, 0, 0, exit.layer_override, exit.fadeout, exit.enabled);
            warp.execute();
        }
        else 
        {
            inventory.bomb_count = 45;
            let warp = Warp::new(stage::outset::UNDER_LINKS_HOUSE, 0, 0, exit.layer_override, exit.fadeout, true);
            warp.execute();
        }
    }
    else if stage_name == stage::sea::SEA && exit.entrance.room == 12
    {
        let warp = Warp::new(stage::cavern::PAWPRINT_ISLE_WIZZROBE, 0, 0, exit.layer_override, exit.fadeout, exit.enabled);
        warp.execute();
    }
    else if stage_name == stage::dragon_roost_island::POSTAL_SERVICE
    {
        if exit.entrance.entrance == 3
        {
            let warp = Warp::new(stage::dragon_roost_island::POND, 1, 0, exit.layer_override, exit.fadeout, true);
            warp.execute();
        }
        else 
        {
            let warp = Warp::new(stage::dragon_roost_island::POND, 0, 0, exit.layer_override, exit.fadeout, true);
            warp.execute();
        }
    }
    else if stage_name == stage::dragon_roost_cavern::DUNGEON
    {
        reset_flowers();
        let warp = Warp::new(stage::sea::SEA, 0, 11, 6, exit.fadeout, true);
        warp.execute();
    }
    else 
    {
        let mut entrance_num = 0;
        if stage_name == stage::windfall::BOMB_SHOP
        {
            entrance_num = 1;
        }
        else if exit.entrance.stage_name() == stage::windfall::HOUSE_OF_WEALTH
        {
            if exit.entrance.entrance == 0
            {
                entrance_num = 3;
            }
            else 
            {
                entrance_num = 4;
            }
        }
        else if exit.entrance.stage_name() == stage::windfall::CAFE_BAR
        {
            entrance_num = 6;
        }
        else if exit.entrance.stage_name() == stage::windfall::CHU_JELLY_JUICE_SHOP
        {
            entrance_num = 7;
        }
        else if exit.entrance.stage_name() == stage::windfall::GAME_ROOM
        {
            if exit.entrance.entrance == 1
            {
                entrance_num = 8;
            }
            else 
            {
                entrance_num = 9;
            }
        }
        else if exit.entrance.stage_name() == stage::windfall::LENZOS_HOUSE
        {
            entrance_num = 10;
        }
        else if exit.entrance.stage_name() == stage::windfall::SCHOOL_OF_JOY
        {
            entrance_num = 12;
        }
        else if exit.entrance.stage_name() == stage::windfall::JAIL
        {
            entrance_num = 13;
        }
        if entrance_num > 0
        {
            reset_flowers();
            let warp = Warp::new(stage::sea::SEA, entrance_num, 11, 6, exit.fadeout, true);
            warp.execute();
        }
    }
}

pub static mut rupees_left_addr: Addr = 0;

#[no_mangle]
#[inline(never)]
pub extern "C" fn init_rupees(addr: Addr) {
    unsafe {rupees_left_addr = addr + 0x2FFC;}
    system::dmeter_rupy_init(addr);
}

#[no_mangle]
pub extern "C" fn start() {
    game_loop();
    init();
    set_control_stuff();
    init_rupees(0);
}
