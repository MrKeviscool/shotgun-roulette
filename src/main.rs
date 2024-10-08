use core::time::Duration;
use std::{io::Write, process::exit, thread};
use rand::{self, thread_rng, Rng, seq::SliceRandom};
use clearscreen;

const STDDELAY:u64 = 1500; //1300
struct Items{
    beers:u8,
    knives:u8,
    magnify:u8,
    cuffs:u8,
    durrys:u8
}


fn main() {
    let mut p1inv:Items = Items{beers: 0, knives: 0, magnify: 0, cuffs: 0, durrys: 0};
    let mut p2inv:Items = Items{beers: 0, knives: 0, magnify: 0, cuffs: 0, durrys: 0};
    let mut shells:Vec<bool> = Vec::new();
    let mut p1health:i8 = 4;
    let mut p2health:i8 = 4;
    let mut p1turn:bool = true;
    let mut damage: i8 = 1;
    let mut cuffed = false;
    let mut p1roundwon: u8 = 0;
    let mut p2roundwon: u8 = 0;
    newshells(&mut shells, &mut p1inv, &mut p2inv, &p1roundwon, &p2roundwon);
    loop{
        displayscreen(&p1health, &p2health, &p1inv, &p2inv, &p1turn, &damage, p1roundwon+p2roundwon);
        println!("[B]EER: racks gun [K]NIFE: deals double damage [M]AGNIFY: says whats in chamber [C]UFFS: skips opponents turn [D]URRY: restore 1 health");
        println!("[S]ELF: shoot self, get an extra turn if blank");
        println!("[O]PPONENT: shoot opponent\n");
        //println!("shells: {:?}    shells len {}", shells, shells.len());
        print!("COMMAND: ");
        std::io::stdout().flush().unwrap();

        let mut buff: String = String::new();
        let buff:char = loop{
            buff.clear();
            std::io::stdin().read_line(&mut buff).unwrap();
            buff.pop(); 
            let buff: char = buff.to_ascii_lowercase().chars().nth(0).unwrap();
            match buff{
                'b'|'k'|'m'|'c'|'s'|'o'|'d'|'q' => break buff,
                _ => ()
            };
            print!("enter a valid option: ");
            std::io::stdout().flush().unwrap();
        };
        if buff == 'm'{
            if p1turn && p1inv.magnify > 0{
                println!("there is a {} shell in the chamber", shells[shells.len()-1]);
                p1inv.magnify-=1;
            }
            else if !p1turn && p2inv.magnify > 0{
                println!("there is a {} shell in the chamber", shells[shells.len()-1]);
                p2inv.magnify-=1;
            }
            else{
                println!("NOT ENOUGH MAGNIFYING GLASSES");
            }
            thread::sleep(Duration::from_millis(STDDELAY));
            continue;
        }
        
        if buff == 'b'{
            if p1turn && p1inv.beers > 0{
                println!("the shell was {}", shells.pop().unwrap());
                p1inv.beers-=1;
            }
            else if !p1turn && p2inv.beers > 0{
                println!("the shell was {}", shells.pop().unwrap());
                p2inv.beers-=1;
            }
            else{
                println!("not enough beer");
            }
            if shells.len() <= 0{
                newshells(&mut shells, &mut p1inv, &mut p2inv, &p1roundwon, &p2roundwon);
            }
            thread::sleep(Duration::from_millis(STDDELAY));
            continue;
        }
    
        if buff == 'k'{
            if p1turn{
                if p1inv.knives <= 0{
                    println!("not enough knives");
                    thread::sleep(Duration::from_millis(STDDELAY));
                    continue;
                }
                p1inv.knives-=1;
                damage = 2;
            }
            else{
                if p2inv.knives <= 0{
                    println!("not enough knives");
                    thread::sleep(Duration::from_millis(STDDELAY));
                    continue;
                }
                p2inv.knives-=1;
                damage = 2;
            }
            continue;
        }
    
        if buff == 'c'{
            if p1turn && p1inv.cuffs > 0{
                cuffed = true;
                println!("player two cuffed, skip their turn");
                p1inv.cuffs-=1;
            }
            else if !p1turn && p2inv.cuffs > 0{
                cuffed = true;
                println!("player one cuffed, skip their turn");
                p2inv.cuffs-=1;
            }
            else{
                println!("not enough cuffs");
            }
            thread::sleep(Duration::from_millis(STDDELAY));
            continue;
        }

        if buff == 'd'{
            if p1turn && p1inv.durrys > 0{
                p1health+=1;
                p1inv.durrys-=1;
                println!("blazed a durry, +1 health")
            }
            else if !p1turn && p2inv.durrys > 0{
                p2health+=1;
                p2inv.durrys-=1;
                println!("blazed a durry, +1 health")
            }
            else{
                println!("NOT ENOUGH DURRYS");
            }
            thread::sleep(Duration::from_millis(STDDELAY));
            continue;
        }
        if buff == 'q'{
            exit(0);
        }
        if p1turn{
            if !cuffed{
                p1turn = false;
            }
            if buff == 's'|| buff =='o' {
                cuffed = false;
                print!(".");
                std::io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(1000));
                print!(".");
                std::io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(1000));
                println!(".");
                std::io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(1000));
    
                if shells.pop().unwrap(){
                    println!("BANG!");
                    if buff == 's'{p1health-=damage;}
                    else{p2health-=damage;}
                }
                else{
                    println!("CLICK!");
                    if buff=='s'{
                        p1turn = true;
                    }
                }
                damage = 1;
                thread::sleep(Duration::from_millis(1500));
            }
            if shells.len() == 0{
                newshells(&mut shells, &mut p1inv, &mut p2inv, &p1roundwon, &p2roundwon);
                continue;
            }
        }
        else{
            if !cuffed{
                p1turn = true;
            }
            if buff == 's'|| buff =='o' {
                cuffed = false;
                print!(".");
                std::io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(1000));
                print!(".");
                std::io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(1000));
                println!(".");
                std::io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(1000));
    
                if shells.pop().unwrap(){
                    println!("BANG!");
                    if buff == 's'{p2health-=damage;}
                    else{p1health-=damage;}
                }
                else{
                    println!("CLICK!");
                    if buff=='s'{
                        p1turn = false;
                    }
                }
                damage = 1;
                thread::sleep(Duration::from_millis(1500));
            }
            if shells.len() == 0{
                newshells(&mut shells, &mut p1inv, &mut p2inv, &p1roundwon, &p2roundwon);
                continue;
            }
        }
        if checkhealths(&mut p1health, &mut p2health, &mut p1roundwon, &mut p2roundwon){
            if p1roundwon+p2roundwon > 2{
                if p1roundwon > p2roundwon{
                    endgame(true);
                }
                else{
                    endgame(false);
                }
                continue;
            }    
            newshells(&mut shells, &mut p1inv, &mut p2inv, &p1roundwon, &p2roundwon);
        }
        
    }
    

}

fn displayscreen(p1health: &i8, p2health: &i8, p1inv: &Items, p2inv: &Items, p1turn: &bool, damage: &i8, round: u8){
    clearscreen::clear().unwrap();
    if *p1turn{
        println!("    TURN    PLAYER 1:");
        println!("HEALTH: {}        ITEMS: {}x beers, {}x knives, {}x magnifying glasses, {}x cuffs, {}x durrys", p1health, p1inv.beers, p1inv.knives, p1inv.magnify, p1inv.cuffs, p1inv.durrys);
        println!("\n         PLAYER 2:");
        println!("HEALTH: {}        ITEMS: {}x beers, {}x knives, {}x magnifying glasses {}x cuffs, {}x durrys", p2health, p2inv.beers, p2inv.knives, p2inv.magnify, p2inv.cuffs, p2inv.durrys);
    }
    else{
        println!("           PLAYER 1:");
        println!("HEALTH: {}        ITEMS: {}x beers, {}x knives, {}x magnifying glasses {}x cuffs, {}x durrys", p1health, p1inv.beers, p1inv.knives, p1inv.magnify, p1inv.cuffs, p1inv.durrys);
        println!("\n    TURN    PLAYER 2:");
        println!("HEALTH: {}        ITEMS: {}x beers, {}x knives, {}x magnifying glasses {}x cuffs, {}x durrys", p2health, p2inv.beers, p2inv.knives, p2inv.magnify, p2inv.cuffs, p2inv.durrys);

    }
    println!("\n           ROUND: {}", round+1);
    if *damage > 1{
        println!("\nDOUBLE DAMAGE!");
    }
    else{
        println!("\n");
    }

}

fn checkhealths(p1health: &mut i8, p2health: &mut i8, p1roundwon: &mut u8, p2roundwon: &mut u8) -> bool{
    if *p1health <= 0{
        *p2roundwon+=1;
        *p1health = 4;
        *p2health = 4;
        return true;
    }
    else if *p2health <= 0{
        *p1roundwon+=1;
        *p1health = 4;
        *p2health = 4;
        return true;
    }
    return false;
}

fn newshells(shells: &mut Vec<bool>, p1inv: &mut Items, p2inv: &mut Items, p1roundwon: &u8, p2roundwon: &u8){
    clearscreen::clear().unwrap();
    println!("loading shells...");
    let amount:usize = thread_rng().gen_range(2..=6);
    if amount == 2{
        if rand::random(){
            shells.push(true);
            shells.push(false);
        }
        else{
            shells.push(false);
            shells.push(true);
        }
        println!("true false");
        thread::sleep(Duration::from_secs(2));
        return;
    }
    for _ in 0..amount{
        shells.push(rand::random());
    }
    thread::sleep(Duration::from_secs(1));
    for i in shells.iter(){
        print!("{} ", i);
    }

    std::io::stdout().flush().unwrap();
    shells.shuffle(&mut thread_rng());

    thread::sleep(Duration::from_millis((amount*750) as u64));
    if p1roundwon+p2roundwon == 0 {return;} 
    for _ in 0..((p1roundwon+p2roundwon)*2){
        match thread_rng().gen_range(0..=4) {
            0 => p1inv.beers+=1,
            1 => p1inv.knives+=1,
            2 => p1inv.magnify+=1,
            3 => p1inv.cuffs+=1,
            4 => p1inv.durrys+=1,
            _ => panic!("past rnd range")
        };
        match thread_rng().gen_range(0..=4) {
            0 => p2inv.beers+=1,
            1 => p2inv.knives+=1,
            2 => p2inv.magnify+=1,
            3 => p2inv.cuffs+=1,
            4 => p1inv.durrys+=1,
            _ => panic!("past rnd range")
        };
    }

}

fn endgame(p1won: bool){
    if p1won{
        println!("PLAYER ONE WON!!!");
    }
    else{
        println!("PLAYER TWO WON!!!");
    }
}