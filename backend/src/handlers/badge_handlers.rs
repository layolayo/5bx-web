use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;

use crate::{
    auth::AuthUser,
    config::Config,
    engine::{calculate_age, get_age_goal, get_level_display, AGE_TARGETS, ELITE_TARGETS},
    models::{User, UserBadgeRow},
};

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
    pub earned_badges: Vec<UserBadgeRow>,
    pub targets: Vec<MilestoneTarget>,
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

    let earned = sqlx::query_as::<_, UserBadgeRow>(
        "SELECT * FROM user_badges WHERE user_id = $1 ORDER BY earned_at DESC",
    )
    .bind(user.id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let earned_keys: Vec<String> = earned.iter().map(|b| b.badge_key.clone()).collect();
    let age = calculate_age(user.dob);
    let mut targets = Vec::new();

    // 1. Current Age Target
    let (ac, al) = get_age_goal(age);
    targets.push(MilestoneTarget {
        category: "Maintenance".to_string(),
        title: format!("Age {} Maintenance Target", age),
        chart: ac,
        level: al,
        level_display: get_level_display(al).to_string(),
        is_achieved: earned_keys.contains(&format!("age_target_{}", age)),
        image_name: format!("{}.png", age),
    });

    // 2. Flying Crew Elite Targets
    for &((min_a, max_a), (ec, el)) in ELITE_TARGETS {
        if age >= min_a && age <= max_a {
            targets.push(MilestoneTarget {
                category: "Elite".to_string(),
                title: format!("Flying Crew Elite (Age {}-{})", min_a, max_a),
                chart: ec,
                level: el,
                level_display: get_level_display(el).to_string(),
                is_achieved: earned_keys.contains(&format!("elite_{}_{}", min_a, max_a)),
                image_name: format!("FC{}-{}.png", min_a, max_a),
            });
        }
    }

    // 3. Superman Targets
    for &((min_a, max_a), (sc, sl)) in AGE_TARGETS {
        if age > max_a {
            targets.push(MilestoneTarget {
                category: "Superman".to_string(),
                title: format!("Superman (Age {}-{} Standards)", min_a, max_a),
                chart: sc,
                level: sl,
                level_display: get_level_display(sl).to_string(),
                is_achieved: earned_keys.contains(&format!("superman_{}_{}", min_a, max_a)),
                image_name: "SUPERMAN.png".to_string(),
            });
        }
    }

    Ok(Json(BadgesResponse {
        earned_badges: earned,
        targets,
    }))
}
