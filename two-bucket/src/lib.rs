use std::cmp::min;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

impl Bucket {
    fn negate(&self) -> Self {
        match self {
            Bucket::One => Bucket::Two,
            Bucket::Two => Bucket::One,
        }
    }
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[rustfmt::skip]
/// Solve the bucket problem
pub fn solve( capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {
    let (one, two) = match start_bucket {
        Bucket::One => (
            SBucket { capacity: capacity_1, current_volume: capacity_1 },
            SBucket { capacity: capacity_2, current_volume: 0, }),
        Bucket::Two => (
            SBucket { capacity: capacity_2, current_volume: capacity_2 },
            SBucket { capacity: capacity_1, current_volume: 0 }),
    };

    let mut attempts = HashSet::new();
    match dfs( one, two, &goal,BucketStats { moves: 1, goal_bucket: Bucket::One, other_bucket: 0 }, &mut attempts) {
        None => None,
        Some(stats) => match start_bucket {
            Bucket::One => Some(stats),
            Bucket::Two => Some(BucketStats { goal_bucket: stats.goal_bucket.negate(), ..stats}),
        },
    }
}

#[derive(Copy, Clone)]
struct SBucket {
    capacity: u8,
    current_volume: u8,
}

#[rustfmt::skip]
impl SBucket {
    fn pour_into(&self, other: &SBucket) -> (SBucket, SBucket) {
        let amount = min(self.current_volume, other.free_volume());
        (
            SBucket { current_volume: self.current_volume - amount, ..*self },
            SBucket { current_volume: other.current_volume + amount,..*other },
        )
    }

    fn free_volume(&self) -> u8 { self.capacity - self.current_volume }
    fn is_empty(&self) -> bool { self.current_volume == 0 }
    fn is_full(&self) -> bool { self.current_volume == self.capacity }
    fn empty(&self) -> Self { SBucket { current_volume: 0, ..*self }}
    fn fill(&self) -> Self { SBucket { current_volume: self.capacity, ..*self }}
}

fn dfs(one: SBucket, two: SBucket, goal: &u8, mut stats: BucketStats, attempts: &mut HashSet<(u8, u8)>) -> Option<BucketStats> {
    if one.current_volume == *goal {
        return Some(BucketStats { goal_bucket: Bucket::One, other_bucket: two.current_volume, ..stats });
    }
    if two.current_volume == *goal {
        return Some(BucketStats { goal_bucket: Bucket::Two, other_bucket: one.current_volume, ..stats });
    }
    if stats.moves == u8::MAX {
        return None;
    }
    if !attempts.insert((one.current_volume, two.current_volume)){
        return None;
    }
    if one.is_empty() && two.is_full() {
        return None;
    }

    stats.moves += 1;
    let mut min_solution: Option<BucketStats> = None;

    let mut check_solution = |new_stats: Option<BucketStats>| -> () {
        if let Some(new_stats) = new_stats
            && min_solution
                .as_ref()
                .is_none_or(|min| new_stats.moves < min.moves)
        {
            min_solution = Some(new_stats);
        }
    };

    // pour one into two
    if !one.is_empty() && !two.is_full() {
        let (one, two) = one.pour_into(&two);
        check_solution(dfs(one, two, goal, stats, attempts))
    }

    // pour two into one
    if !one.is_full() && !two.is_empty() {
        let (two, one) = two.pour_into(&one);
        check_solution(dfs(one, two, goal, stats, attempts))
    }

    // empty one
    if !one.is_empty() {
        check_solution(dfs(one.empty(), two, goal, stats, attempts));
    }

    // empty two
    if !two.is_empty() {
        check_solution(dfs(one, two.empty(), goal, stats, attempts));
    }

    // fill one
    if !one.is_full() {
        check_solution(dfs(one.fill(), two, goal, stats, attempts));
    }

    // fill two
    if !two.is_full() {
        check_solution(dfs(one, two.fill(), goal, stats, attempts));
    }

    min_solution
}
