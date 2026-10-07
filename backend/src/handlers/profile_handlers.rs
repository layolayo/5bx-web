use axum::{extract::State, http::StatusCode, Json};
use chrono::NaiveDate;
use serde_json::json;
use sqlx::PgPool;

use crate::{
    auth::AuthUser,
    config::Config,
    engine::{calculate_age, get_age_goal, get_level_display},
    models::{
        ManualLevelAdjustmentRequest, UpdateProfileRequest, User, UserProfileResponse,
    },
};

pub async fn update_profile(
    auth: AuthUser,
    State((pool, _)): State<(PgPool, Config)>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UserProfileResponse>, (StatusCode, Json<serde_json::Value>)> {
    let mut user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(auth.user_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    if let Some(ref email) = payload.email {
        let trimmed = email.trim().to_lowercase();
        if !trimmed.contains('@') {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "A valid email address is required."})),
            ));
        }
        user.email = trimmed;
    }

    if let Some(ref dob_str) = payload.dob {
        let dob = NaiveDate::parse_from_str(dob_str, "%Y-%m-%d").map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Date of birth must be YYYY-MM-DD."})),
            )
        })?;
        user.dob = dob;
    }

    if let Some(gc) = payload.goal_chart {
        user.goal_chart = gc.clamp(1, 6);
    }

    if let Some(gl) = payload.goal_level {
        user.goal_level = gl.clamp(1, 12);
    }

    let updated = sqlx::query_as::<_, User>(
        r#"
        UPDATE users 
        SET email = $1, dob = $2, goal_chart = $3, goal_level = $4, updated_at = NOW()
        WHERE id = $5
        RETURNING *
        "#,
    )
    .bind(&user.email)
    .bind(user.dob)
    .bind(user.goal_chart)
    .bind(user.goal_level)
    .bind(user.id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update profile: {}", e)})),
        )
    })?;

    let age = calculate_age(updated.dob);
    let (target_c, target_l) = get_age_goal(age);

    Ok(Json(UserProfileResponse {
        id: updated.id,
        username: updated.username,
        email: updated.email,
        dob: updated.dob.to_string(),
        age,
        strength_chart: updated.strength_chart,
        strength_level: updated.strength_level,
        strength_level_display: get_level_display(updated.strength_level).to_string(),
        cardio_chart: updated.cardio_chart,
        cardio_level: updated.cardio_level,
        cardio_level_display: get_level_display(updated.cardio_level).to_string(),
        goal_chart: updated.goal_chart,
        goal_level: updated.goal_level,
        goal_level_display: get_level_display(updated.goal_level).to_string(),
        age_target_chart: target_c,
        age_target_level: target_l,
        age_target_display: get_level_display(target_l).to_string(),
    }))
}

pub async fn manual_level_adjustment(
    auth: AuthUser,
    State((pool, _)): State<(PgPool, Config)>,
    Json(payload): Json<ManualLevelAdjustmentRequest>,
) -> Result<Json<UserProfileResponse>, (StatusCode, Json<serde_json::Value>)> {
    let s_chart = payload.strength_chart.clamp(1, 6);
    let s_level = payload.strength_level.clamp(1, 12);
    let c_chart = payload.cardio_chart.clamp(1, 6);
    let c_level = payload.cardio_level.clamp(1, 12);

    let updated = sqlx::query_as::<_, User>(
        r#"
        UPDATE users 
        SET strength_chart = $1, strength_level = $2, cardio_chart = $3, cardio_level = $4, updated_at = NOW()
        WHERE id = $5
        RETURNING *
        "#,
    )
    .bind(s_chart)
    .bind(s_level)
    .bind(c_chart)
    .bind(c_level)
    .bind(auth.user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to adjust levels: {}", e)})),
        )
    })?;

    // Record an audit entry in workout_sessions
    let reason = payload.reason.unwrap_or_else(|| "Manual level adjustment by user".to_string());
    let _ = sqlx::query(
        r#"
        INSERT INTO workout_sessions 
        (user_id, strength_chart, strength_level, cardio_chart, cardio_level, verdict_strength, verdict_cardio, overall_status, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, 'Manual Adjustment', $8)
        "#,
    )
    .bind(auth.user_id)
    .bind(s_chart)
    .bind(s_level)
    .bind(c_chart)
    .bind(c_level)
    .bind(format!("Manual set to C{} {}", s_chart, get_level_display(s_level)))
    .bind(format!("Manual set to C{} {}", c_chart, get_level_display(c_level)))
    .bind(reason)
    .execute(&pool)
    .await;

    let age = calculate_age(updated.dob);
    let (target_c, target_l) = get_age_goal(age);

    Ok(Json(UserProfileResponse {
        id: updated.id,
        username: updated.username,
        email: updated.email,
        dob: updated.dob.to_string(),
        age,
        strength_chart: updated.strength_chart,
        strength_level: updated.strength_level,
        strength_level_display: get_level_display(updated.strength_level).to_string(),
        cardio_chart: updated.cardio_chart,
        cardio_level: updated.cardio_level,
        cardio_level_display: get_level_display(updated.cardio_level).to_string(),
        goal_chart: updated.goal_chart,
        goal_level: updated.goal_level,
        goal_level_display: get_level_display(updated.goal_level).to_string(),
        age_target_chart: target_c,
        age_target_level: target_l,
        age_target_display: get_level_display(target_l).to_string(),
    }))
}
