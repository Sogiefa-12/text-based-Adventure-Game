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


// Define an 'Enemy' struct
// The Enemy struct represents an enemy in the game with a name, health, and strength.
// The `Enemy` struct has a constructor function `new` that initializes the enemy's name
// health, and strength. The `health` field is marked with `#[allow(dead_code)]` to avoid warnings
// since it is not used in the current implementation, but it can be useful for future enhancements, 
// such as tracking enemy health during battles.  


struct Enemy {
    name: String,
    #[allow(dead_code)]
    health: i32,
    strength: i32,
}

// Implementing the Enemy struct with a constructor function
impl Enemy {
    fn new(name: &str, health: i32, strength: i32) -> Enemy {
        Enemy {
            name: name.to_string(),
            health,
            strength,
        }
    }
}

// This is the main entry point of the Rust Adventure Game
// creating the main function which contains the game loop
fn main() {
    // Initialize a player instance with a name
    // and set the game to be in a playing state
    println!("Welcome to the Rust Adventure Game!");
    println!("You are a brave adventurer on a quest to defeat the evil goblin king!");
    println!("Your journey begins now...");
    // The player starts with 100 health and 50 strength
    // The player will encounter enemies and can choose to fight or flee
    // The game will continue until the player decides to quit or is defeated
    // The player can choose to fight or flee from enemies
    // The player will be prompted for their action
    // The game will display the player's current status and the outcome of their actions
    // The player can quit the game at any time
    // The player will be able to see their health and strength at all times
    // The player will be able to see the enemies they encounter
    // The player will be able to see the outcome of their actions
    // The player will be able to see the game over message when they are defeated
    // The player will be able to see the game over message when they quit the game
    // The player will be able to see the game over message when they win the game

    // Create a new player instance
    // and set the game to be in a playing state
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
    println!("Do you want to fight, flee or quit? (type 'fight', 'flee' or 'quit')");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_lowercase()

}

// `player_wins function`: Takes references to `Player` and `Enemy` instances and returns `true` 
// if the player's strength is greater than or equal to the enemy's strength.
fn player_wins(player: &Player, enemy: &Enemy) -> bool {
    if player.strength >= enemy.strength {
        true
    } else {
        false
    }
}
