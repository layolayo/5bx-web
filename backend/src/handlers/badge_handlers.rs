use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    auth::AuthUser,
    config::Config,
    engine::{
        calculate_age, get_age_goal, get_level_display, get_total_score, AGE_TARGETS, ELITE_TARGETS,
    },
    models::User,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarnedBadgeDisplay {
    pub key: String,
    pub title: String,
    pub details: String,
    pub image_name: String,
    pub badge_type: String,
    pub status_text: String,
    pub score: i32,
    pub is_highest: bool,
}

#[derive(Debug, Serialize)]
pub struct MilestoneTarget {
    pub category: String,
    pub title: String,
    pub chart: i32,
    pub level: i32,
    pub level_display: String,
    pub is_achieved: bool,
    pub image_name: String,
}

#[derive(Debug, Serialize)]
pub struct BadgesResponse {
    pub earned_badges: Vec<EarnedBadgeDisplay>,
    pub highest_badge: Option<EarnedBadgeDisplay>,
    pub targets: Vec<MilestoneTarget>,
}

fn get_standard_image_name(min_a: i32) -> String {
    match min_a {
        15 => "15.png".to_string(),
        16 => "16-17.png".to_string(),
        18 => "18-25.png".to_string(),
        26 => "26-29.png".to_string(),
        30 => "30-34.png".to_string(),
        35 => "35-39.png".to_string(),
        40 => "40-44.png".to_string(),
        45 => "45-49.png".to_string(),
        50 => "50-60.png".to_string(),
        _ => "61-99.png".to_string(),
    }
}

fn get_age_target_image(age: i32) -> String {
    for &((min_a, max_a), _) in AGE_TARGETS {
        if age >= min_a && age <= max_a {
            return get_standard_image_name(min_a);
        }
    }
    "50-60.png".to_string()
}

pub async fn get_user_badges(
    auth: AuthUser,
    State((pool, _)): State<(PgPool, Config)>,
) -> Result<Json<BadgesResponse>, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(auth.user_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    let s_score = get_total_score(user.strength_chart, user.strength_level);
    let c_score = get_total_score(user.cardio_chart, user.cardio_level);
    let age = calculate_age(user.dob);

    let mut earned = Vec::new();

    // 1. Evaluate Standard & Superman Age Badges
    for &((min_a, max_a), (t_c, t_l)) in AGE_TARGETS {
        let t_score = get_total_score(t_c, t_l);
        let s_pass = s_score >= t_score;
        let c_pass = c_score >= t_score;

        if s_pass || c_pass {
            let status_text = if s_pass && c_pass {
                "✨ FULLY ACHIEVED ✨".to_string()
            } else if s_pass {
                "💪 Strength Standard".to_string()
            } else {
                "❤️ Cardio Standard".to_string()
            };

            let age_str = if min_a == max_a {
                format!("{}", min_a)
            } else {
                format!("{}-{}", min_a, max_a)
            };

            let is_superman = age > max_a;
            let (title, badge_type, img, score) = if is_superman {
                (
                    format!("Superman (Age {} Standards)", age_str),
                    "Superman".to_string(),
                    "SUPERMAN.png".to_string(),
                    t_score + 200,
                )
            } else {
                (
                    format!("Age {} Maintenance Standard", age_str),
                    "Standard".to_string(),
                    get_standard_image_name(min_a),
                    t_score,
                )
            };

            earned.push(EarnedBadgeDisplay {
                key: format!("std_{}_{}", min_a, max_a),
                title,
                details: format!("Chart {} / Level {}", t_c, get_level_display(t_l)),
                image_name: img,
                badge_type,
                status_text,
                score,
                is_highest: false,
            });
        }
    }

    // 2. Evaluate Elite Targets
    for &((min_a, max_a), (ec, el)) in ELITE_TARGETS {
        let e_score = get_total_score(ec, el);
        let s_pass = s_score >= e_score;
        let c_pass = c_score >= e_score;

        if s_pass || c_pass {
            let status_text = if s_pass && c_pass {
                "✨ FULLY ACHIEVED ✨".to_string()
            } else if s_pass {
                "💪 Strength Standard".to_string()
            } else {
                "❤️ Cardio Standard".to_string()
            };

            let is_superman_elite = age > max_a;
            let (title, badge_type, img, score) = if is_superman_elite {
                (
                    format!("Superman Elite (Age {}-{} Standards)", min_a, max_a),
                    "Elite Superman".to_string(),
                    "ELITESUPERMAN.png".to_string(),
                    e_score + 700,
                )
            } else {
                (
                    format!("Flying Crew Elite (Age {}-{})", min_a, max_a),
                    "Elite".to_string(),
                    format!("FC{}-{}.png", min_a, max_a),
                    e_score + 500,
                )
            };

            earned.push(EarnedBadgeDisplay {
                key: format!("elite_{}_{}", min_a, max_a),
                title,
                details: format!("Chart {} / Level {}", ec, get_level_display(el)),
                image_name: img,
                badge_type,
                status_text,
                score,
                is_highest: false,
            });
        }
    }

    // Sort by score descending (highest award first)
    earned.sort_by(|a, b| b.score.cmp(&a.score));

    let mut highest_badge = None;
    if !earned.is_empty() {
        earned[0].is_highest = true;
        highest_badge = Some(earned[0].clone());
    }

    // 3. Targets Roadmap
    let mut targets = Vec::new();

    // Current Age Target
    let (ac, al) = get_age_goal(age);
    let target_score = get_total_score(ac, al);
    targets.push(MilestoneTarget {
        category: "Maintenance".to_string(),
        title: format!("Age {} Maintenance Target", age),
        chart: ac,
        level: al,
        level_display: get_level_display(al).to_string(),
        is_achieved: s_score >= target_score && c_score >= target_score,
        image_name: get_age_target_image(age),
    });

    // Flying Crew Elite Targets
    for &((min_a, max_a), (ec, el)) in ELITE_TARGETS {
        if age >= min_a && age <= max_a {
            let e_score = get_total_score(ec, el);
            targets.push(MilestoneTarget {
                category: "Elite".to_string(),
                title: format!("Flying Crew Elite (Age {}-{})", min_a, max_a),
                chart: ec,
                level: el,
                level_display: get_level_display(el).to_string(),
                is_achieved: s_score >= e_score && c_score >= e_score,
                image_name: format!("FC{}-{}.png", min_a, max_a),
            });
        }
    }

    // Superman Targets (Younger standards)
    for &((min_a, max_a), (sc, sl)) in AGE_TARGETS {
        if age > max_a {
            let sc_score = get_total_score(sc, sl);
            targets.push(MilestoneTarget {
                category: "Superman".to_string(),
                title: format!("Superman (Age {}-{} Standards)", min_a, max_a),
                chart: sc,
                level: sl,
                level_display: get_level_display(sl).to_string(),
                is_achieved: s_score >= sc_score && c_score >= sc_score,
                image_name: "SUPERMAN.png".to_string(),
            });
        }
    }

    Ok(Json(BadgesResponse {
        earned_badges: earned,
        highest_badge,
        targets,
    }))
}
