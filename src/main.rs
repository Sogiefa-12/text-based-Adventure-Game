// defining the player struct to represent the players character
struct Player {
    name: String, // the name is stored as a string
    health: i32,   // the player's health is stored as an int 32
    strength: i32,  // the player's strength is represented an an int 32
}


// creating an implementation block which provides a contructor function to initalize the variables
impl Player {
    fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            health: 100,
            strength: 50,
        }
    }
}

// creating the main function which contains the game loop
fn main() {
    let mut player = Player::new("Hero");
    let mut playing = true;

    while playing {
        //Display the Players's current status
        println!("{}: HP - {}, Strength - {}", player.name, player.health, player.strength);

        // Player encounters an enemy
        let enemy = Enemy::new("Goblin", 50, 25); // creating an enemy instance
        println!("A wild {} appeared!", enemy.name);

        // Fight or flee?
        let action = get_player_action(); // calling the get_player_action function

        if action == "fight" {
            // Calculate fight outcome
            if player_wins(&player, &enemy) {
                println!("You defeated the {}!", enemy.name);
            } else {
                println!("The {} defeated you!", enemy.name);
                player.health = 0;
            }
        } else if action == "flee" {
            println!("You escaped from the {}!", enemy.name);
        } else if action == "quit" {
            println!("You have decided to quit the game. Goodbye!");
            break;
        }else {
            println!("Invalid action!");
        }

        // Check if the player is still alive
        if player.health <= 0 {
            println!("Game Over!");
            playing = false;
        }
    }
}


//This code defines two functions:

// `get_player_action`: Prompts the player for their desired 
// action (fight or flee) and returns it as a `String`.

fn get_player_action() -> String {
    println!("Do you want to fight, flee or quit? (type 'fight', 'flee' or 'quit)");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_lowercase()

}

// `player_wins`: Takes references to `Player` and `Enemy` instances and returns `true` 
// if the player's strength is greater than or equal to the enemy's strength.
fn player_wins(player: &Player, enemy: &Enemy) -> bool {
    if player.strength >= enemy.strength {
        true
    } else {
        false
    }
}

// Define an 'Enemy' struct

struct Enemy {
    name: String,
    #[allow(dead_code)]
    health: i32,
    strength: i32,
}


impl Enemy {
    fn new(name: &str, health: i32, strength: i32) -> Enemy {
        Enemy {
            name: name.to_string(),
            health,
            strength,
        }
    }
}