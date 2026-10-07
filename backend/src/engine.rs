// Canadian Air Force 5BX Engine & Progression Logic
// Ported faithfully from modules/five_bx_data.py and bio_5bx_app_v13.py

use chrono::{Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

pub const TIME_LIMITS: [i32; 5] = [120, 60, 60, 60, 360];

pub fn get_level_display(level: i32) -> &'static str {
    match level {
        1 => "D-",
        2 => "D",
        3 => "D+",
        4 => "C-",
        5 => "C",
        6 => "C+",
        7 => "B-",
        8 => "B",
        9 => "B+",
        10 => "A-",
        11 => "A",
        12 => "A+",
        _ => "D-",
    }
}

pub fn get_total_score(chart: i32, level: i32) -> i32 {
    (chart * 12) + level
}

pub fn calculate_age(dob: NaiveDate) -> i32 {
    let today = Utc::now().date_naive();
    let mut age = today.year() - dob.year();
    if (today.month(), today.day()) < (dob.month(), dob.day()) {
        age -= 1;
    }
    age.max(10)
}

// Age Maintenance Goals: (min_age, max_age) -> (chart, level)
pub const AGE_TARGETS: &[((i32, i32), (i32, i32))] = &[
    ((15, 15), (4, 1)),
    ((16, 17), (4, 6)),
    ((18, 25), (5, 5)),
    ((26, 29), (4, 12)),
    ((30, 34), (4, 4)),
    ((35, 39), (3, 8)),
    ((40, 44), (3, 5)),
    ((45, 49), (2, 12)),
    ((50, 60), (2, 6)),
    ((61, 99), (1, 6)),
];

// Flying Crew Elite Targets
pub const ELITE_TARGETS: &[((i32, i32), (i32, i32))] = &[
    ((18, 24), (5, 9)),
    ((25, 29), (5, 3)),
    ((30, 34), (4, 8)),
    ((35, 39), (4, 4)),
    ((40, 44), (3, 12)),
    ((45, 49), (3, 9)),
];

pub fn get_age_goal(age: i32) -> (i32, i32) {
    for &((min_a, max_a), target) in AGE_TARGETS {
        if age >= min_a && age <= max_a {
            return target;
        }
    }
    (1, 12)
}

pub fn get_next_level(current_chart: i32, current_level: i32) -> (i32, i32) {
    if current_level < 12 {
        (current_chart, current_level + 1)
    } else if current_chart < 6 {
        (current_chart + 1, 1)
    } else {
        (6, 12)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluatedProgression {
    pub new_chart: i32,
    pub new_level: i32,
    pub verdict: String,
    pub status: String, // "UP", "MAINTAIN", "DOWN"
}

// Evaluates Strength (Exercises 1-4)
// chart_targets: 12 levels for the current chart, ordered by level 1..12
pub fn evaluate_strength_progression(
    current_chart: i32,
    current_level: i32,
    achieved_reps: &[i32; 4],
    chart_levels: &[crate::models::ExerciseChart],
    consecutive_fails: i32,
) -> EvaluatedProgression {
    // Current level target
    let current_target = chart_levels
        .iter()
        .find(|c| c.chart == current_chart && c.level == current_level);

    let target_met = if let Some(target) = current_target {
        achieved_reps[0] >= target.ex1
            && achieved_reps[1] >= target.ex2
            && achieved_reps[2] >= target.ex3
            && achieved_reps[3] >= target.ex4
    } else {
        false
    };

    if target_met {
        // Find highest placement within the chart
        let mut perf_level = current_level;
        for c in chart_levels.iter().filter(|c| c.chart == current_chart) {
            if achieved_reps[0] >= c.ex1
                && achieved_reps[1] >= c.ex2
                && achieved_reps[2] >= c.ex3
                && achieved_reps[3] >= c.ex4
            {
                if c.level > perf_level {
                    perf_level = c.level;
                }
            }
        }

        // Guaranteed at least +1 promotion if target met
        let (next_c, next_l) = get_next_level(current_chart, current_level);
        let curr_score = get_total_score(current_chart, current_level);

        let (final_c, final_l) = if current_level == 12 {
            // Advancing to next chart
            (next_c, next_l)
        } else if perf_level > current_level {
            // Leapfrog within chart
            (current_chart, perf_level)
        } else {
            // Regular level up
            (next_c, next_l)
        };

        let new_score = get_total_score(final_c, final_l);
        let diff = new_score - curr_score;
        let dest = format!("C{} {}", final_c, get_level_display(final_l));

        let verdict = if diff > 1 {
            format!("LEAPFROG to {}", dest)
        } else if final_c > current_chart {
            format!("PROMOTION to {}", dest)
        } else {
            format!("LEVEL UP to {}", dest)
        };

        EvaluatedProgression {
            new_chart: final_c,
            new_level: final_l,
            verdict,
            status: "UP".to_string(),
        }
    } else {
        // Target was missed
        let fails = consecutive_fails + 1;
        if fails >= 3 {
            // Demotion: Calculate matching performance level
            let mut demoted_c = current_chart;
            let mut demoted_l = 1;

            // Find best level matching user's reps within current chart
            for c in chart_levels.iter().filter(|c| c.chart == current_chart) {
                if achieved_reps[0] >= c.ex1
                    && achieved_reps[1] >= c.ex2
                    && achieved_reps[2] >= c.ex3
                    && achieved_reps[3] >= c.ex4
                {
                    if c.level > demoted_l {
                        demoted_l = c.level;
                    }
                }
            }

            // If user is at Level 1 and failed 3 times: drop to previous chart Level 12 (A+)
            if current_level == 1 && current_chart > 1 {
                let l1_target = chart_levels.iter().find(|c| c.chart == current_chart && c.level == 1);
                let failed_l1 = l1_target.map_or(true, |t| {
                    achieved_reps[0] < t.ex1
                        || achieved_reps[1] < t.ex2
                        || achieved_reps[2] < t.ex3
                        || achieved_reps[3] < t.ex4
                });
                if failed_l1 {
                    demoted_c = current_chart - 1;
                    demoted_l = 12;
                }
            }

            let dest = format!("C{} {}", demoted_c, get_level_display(demoted_l));
            EvaluatedProgression {
                new_chart: demoted_c,
                new_level: demoted_l,
                verdict: format!("DEMOTION to {} (3 Strikes)", dest),
                status: "DOWN".to_string(),
            }
        } else {
            let loc = format!("C{} {}", current_chart, get_level_display(current_level));
            EvaluatedProgression {
                new_chart: current_chart,
                new_level: current_level,
                verdict: format!("MAINTAIN {} ({}/3 Strikes)", loc, fails),
                status: "MAINTAIN".to_string(),
            }
        }
    }
}

// Evaluates Cardio (Exercise 5: Stationary reps or Outdoor Run/Walk seconds)
pub fn evaluate_cardio_progression(
    current_chart: i32,
    current_level: i32,
    mode: &str, // "stationary", "run", "walk"
    reps: i32,
    duration_secs: i32,
    chart_levels: &[crate::models::ExerciseChart],
    consecutive_fails: i32,
) -> EvaluatedProgression {
    let current_target = chart_levels
        .iter()
        .find(|c| c.chart == current_chart && c.level == current_level);

    let is_alt_cardio = mode == "run" || mode == "walk";

    let target_met = if let Some(target) = current_target {
        if mode == "run" {
            duration_secs > 0 && duration_secs <= target.ex5_run
        } else if mode == "walk" {
            duration_secs > 0 && target.ex5_walk > 0 && duration_secs <= target.ex5_walk
        } else {
            reps >= target.ex5
        }
    } else {
        false
    };

    if target_met {
        let mut perf_level = current_level;

        for c in chart_levels.iter().filter(|c| c.chart == current_chart) {
            let meets_this = if mode == "run" {
                duration_secs > 0 && duration_secs <= c.ex5_run
            } else if mode == "walk" {
                duration_secs > 0 && c.ex5_walk > 0 && duration_secs <= c.ex5_walk
            } else {
                reps >= c.ex5
            };

            if meets_this && c.level > perf_level {
                perf_level = c.level;
            }
        }

        let (next_c, next_l) = get_next_level(current_chart, current_level);
        let curr_score = get_total_score(current_chart, current_level);

        let (final_c, final_l) = if current_level == 12 {
            (next_c, next_l)
        } else if perf_level > current_level {
            (current_chart, perf_level)
        } else {
            (next_c, next_l)
        };

        let new_score = get_total_score(final_c, final_l);
        let diff = new_score - curr_score;
        let dest = format!("C{} {}", final_c, get_level_display(final_l));

        let verdict = if diff > 1 {
            format!("LEAPFROG to {}", dest)
        } else if final_c > current_chart {
            format!("PROMOTION to {}", dest)
        } else {
            format!("LEVEL UP to {}", dest)
        };

        EvaluatedProgression {
            new_chart: final_c,
            new_level: final_l,
            verdict,
            status: "UP".to_string(),
        }
    } else {
        let fails = consecutive_fails + 1;
        if fails >= 3 {
            let mut demoted_c = current_chart;
            let mut demoted_l = 1;

            if !is_alt_cardio {
                for c in chart_levels.iter().filter(|c| c.chart == current_chart) {
                    if reps >= c.ex5 && c.level > demoted_l {
                        demoted_l = c.level;
                    }
                }
            }

            if current_level == 1 && current_chart > 1 {
                demoted_c = current_chart - 1;
                demoted_l = 12;
            }

            let dest = format!("C{} {}", demoted_c, get_level_display(demoted_l));
            EvaluatedProgression {
                new_chart: demoted_c,
                new_level: demoted_l,
                verdict: format!("DEMOTION to {} (3 Strikes)", dest),
                status: "DOWN".to_string(),
            }
        } else {
            let loc = format!("C{} {}", current_chart, get_level_display(current_level));
            EvaluatedProgression {
                new_chart: current_chart,
                new_level: current_level,
                verdict: format!("MAINTAIN {} ({}/3 Strikes)", loc, fails),
                status: "MAINTAIN".to_string(),
            }
        }
    }
}

// Award badges upon crossing milestones
pub fn check_earned_milestones(
    age: i32,
    s_new_c: i32,
    s_new_l: i32,
    c_new_c: i32,
    c_new_l: i32,
    existing_badge_keys: &[String],
) -> Vec<crate::models::NewlyAwardedBadge> {
    let mut awarded = Vec::new();
    let s_score = get_total_score(s_new_c, s_new_l);
    let c_score = get_total_score(c_new_c, c_new_l);

    // 1. User Age Maintenance Target
    let (target_c, target_l) = get_age_goal(age);
    let target_score = get_total_score(target_c, target_l);
    let age_badge_key = format!("age_target_{}", age);

    if s_score >= target_score && c_score >= target_score && !existing_badge_keys.contains(&age_badge_key) {
        awarded.push(crate::models::NewlyAwardedBadge {
            key: age_badge_key,
            title: format!("🏆 Age {} Maintenance Standard Reached", age),
            badge_type: "AgeTarget".to_string(),
            image_name: format!("{}.png", age),
        });
    }

    // 2. Flying Crew Elite Targets
    for &((min_a, max_a), (ec, el)) in ELITE_TARGETS {
        if age >= min_a && age <= max_a {
            let elite_score = get_total_score(ec, el);
            let elite_key = format!("elite_{}_{}", min_a, max_a);
            if s_score >= elite_score && c_score >= elite_score && !existing_badge_keys.contains(&elite_key) {
                awarded.push(crate::models::NewlyAwardedBadge {
                    key: elite_key,
                    title: format!("✈️ Flying Crew Elite (Age {}-{})", min_a, max_a),
                    badge_type: "Elite".to_string(),
                    image_name: format!("FC{}-{}.png", min_a, max_a),
                });
            }
        }
    }

    // 3. Superman Targets (Younger Age Standards Surpassed)
    for &((min_a, max_a), (sc, sl)) in AGE_TARGETS {
        if age > max_a {
            let younger_score = get_total_score(sc, sl);
            let superman_key = format!("superman_{}_{}", min_a, max_a);
            if s_score >= younger_score && c_score >= younger_score && !existing_badge_keys.contains(&superman_key) {
                awarded.push(crate::models::NewlyAwardedBadge {
                    key: superman_key,
                    title: format!("🦸 Superman: Achieved Age {}-{} Standards", min_a, max_a),
                    badge_type: "Superman".to_string(),
                    image_name: "SUPERMAN.png".to_string(),
                });
            }
        }
    }

    awarded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_display() {
        assert_eq!(get_level_display(1), "D-");
        assert_eq!(get_level_display(6), "C+");
        assert_eq!(get_level_display(12), "A+");
    }

    #[test]
    fn test_get_total_score() {
        assert_eq!(get_total_score(1, 1), 13);
        assert_eq!(get_total_score(1, 12), 24);
        assert_eq!(get_total_score(2, 1), 25);
    }

    #[test]
    fn test_get_next_level() {
        assert_eq!(get_next_level(1, 1), (1, 2));
        assert_eq!(get_next_level(1, 12), (2, 1));
        assert_eq!(get_next_level(6, 12), (6, 12));
    }

    #[test]
    fn test_age_targets() {
        assert_eq!(get_age_goal(52), (2, 6)); // Chart 2, C+
        assert_eq!(get_age_goal(15), (4, 1)); // Chart 4, D-
    }

    #[test]
    fn test_strength_progression_level_up() {
        use crate::models::ExerciseChart;
        let chart_levels = vec![
            ExerciseChart { id: 1, chart: 1, level: 1, ex1: 2, ex2: 3, ex3: 4, ex4: 2, ex5: 100, ex5_run: 480, ex5_walk: 1260 },
            ExerciseChart { id: 2, chart: 1, level: 2, ex1: 3, ex2: 4, ex3: 5, ex4: 3, ex5: 145, ex5_run: 450, ex5_walk: 1260 },
            ExerciseChart { id: 3, chart: 1, level: 3, ex1: 4, ex2: 5, ex3: 6, ex4: 3, ex5: 175, ex5_run: 420, ex5_walk: 1200 },
        ];

        // 1. Exact match for Level 1 targets -> Level Up to Level 2
        let res = evaluate_strength_progression(1, 1, &[2, 3, 4, 2], &chart_levels, 0);
        assert_eq!(res.status, "UP");
        assert_eq!(res.new_chart, 1);
        assert_eq!(res.new_level, 2);
        assert!(res.verdict.contains("LEVEL UP"));

        // 2. High reps meeting Level 3 targets -> LEAPFROG to Level 3
        let res_leap = evaluate_strength_progression(1, 1, &[10, 10, 10, 10], &chart_levels, 0);
        assert_eq!(res_leap.status, "UP");
        assert_eq!(res_leap.new_chart, 1);
        assert_eq!(res_leap.new_level, 3);
        assert!(res_leap.verdict.contains("LEAPFROG"));

        // 3. Failed rep on Exercise 1 (1 rep < 2) -> MAINTAIN (1/3 Strikes)
        let res_fail = evaluate_strength_progression(1, 1, &[1, 3, 4, 2], &chart_levels, 0);
        assert_eq!(res_fail.status, "MAINTAIN");
        assert_eq!(res_fail.new_level, 1);
        assert!(res_fail.verdict.contains("1/3 Strikes"));

        // 4. Failed rep 3 consecutive times -> DEMOTION (3 Strikes)
        let res_demote = evaluate_strength_progression(1, 2, &[1, 3, 4, 2], &chart_levels, 2);
        assert_eq!(res_demote.status, "DOWN");
        assert!(res_demote.verdict.contains("DEMOTION"));
    }
}
