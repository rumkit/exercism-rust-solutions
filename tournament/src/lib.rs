use std::collections::HashMap;
use std::cmp::Reverse;

struct TeamScore {
    name: String,
    matches: u32,
    wins: u32,
    losses: u32,
    draws: u32,
    points: u32,
}

impl TeamScore {
    fn new(name: &str) -> Self {
        TeamScore {
            name: name.to_string(),
            matches: 0,
            wins: 0,
            losses: 0,
            draws: 0,
            points: 0,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let results = match_results
        .lines()
        .map(|line| {
            let mut iter = line.split(';');
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut scores = HashMap::new();
    for r in results {
        let team1 = r.0;
        let team2 = r.1;
        let result = r.2;

        let mut team1_entry = scores.remove(team1).unwrap_or(TeamScore::new(team1));
        let mut team2_entry = scores.remove(team2).unwrap_or(TeamScore::new(team2));

        match result {
            "win" => {
                team1_entry.wins += 1;
                team2_entry.losses += 1;
                team1_entry.points += 3;
                team1_entry.matches += 1;
                team2_entry.matches += 1;
            }
            "loss" => {
                team2_entry.wins += 1;
                team1_entry.losses += 1;
                team2_entry.points += 3;
                team1_entry.matches += 1;
                team2_entry.matches += 1;
            }
            "draw" => {
                team1_entry.matches += 1;
                team2_entry.matches += 1;
                team1_entry.points += 1;
                team2_entry.points += 1;
                team1_entry.draws += 1;
                team2_entry.draws += 1;
            }
            _ => panic!("unexpected result: {}", result),
        }


        scores.insert(team1, team1_entry);
        scores.insert(team2, team2_entry);
    }

    let mut scores = scores.values().collect::<Vec<_>>();
    scores.sort_by_key(|s| (Reverse(s.points), &s.name));

    let mut output = String::from("Team                           | MP |  W |  D |  L |  P\n");
    for score in scores {
        output += &format!(
            "{:30} |{:>3} |{:>3} |{:>3} |{:>3} |{:>3}\n",
            score.name, score.matches, score.wins, score.draws, score.losses, score.points
        );
    }

    output.trim_end().to_string()
}
