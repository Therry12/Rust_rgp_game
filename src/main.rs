#[derive(Default, Debug)]
struct Player {
    player_name: String,
    player_attack: i32,
    player_balance: i32,
    player_hp: i32,
}

impl Player {
    fn init_player() -> Player {
        Player {
            // Setting default values.
            player_hp: 10,
            player_name: String::new(),
            player_attack: 5,
            player_balance: 0,
        }
    }
    fn attack_enemy(&self) -> i32 {
        self.player_attack
    }
}

enum E_ENEMY_TYPES {
    Goblin,
    NoType,
}
struct Enemy {
    enemy_type: E_ENEMY_TYPES,
    enemy_attack: i32,
    enemy_reward: i32,
    enemy_hp: i32,
}

impl Enemy {
    fn init_enemy() -> Enemy {
        Enemy {
            enemy_hp: 10,
            enemy_attack: 4,
            enemy_reward: 2,
            enemy_type: E_ENEMY_TYPES::Goblin,
        }
    }
}

#[derive(PartialEq)]
enum EPLAYER_ACTION {
    Attack,
    Leave,
    Ignore,
}

fn Start_Game() {
    let input: EPLAYER_ACTION;
    let mut enemy = Enemy::init_enemy();
    let mut player = Player::init_player();

    let mut enemy_defetead = false;

    player.player_name = Hello_User();
    println!("Hello, {}", player.player_name);

    while true {
        // MAIN GAME LOOP
        if !enemy_defetead {
            println!("[{}hp] 'A'ttack him or 'L'eave? ", enemy.enemy_hp);
        } else {
            println!("'A'ttack him or 'L'eave? ");
            enemy = Enemy::init_enemy();
            enemy_defetead = false;
            continue;
        }
        let input: EPLAYER_ACTION = Get_Action();
        match input {
            EPLAYER_ACTION::Leave => return,
            EPLAYER_ACTION::Attack => {
                enemy.enemy_hp -= player.attack_enemy();
                if enemy.enemy_hp <= 0 {
                    enemy_defetead = true;
                    player.player_balance += enemy.enemy_reward;
                    println!(
                        "Goblin defeated. You earn {} money.\n Now your balance: {}",
                        enemy.enemy_reward, player.player_balance
                    );
                    continue;
                }
                println!("You attack goblin, now goblin hp is {}", enemy.enemy_hp);
            }
            EPLAYER_ACTION::Ignore => todo!(), // TODO: Add ignore option
        }
    }

    println!("Goodbye, {}, good luck!", player.player_name);
}

fn Init_Enemies(enemy_type: &E_ENEMY_TYPES) {
    println!("You init goblins.");
}

fn Hello_User() -> String {
    let mut player_name = String::new();

    println!("Hello, traveler, enter your name.");
    std::io::stdin()
        .read_line(&mut player_name)
        .expect("Can't read stdin");
    player_name
}

fn Get_Action() -> EPLAYER_ACTION {
    let mut player_input = String::new();
    std::io::stdin()
        .read_line(&mut player_input)
        .expect("Can't read stdin. Player->Get_Action");

    let player_input: char = player_input
        .trim()
        .parse()
        .expect("Can't convert player_input to char in Get_Action");
    println!("BUTTON PRESSED: {}", player_input);
    match player_input {
        'A' | 'a' => {
            println!("You choose attack");
            return EPLAYER_ACTION::Attack;
        }
        'L' | 'l' => {
            println!("You choose leave");
            return EPLAYER_ACTION::Leave;
        }
        _ => {
            println!("Incorrect choose.");
            return EPLAYER_ACTION::Ignore;
        }
    }
}

fn main() {
    Start_Game();
}
