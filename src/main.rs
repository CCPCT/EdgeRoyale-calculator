use std::io::{self, Write};

const SPELL_NAME: [&str; 5] = ["rocket", "poison", "vines", "log", "tornado"];
const SPELL_DAMAGE: [i32; 5] = [371, 184, 76, 41, 25];
const INITIAL_THRESHOLD: i32 = 10;
const MAX_ITERATIONS: i32 = 100_000;

fn cal() -> bool {
    print!("Tower Damage: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let original_damage: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return true,
    };

    if original_damage == 0 {
        return false;
    }

    if original_damage <= 25 {
        eprintln!("No possible solution");
        return true;
    }

    let mut tower_damage = original_damage;
    let mut damage_threshold = INITIAL_THRESHOLD;
    let mut current_spell = 0;
    let mut tree: Vec<usize> = Vec::new();
    let mut iterations = 0;

    loop {
        iterations += 1;
        if iterations >= MAX_ITERATIONS {
            eprintln!("Took too long");
            return true;
        }

        if current_spell >= 5 {
            current_spell %= 5;
        }

        if tower_damage > 0 && tower_damage < damage_threshold {
            // Found solution
            break;
        } else if tower_damage <= 0 {
            // Backtrack logic
            if let Some(last_spell_idx) = tree.pop() {
                tower_damage += SPELL_DAMAGE[last_spell_idx];
                
                if last_spell_idx == 4 {
                    if tree.is_empty() {
                        println!("Cant find solution for threshold: {}", damage_threshold);
                        damage_threshold += 10;
                        current_spell = 0;
                        tower_damage = original_damage;
                        continue;
                    }
                    // Continue popping if we hit the end of the spell list
                    current_spell = tree.pop().unwrap() + 1;
                    tower_damage += SPELL_DAMAGE[current_spell - 1];
                } else {
                    current_spell = last_spell_idx + 1;
                }
            }
        } else {
            // Apply spell
            tower_damage -= SPELL_DAMAGE[current_spell];
            tree.push(current_spell);
        }
    }

    println!("Found Solution in threshold: {}", damage_threshold);
    for &spell_idx in &tree {
        println!("{}", SPELL_NAME[spell_idx]);
    }
    println!("Remain health: {}", tower_damage);
    true
}

fn main() {
    loop {
        if !cal() {
            break;
        }
    }
}