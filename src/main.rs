use core::time::Duration;
use std::{io::Write, thread};
use rand::{self, thread_rng, Rng};
use clearscreen;

//true = live
//switch to fastrand for smalr exe
const STDDELAY:u64 = 1300; //1300
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
    let mut p1health:i8 = 1;
    let mut p2health:i8 = 1;
    let mut p1turn:bool = true;
    let mut damage: i8 = 1;
    let mut magnified:i8 = -1;
    let mut cuffed = false;
    let mut p1roundwon: u8 = 0;
    let mut p2roundwon: u8 = 0;
    newshells(&mut shells, &mut p1inv, &mut p2inv, &p1roundwon, &p2roundwon);
    loop{

        displayscreen(&p1health, &p2health, &p1inv, &p2inv, &p1turn, &damage, p1roundwon+p2roundwon);
        println!("{:?}", shells);// debug

        println!("[B]EER: racks gun [K]NIFE: deals double damage [M]AGNIFY: says whats in chamber [C]UFFS: SKIPS OTHER PLAYERS TURN [D]URRY: restore 1 health");
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
                'b'|'k'|'m'|'c'|'s'|'o'|'d' => break buff,
                _ => ()
            };
            print!("enter a valid option: ");
            std::io::stdout().flush().unwrap();
        };
        let sheindx:usize;
        if magnified < 0{
            sheindx = thread_rng().gen_range(0..shells.len());
        }
        else{
            sheindx = magnified as usize;
        }
        
        if buff == 'b'{
            if p1turn{
                if p1inv.beers <= 0{
                    println!("not enough beer");
                    thread::sleep(Duration::from_millis(STDDELAY));
                    continue;
                }
                p1inv.beers -=1;
            }
            else {
                if p2inv.beers <= 0{
                    println!("not enough beer");
                    thread::sleep(Duration::from_millis(STDDELAY));
                    continue;
                }
                p2inv.beers -=1;
            }
            println!("the shell was {}", shells.remove(sheindx));
            thread::sleep(Duration::from_millis(STDDELAY));
            if shells.len() == 0{newshells(&mut shells, &mut p1inv, &mut p2inv, &p1roundwon, &p2roundwon);}
            continue;
        }
    
        if buff == 'k'{
            if p1turn{
                if p1inv.knives <= 0{
                    println!("NOT ENOUGH KNIVES");
                    thread::sleep(Duration::from_millis(STDDELAY));
                    continue;
                }
                p1inv.knives-=1;
                damage = 2;
            }
            else{
                if p2inv.knives <= 0{
                    println!("NOT ENOUGH KNIVES");
                    thread::sleep(Duration::from_millis(STDDELAY));
                    continue;
                }
                p2inv.knives-=1;
                damage = 2;
            }
            continue;
        }
    
        if buff == 'm'{
            if p1turn && p1inv.magnify > 0{
                magnified = sheindx as i8;
                println!("there is a {} shell in the chamber", shells[magnified as usize]);
                p1inv.magnify-=1;
            }
            else if !p1turn && p2inv.magnify > 0{
                magnified = sheindx as i8;
                println!("there is a {} shell in the chamber", shells[magnified as usize]);
                p2inv.magnify-=1;
            }
            else{
                println!("NOT ENOUGH MAGNIFYING GLASSES");
            }
            thread::sleep(Duration::from_millis(STDDELAY));
            continue;
        }
        if buff == 'c'{
            cuffed = true;
            if p1turn{
                println!("player 2 cuffed, skip their turn");
                p1inv.cuffs-=1;
            }
            else{
                println!("player 1 cuffed, skip their turn");
                p2inv.cuffs-=1;
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
    
                if shells[sheindx]{
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
                shells.remove(sheindx);
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
                let sheindx:usize = thread_rng().gen_range(0..shells.len());
                if shells[sheindx]{
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
                shells.remove(sheindx);
                damage = 1;
                thread::sleep(Duration::from_millis(1500));
            }
            if shells.len() == 0{
                newshells(&mut shells, &mut p1inv, &mut p2inv, &p1roundwon, &p2roundwon);
                continue;
            }
        }
        magnified = -1;
        if  checkhealths(&mut p1health, &mut p2health, &mut p1roundwon, &mut p2roundwon)
        {newshells(&mut shells, &mut p1inv, &mut p2inv, &p1roundwon, &p2roundwon);}
        
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
        //newshells(&mutshells, p1inv, p2inv, p1roundwon, p2roundwon);
        return false;
    }

    fn newshells(shells: &mut Vec<bool>, p1inv: &mut Items, p2inv: &mut Items, p1roundwon: &u8, p2roundwon: &u8){
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
        for _ in 0..amount{
            shells.push(rand::random());
        }
        thread::sleep(Duration::from_secs(1));
        for i in shells{
            print!("{} ", i);
        }
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis((amount*500) as u64));
        if p1roundwon+p2roundwon == 0 {return;} 
        for _ in 0..((p1roundwon+p2roundwon)*2){
            /////ADD CODE TO ADD RANDOM ITEMS TO BOTH PLAYERS INV
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
            println!("added item");
        }
        //thread::sleep(Duration::from_millis((amount*500) as u64));

    }
}

fn displayscreen(p1health: &i8, p2health: &i8, p1inv: &Items, p2inv: &Items, p1turn: &bool, damage: &i8, round: u8){
    clearscreen::clear().unwrap();
    if *p1turn{
        println!("    TURN    PLAYER 1:");
        println!("HEALTH: {}        ITEMS: {}x beers, {}x knives, {}x magnifying glasses, {}x cuffs, {}x durryss", p1health, p1inv.beers, p1inv.knives, p1inv.magnify, p1inv.cuffs, p1inv.durrys);
        println!("\n         PLAYER 2:");
        println!("HEALTH: {}        ITEMS: {}x beers, {}x knives, {}x magnifying glasses {}x cuffs, {}x durryss", p2health, p2inv.beers, p2inv.knives, p2inv.magnify, p2inv.cuffs, p2inv.durrys);
    }
    else{
        println!("           PLAYER 1:");
        println!("HEALTH: {}        ITEMS: {}x beers, {}x knives, {}x magnifying glasses {}x cuffs, {}x durryss", p1health, p1inv.beers, p1inv.knives, p1inv.magnify, p1inv.cuffs, p1inv.durrys);
        println!("\n    TURN    PLAYER 2:");
        println!("HEALTH: {}        ITEMS: {}x beers, {}x knives, {}x magnifying glasses {}x cuffs, {}x durryss", p2health, p2inv.beers, p2inv.knives, p2inv.magnify, p2inv.cuffs, p2inv.durrys);

    }
    println!("\n           ROUND: {}", round+1);
    if *damage > 1{
        println!("\nDOUBLE DAMAGE!");
    }
    else{
        println!("\n");
    }
}