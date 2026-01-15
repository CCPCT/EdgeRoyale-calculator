use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Spell {
    name: String,
    damage: i32,
    enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
    threshold_damage: i32,
    max_iterations: i32,
    spells: Vec<Spell>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            threshold_damage: 3,
            max_iterations: 10000,
            spells: vec![
                Spell { name: "rocket".to_string(), damage: 371, enabled: true },
                Spell { name: "poison".to_string(), damage: 184, enabled: true },
                Spell { name: "void 1".to_string(), damage: 144, enabled: true },
                Spell { name: "vines".to_string(), damage: 76, enabled: true },
                Spell { name: "void 2".to_string(), damage: 75, enabled: true },
                Spell { name: "log".to_string(), damage: 41, enabled: true },
                Spell { name: "tornado".to_string(), damage: 25, enabled: true }
            ],
        }
    }
}

fn get_config() -> AppConfig {
    let path = "config.json";

    if !Path::new(path).exists() {
        // create default config file if not exist
        let default_config = AppConfig::default();
        let json = serde_json::to_string_pretty(&default_config).unwrap();
        fs::write(path, json).expect("Unable to write default config");
        println!("Created default config.json");
        return default_config;
    }

    let data = fs::read_to_string(path).expect("Unable to read config");
    let mut config:AppConfig = serde_json::from_str(&data).expect("Config JSON was poorly formatted... \nDelete config folder to generate a new one");

    // filter out disabled
    config.spells.retain(|s| s.enabled);

    // sort it in descending order
    config.spells.sort_by(|a, b| b.damage.cmp(&a.damage));

    config
}

fn cal(config: &AppConfig) -> bool {
    // Filter out disabled spells

    if config.spells.is_empty() {
        println!("Error: No spells are enabled in config!");
        return false;
    }

    // input
    print!(">0 for tower health\n0 for exit\n-1 for settings\ninput: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let original_damage: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return true,
    };

    if original_damage == -1 {
        open_config_file("config.json");
        return true; // Return to the start of the loop
    }

    if original_damage == 0 { return false; }

    let mut current_damage = original_damage;
    let mut current_spell_idx = 0;
    let mut tree: Vec<usize> = Vec::new();
    let mut iterations = 0;
    
    let mut solution_tree: Vec<usize> = Vec::new();
    let mut solution_damage: i32 = i32::MAX;

    loop {
        iterations += 1;
        if iterations >= config.max_iterations {
            println!("Ended Search: took too long!");
            break;
        }

        let spell = &config.spells[current_spell_idx];
        current_damage -= spell.damage;
        tree.push(current_spell_idx);

        if current_damage <= 0 {
            
            // Backtrack logic
            while let Some(last_idx) = tree.pop() {
                current_damage += config.spells[last_idx].damage;
                if last_idx + 1 < config.spells.len() {
                    // if not iterate to last element e.g. max 5, 2,1,5,5 -> back trace twice
                    current_spell_idx = last_idx + 1;
                    break;
                }
                if tree.is_empty() {
                    // backtrace to first
                    println!("Ended Search: Looped through all solutions!");
                    print_result(&config.spells, solution_tree, solution_damage);
                    return true;
                }
            }
            
            if current_damage < solution_damage {
                solution_damage = current_damage;
                solution_tree = tree.clone();
            }

            if solution_damage <= config.threshold_damage {
                println!("Ended Search: Found good enough solution!");
                break;
            }

        } else {
            current_spell_idx = 0;
        }
    }

    print_result(&config.spells, solution_tree, solution_damage);
    true
}

fn print_result(active_spells: &[Spell], tree: Vec<usize>, final_damage: i32) {
    println!("\n-------------------");
    if tree.is_empty() {
        println!("No solution.");
        return;
    }
    
    let mut last_idx = -1;
    let mut count = 0;
    for &idx in &tree {
        if last_idx == idx as i32 {
            count += 1;
        } else {
            if last_idx != -1 {
                let name: String = active_spells[last_idx as usize].name.clone();
                let damage: i32 = active_spells[last_idx as usize].damage;
                if count == 1 {
                    println!("{:>8}    | {}", name, damage);
                } else {
                    println!("{:>8} x{:2}| {} ", name, count, damage * count);
                }
            }
            last_idx = idx as i32;
            count = 1;
        }
    }
    let name: String = active_spells[last_idx as usize].name.clone();
    let damage: i32 = active_spells[last_idx as usize].damage;
    if count == 1 {
        println!("{:>8}    | {}", name, damage);
    } else {
        println!("{:>8} x{:2}| {} ", name, count, damage * count);
    }
    println!("\nRemaining: {}", final_damage);
    println!("--------------------\n");
}

fn open_config_file(path: &str) {
    // open config file for user to edit
    opener::open(path).expect("Error: Could not open file!");
    println!("Opened config file!");
    print!("Press Enter to continue... (ignore output of the text editor if present)");
    io::stdout().flush().unwrap();
    let mut discard = String::new();
    io::stdin().read_line(&mut discard).unwrap();
}
fn main() {
    loop {
        if !cal(&get_config()) { break; }
    }
}
