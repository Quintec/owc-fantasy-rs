use std::collections::HashMap;

use crate::util::match_costs::{MapStats, TeamResult, UserMatchCostEntry};

/// Per-player scoring breakdown returned by the calculator.
#[derive(Debug, Clone)]
pub struct PlayerScoreSummary {
    pub user_id: u32,
    pub points: i32,
}

/// Calculate scores for TeamVS matches according to the rules:
/// - 3 points for playing 30-65% of maps
/// - 5 points for playing >65% of maps
/// - 1 point for each map a player gets the highest score on
/// - 1 point for the second highest match cost on your team
/// - 3 points for the highest match cost on your team (but not the highest in the match)
/// - 5 points for the highest match cost in the match
/// - 2 points if your team won
/// - 1 point if the player's score on a given map is higher than the average score of that map
///
/// This function returns a Vec of PlayerScoreSummary (user_id -> points).
pub fn calculate_teamvs_scores(
    blue: &TeamResult,
    red: &TeamResult,
    maps: &[MapStats],
) -> Vec<PlayerScoreSummary> {
    let mut points: HashMap<u32, i32> = HashMap::new();
    let mut match_costs: HashMap<u32, f32> = HashMap::new();

    for p in blue.players.iter().chain(red.players.iter()) {
        points.insert(p.user_id, 0);
        match_costs.insert(p.user_id, p.match_cost);
    }

    // Global highest match cost
    let global_highest = match_costs.values().cloned().fold(f32::MIN, f32::max);

    // Per-team highest and second-highest
    let team_vals = |players: &Vec<UserMatchCostEntry>| {
        let mut vals: Vec<f32> = players.iter().map(|p| p.match_cost).collect();
        vals.sort_by(|a, b| b.total_cmp(a));
        (vals.get(0).copied(), vals.get(1).copied())
    };

    let (blue_high, blue_second) = team_vals(&blue.players);
    let (red_high, red_second) = team_vals(&red.players);

    let winning_is_blue = if blue.win_count > red.win_count {
        Some(true)
    } else if red.win_count > blue.win_count {
        Some(false)
    } else {
        None
    };

    // Per-map calculations: highest score in match, and average per map
    let maps_total = maps.len();
    for map in maps.iter() {
        if map.scores.is_empty() {
            continue;
        }

        let sum: u64 = map.scores.iter().map(|s| s.score as u64).sum();
        let avg = sum as f32 / map.scores.len() as f32;

        let max_score = map.scores.iter().map(|s| s.score).max().unwrap_or(0);

        for s in map.scores.iter() {
            // 1 point for highest score on this map (ties allowed)
            if s.score == max_score {
                *points.entry(s.user_id).or_default() += 1;
            }

            // 1 point if player's score on this map is higher than the average
            if (s.score as f32) > avg {
                *points.entry(s.user_id).or_default() += 1;
            }
        }
    }

    // Participation bonus based on maps played
    let mut maps_played: HashMap<u32, usize> = HashMap::new();
    for map in maps.iter() {
        for s in map.scores.iter() {
            *maps_played.entry(s.user_id).or_default() += 1;
        }
    }

    for (&user_id, &played) in maps_played.iter() {
        if maps_total == 0 {
            continue;
        }
        let pct = played as f32 / maps_total as f32;
        if pct > 0.65 {
            *points.entry(user_id).or_default() += 5;
        } else if pct >= 0.30 {
            *points.entry(user_id).or_default() += 3;
        }
    }

    // Match-cost based bonuses per team
    for p in blue.players.iter().chain(red.players.iter()) {
        let user_id = p.user_id;
        let cost = p.match_cost;

        // 5 points for highest match cost in the match
        if (cost - global_highest).abs() < 1e-6 {
            *points.entry(user_id).or_default() += 5;
            continue;
        }

        // Determine which team this player is on and award team-based points
        let (team_high, team_second) = if blue.players.iter().any(|x| x.user_id == user_id) {
            (blue_high, blue_second)
        } else {
            (red_high, red_second)
        };

        if let Some(th) = team_high {
            if (cost - th).abs() < 1e-6 {
                // highest on your team but not highest in match
                *points.entry(user_id).or_default() += 3;
                continue;
            }
        }

        if let Some(ts) = team_second {
            if (cost - ts).abs() < 1e-6 {
                *points.entry(user_id).or_default() += 1;
            }
        }
    }

    // Team win bonus: +2 points for players on winning team
    if let Some(is_blue) = winning_is_blue {
        let winners: Vec<u32> = if is_blue {
            blue.players.iter().map(|p| p.user_id).collect()
        } else {
            red.players.iter().map(|p| p.user_id).collect()
        };

        for uid in winners {
            *points.entry(uid).or_default() += 2;
        }
    }

    // Convert to Vec<PlayerScoreSummary>
    points
        .into_iter()
        .map(|(user_id, pts)| PlayerScoreSummary { user_id, points: pts })
        .collect()
}
