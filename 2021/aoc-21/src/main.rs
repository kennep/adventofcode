use std::collections::HashMap;

#[derive(Hash,Eq,PartialEq,Debug,Clone)]
struct Player {
    name: String,
    score: u64,
    position: u64
}

impl Player {
    fn turn(&mut self, dice: &mut Dice) {
        let roll = dice.roll() + dice.roll() + dice.roll();
        self.turn_with_roll_mut(roll);
    }

    fn turn_with_roll_mut(&mut self, roll: u64) {
        self.position = (self.position + roll - 1) % 10  + 1;
        self.score += self.position;
    }

    fn turn_with_roll(&self, roll: u64) -> Player {
        let position = (self.position + roll - 1) % 10  + 1;
        let score = self.score + position;
        Player{name: self.name.to_string(), position, score}
    }

}

struct Dice {
    value: u64,
    rolls: u64
}

impl Dice {
    fn roll(&mut self) -> u64
    {
        self.rolls += 1;
        self.value = self.value % 100 + 1;
        return self.value
    }
}

fn get_players() -> Vec<Player> {
    let starting_positions: Vec<u64> = vec![8, 4];
    
    starting_positions.iter().enumerate().map(|(i, p)| 
        Player { name: i.to_string(), score: 0, position: *p }
    ).collect()
}

fn main() {
    let mut players = get_players();
    
    let mut dice = Dice{value: 0, rolls: 0};
    loop {
        for player in players.iter_mut() {
            player.turn(&mut dice);
            println!("Player {} moves to {}, score: {}", player.name, player.position, player.score);
            if player.score >= 1000  {
                let mut scores: Vec<u64> = players.iter().map(|p| p.score).collect();
                scores.sort();
                println!("Loosing player score: {}, dice rolled: {} times. Result: {}",
                    scores[0], dice.rolls, scores[0]*dice.rolls);
                break;                
            }
        }
        if players.iter().any(|p| p.score >= 100) {
            break;
        }
    }

    let mut players = get_players();

    let mut universes: HashMap<(Player, Player), u64> = HashMap::new();

    universes.insert((players.remove(0), players.remove(0)), 1);

    let roll_counts: Vec<(u64, u64)> = vec!(
        (3, 1),
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1)
    );

    loop {
        let mut all_won = true;
        let states: Vec<_> = universes.drain().collect();
        for (players, count) in states.into_iter() {
            if players.0.score >= 21 || players.1.score >= 21 {
                // game over
                *universes.entry(players).or_insert(0) += count;
                continue;
            }
            for (p1roll, p1count) in roll_counts.iter() {
                let new_p1state = players.0.turn_with_roll(*p1roll);
                if new_p1state.score >= 21  {
                    *universes.entry((new_p1state.clone(), players.1.clone())).or_insert(0) += count * p1count;
                }
                else {
                    for (p2roll, p2count) in roll_counts.iter() {
                        let new_p2state = players.1.turn_with_roll(*p2roll);
    
                        if new_p1state.score < 21 && new_p2state.score < 21 {
                            all_won = false;
                        }
                        *universes.entry((new_p1state.clone(), new_p2state)).or_insert(0) += count * p1count * p2count;
                    }    
                }
            }
        }
        if all_won {
            break;
        }
    }

    let mut p1wins = 0;
    let mut p2wins = 0;
    for ((p1, p2), count) in universes.drain() {
        if p1.score > p2.score {
            p1wins += count;
        }
        else if p1.score == p2.score {
            panic!("Equal score: {:?} {:?}", p1, p2);
        }
        else {
            p2wins += count;
        }
    }

    println!("Player 1 wins in {} universes.", p1wins);
    println!("Player 2 wins in {} universes.", p2wins);
}
