use core::time::Duration;
use std::{io::Write, thread};
use rand::{self, thread_rng, Rng};
use clearscreen;

//true = live

// sleep(Duration::from_millis(1000));

// let mut dealerinv:items = items{beers: 0, knives: 0, magnify: 0, cuffs: 0};

struct items{
    beers:u8,
    knives:u8,
    magnify:u8,
    cuffs:u8
}


fn main() {
    let mut p1inv:items = items{beers: 0, knives: 0, magnify: 0, cuffs: 0};
    let mut p2inv:items = items{beers: 0, knives: 0, magnify: 0, cuffs: 0};
    let mut shells:Vec<bool> = Vec::new();
    let mut p1health:u8 = 4;
    let mut p2health:u8 = 4;
    let mut p1turn:bool = true;
    newshells(&mut shells);
    loop{
        displayscreen(&p1health, &p2health, &p1inv, &p2inv, &p1turn);
        println!("{:?}", shells);
        decidefate(&mut shells, &mut p1inv, &mut p2inv, &mut p1turn, &mut p1health, &mut p2health);
    }
}

fn decidefate(shells: &mut Vec<bool>, p1inv: &mut items, p2inv: &mut items, p1turn: &mut bool, p1health: &mut u8, p2health: &mut u8){
    println!("[B]EER: racks gun [K]NIFE: deals double damage [M]AGNIFY: says whats in chamber [C]UFFS: SKIPS OTHER PLAYERS TURN");
    println!("[S]ELF: shoot self, get an extra turn if blank");
    println!("[O]PPONENT: shoot opponent\n");
    print!("COMMAND: ");
    std::io::stdout().flush().unwrap();
    let mut buff: String = String::new();
    let buff:char = loop{
        buff.clear();
        std::io::stdin().read_line(&mut buff).unwrap();
        buff.pop();
        let buff: char = buff.to_ascii_lowercase().chars().nth(0).unwrap();
        match buff{
            'b'|'k'|'m'|'c'|'s'|'o' => break buff,
            _ => ()
        };
        print!("enter a valid option: ");
        std::io::stdout().flush().unwrap();
    };
    if *p1turn{
        *p1turn = false;
        if buff == 's'{
            print!(".");
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(1000));
            print!(".");
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(1000));
            println!(".");
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(1000));
            let sheindx:usize = thread_rng().gen_range(0..shells.len());
            if shells[sheindx]{
                *p1health-=1;
                println!("BANG!");
            }
            else{
                println!("CLICK!");
                *p1turn = true;
            }
            shells.remove(sheindx);
            thread::sleep(Duration::from_millis(1500));
        }
        if shells.len() == 0{
            newshells(shells);
            return;
        }
    }
    else{
        *p1turn = true;
        if buff == 's'{
            print!(".");
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(1000));
            print!(".");
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(1000));
            println!(".");
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(1000));
            let sheindx:usize = thread_rng().gen_range(0..shells.len());
            if shells[sheindx]{
                *p2health-=1;
                println!("BANG!");
            }
            else{
                println!("CLICK!");
                *p1turn = false;
            }
            shells.remove(sheindx);
            thread::sleep(Duration::from_millis(1500));
        }
        if shells.len() == 0{
            newshells(shells);
            return;
        }
    }
    
}

fn newshells(shells: &mut Vec<bool>){
    clearscreen::clear().unwrap();
    println!("loading shells...");
    let amount:usize = thread_rng().gen_range(2..=8);
    for i in 0..amount{
        shells.push(rand::random());
    }
    thread::sleep(Duration::from_secs(1));
    for i in shells{
        print!("{} ", i);
    }
    std::io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis((amount*500) as u64));
}

fn displayscreen(p1health: &u8, p2health: &u8, p1inv: &items, p2inv: &items, p1turn: &bool){
    clearscreen::clear().unwrap();
    if *p1turn{
        println!("    TURN    PLAYER 1:");
        println!("HEALTH: {}        ITEMS: {}x beers, {}x knives, {}x magnifying glasses {}x cuffs", p1health, p1inv.beers, p1inv.knives, p1inv.magnify, p1inv.cuffs);
        println!("\n         PLAYER 2:");
        println!("HEALTH: {}        ITEMS: {}x beers, {}x knives, {}x magnifying glasses {}x cuffs", p2health, p2inv.beers, p2inv.knives, p2inv.magnify, p2inv.cuffs);
    }
    else{
        println!("           PLAYER 1:");
        println!("HEALTH: {}        ITEMS: {}x beers, {}x knives, {}x magnifying glasses {}x cuffs", p1health, p1inv.beers, p1inv.knives, p1inv.magnify, p1inv.cuffs);
        println!("\n    TURN    PLAYER 2:");
        println!("HEALTH: {}        ITEMS: {}x beers, {}x knives, {}x magnifying glasses {}x cuffs", p2health, p2inv.beers, p2inv.knives, p2inv.magnify, p2inv.cuffs);

    }
    println!("\n\n");
}