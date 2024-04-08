use core::time::Duration;
use std::{io::Write, thread};
use rand::{self, thread_rng, Rng};
use clearscreen;

//true = live
//switch to fastrand for smalr exe

struct items{
    beers:u8,
    knives:u8,
    magnify:u8,
    cuffs:u8
}


fn main() {
    let mut p1inv:items = items{beers: 1, knives: 1, magnify: 0, cuffs: 0};
    let mut p2inv:items = items{beers: 1, knives: 1, magnify: 0, cuffs: 0};
    let mut shells:Vec<bool> = Vec::new();
    let mut p1health:u8 = 4;
    let mut p2health:u8 = 4;
    let mut p1turn:bool = true;
    let mut damage: u8 = 1;
    let mut magnified:i8 = -1;
    newshells(&mut shells);
    loop{
        displayscreen(&p1health, &p2health, &p1inv, &p2inv, &p1turn, &damage);
        println!("{:?}", shells);/// debug
        decidefate(&mut shells, &mut p1inv, &mut p2inv, &mut p1turn, &mut p1health, &mut p2health, &mut damage, &mut magnified);
    }
}

fn decidefate(shells: &mut Vec<bool>, p1inv: &mut items, p2inv: &mut items, p1turn: &mut bool, p1health: &mut u8, p2health: &mut u8, damage: &mut u8, magnified: &mut i8){
    const STDDELAY:u64 = 1300;
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
    let mut sheindx:usize;
    if *magnified < 0{
        sheindx = thread_rng().gen_range(0..shells.len());
    }
    else{
        sheindx = *magnified as usize;
    }
    
    if buff == 'b'{
        if *p1turn{
            if p1inv.beers <= 0{
                println!("not enough beer");
                thread::sleep(Duration::from_millis(STDDELAY));
                return;
            }
            p1inv.beers -=1;
        }
        else {
            if p2inv.beers <= 0{
                println!("not enough beer");
                thread::sleep(Duration::from_millis(STDDELAY));
                return;
            }
            p2inv.beers -=1;
        }
        println!("the shell was {}", shells.remove(sheindx));
        thread::sleep(Duration::from_millis(STDDELAY));
        if shells.len() == 0{newshells(shells);}
        return;
    }

    if buff == 'k'{
        if *p1turn{
            if p1inv.knives <= 0{
                println!("not enough knives");
                thread::sleep(Duration::from_millis(STDDELAY));
                return;
            }
            p1inv.knives-=1;
            *damage = 2;
        }
        else{
            if p2inv.knives <= 0{
                println!("not enough knives");
                thread::sleep(Duration::from_millis(STDDELAY));
                return;
            }
            p2inv.knives-=1;
            *damage = 2;
        }
        return;
    }

    if buff == 'm'{
        *magnified = sheindx as i8;
        println!("there is a {} shell in the chamber", shells[*magnified as usize]);
        thread::sleep(Duration::from_millis(STDDELAY));
        return;
    }

    if *p1turn{
        *p1turn = false;
        if buff == 's'|| buff =='o' {
            print!(".");
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(1000));
            print!(".");
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(1000));
            println!(".");
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(1000));

            if shells[sheindx]{
                println!("BANG!");
                if buff == 's'{*p1health-=*damage;}
                else{*p2health-=*damage;}
            }
            else{
                println!("CLICK!");
                if buff=='s'{
                    *p1turn = true;
                }
            }
            shells.remove(sheindx);
            *damage = 1;
            thread::sleep(Duration::from_millis(1500));
        }
        if shells.len() == 0{
            newshells(shells);
            return;
        }
    }
    else{
        *p1turn = true;
        if buff == 's'|| buff =='o' {
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
                println!("BANG!");
                if buff == 's'{*p2health-=*damage;}
                else{*p1health-=*damage;}
            }
            else{
                println!("CLICK!");
                if buff=='s'{
                    *p1turn = false;
                }
            }
            shells.remove(sheindx);
            *damage = 1;
            thread::sleep(Duration::from_millis(1500));
        }
        if shells.len() == 0{
            newshells(shells);
            return;
        }
    }
    *magnified = -1;
    
}

fn newshells(shells: &mut Vec<bool>){
    clearscreen::clear().unwrap();
    println!("loading shells...");
    let amount:usize = thread_rng().gen_range(2..=8);
    if amount == 2{
        shells.push(true);
        shells.push(false);
        println!("true false");
        thread::sleep(Duration::from_secs(1));
        return;
    }
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

fn displayscreen(p1health: &u8, p2health: &u8, p1inv: &items, p2inv: &items, p1turn: &bool, damage: &u8){
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
    if *damage > 1{
        println!("\nDOUBLE DAMAGE!");
    }
    println!("\n");
}