#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let sum_d_of = |val: u8| dice.iter().filter(|&&d| d == val).sum::<u8>();
    let find_with_count = |counts: &[u8], cnt: u8| counts.iter().position(|&c| c == cnt).unwrap_or(0) as u8;
    let find_with_count_or_more = |counts: &[u8], cnt: u8| counts.iter().position(|&c| c >= cnt).unwrap_or(0) as u8;

    match category {
        Category::Ones => sum_d_of(1),
        Category::Twos => sum_d_of(2),
        Category::Threes => sum_d_of(3),
        Category::Fours => sum_d_of(4),
        Category::Fives => sum_d_of(5),
        Category::Sixes => sum_d_of(6),
        Category::FullHouse => {
            let mut counts = [0; 7];
            for d in dice {
                counts[d as usize] += 1;
            }
            let twos = find_with_count(&counts, 2)  * 2;
            let threes = find_with_count(&counts, 3) * 3;

            if twos != 0 && threes != 0 { twos + threes } else { 0 }
        }
        Category::FourOfAKind => {
            let mut counts = [0; 7];
            for d in dice {
                counts[d as usize] += 1;
            }
            find_with_count_or_more(&counts, 4) * 4
        }
        Category::LittleStraight => {
            let mut dice = dice.to_vec();
            dice.sort_unstable();
            if dice == [1, 2, 3, 4, 5] { 30 } else { 0 }
        }
        Category::BigStraight => {
            let mut dice = dice.to_vec();
            dice.sort_unstable();
            if dice == [2, 3, 4, 5, 6] { 30 } else { 0 }
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => if dice.iter().all(|&d| d == dice[0]) { 50 } else { 0 }

    }
}
