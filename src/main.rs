use std::io::{self, Write};

const SPELL_NAME: [&str; 5] = ["rocket", "poison", "vines", "log", "tornado"];
const SPELL_DAMAGE: [i32; 5] = [371, 184, 76, 41, 25];
const MAX_ITERATIONS: i32 = 10000;
const DAMAGE_THRESHOLD: i32 = 5;

fn cal() -> bool {
    print!("Tower Damage: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let original_damage: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return true, // Continue loop on invalid input
    };

    if original_damage == 0 {
        return false; // Exit program
    }

    let mut current_damage = original_damage;
    let mut current_spell_idx = 0;
    let mut tree: Vec<usize> = Vec::new();
    let mut iterations = 0;
    
    let mut solution_tree: Vec<usize> = Vec::new();
    let mut solution_damage: i32 = i32::MAX;

    loop {
        iterations += 1;
        if iterations >= MAX_ITERATIONS {
            println!("Ended search: Reached iteration limit.");
            break;
        }

        // Apply current spell
        current_damage -= SPELL_DAMAGE[current_spell_idx];
        tree.push(current_spell_idx);

        if current_damage <= 0 {
            // Check if this overshoot is better than our previous best
            let overshoot = current_damage.abs();
            if overshoot < solution_damage {
                solution_damage = overshoot;
                solution_tree = tree.clone();
            }

            // Stop if we found a "good enough" combo
            if solution_damage <= DAMAGE_THRESHOLD {
            println!("Ended search: Found good enough solution.");
                break;
            }

            // BACKTRACK: Undo spells until we can try a different path
            while let Some(last_idx) = tree.pop() {
                current_damage += SPELL_DAMAGE[last_idx];
                
                // If there is a weaker spell to try at this position, switch to it
                if last_idx + 1 < SPELL_DAMAGE.len() {
                    current_spell_idx = last_idx + 1;
                    break;
                }
                
                // If we've exhausted all spells at this level and the tree is empty, we're done
                if tree.is_empty() {
                    println!("Ended search: Explored all viable combinations.");
                    return print_result(solution_tree, solution_damage);
                }
            }
        } else {
            // Damage is still positive, try the strongest spell again (greedy)
            current_spell_idx = 0;
        }
    }

    print_result(solution_tree, solution_damage)
}

fn print_result(solution_tree: Vec<usize>, final_damage: i32) -> bool {
    if solution_tree.is_empty() {
        println!("No solution found.");
        return true;
    }

    println!("\n--- Spells ---");
    let mut last_spell_printed: i32 = -1;
    let mut spell_count: i32 = 0;
    
    for &spell_idx in &solution_tree {
        let current_spell = spell_idx as i32;
        if last_spell_printed == current_spell {
            spell_count += 1;
        } else {
            if last_spell_printed != -1 {
                print_spell(last_spell_printed as usize, spell_count);
            }
            spell_count = 1;
            last_spell_printed = current_spell;
        }
    }
    
    if last_spell_printed != -1 {
        print_spell(last_spell_printed as usize, spell_count);
    }
    
    println!("Remaining Health: {}", final_damage);
    println!("------------------------");
    true
}

fn print_spell(idx: usize, count: i32) {
    if count > 1 {
        println!("{} ×{}", SPELL_NAME[idx], count);
    } else {
        println!("{}", SPELL_NAME[idx]);
    }
}

fn main() {
    println!("Spell Calculator (Enter 0 to quit)");
    loop {
        if !cal() {
            break;
        }
    }
}
