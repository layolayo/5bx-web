// Assessment and Layoff Handlers for 5BX
// Evaluates baseline flight readiness and safe re-entry after absence

use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use serde_json::json;
use sqlx::PgPool;

use crate::{
    auth::AuthUser,
    config::Config,
    engine::{
        calculate_age, calculate_layoff_stepback, evaluate_diagnostic_placement, get_age_goal,
        get_level_display, DiagnosticPlacementResult,
    },
    models::{
        ApplyAssessmentRequest, EvaluateDiagnosticRequest, ExerciseChart, LayoffStatusResponse,
        User, UserProfileResponse, WorkoutSessionRow,
    },
};

pub async fn get_layoff_status(
    auth: AuthUser,
    State((pool, _)): State<(PgPool, Config)>,
) -> Result<Json<LayoffStatusResponse>, (StatusCode, Json<serde_json::Value>)> {
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

    // Find the user's most recent completed session
    let last_session = sqlx::query_as::<_, WorkoutSessionRow>(
        "SELECT * FROM workout_sessions WHERE user_id = $1 ORDER BY timestamp DESC LIMIT 1",
    )
    .bind(auth.user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to retrieve recent history: {}", e)})),
        )
    })?;

    let (days_inactive, last_date_str) = if let Some(session) = last_session {
        let diff = (Utc::now() - session.timestamp).num_days();
        (diff.max(0), Some(session.timestamp.format("%Y-%m-%d").to_string()))
    } else {
        let diff = (Utc::now() - user.created_at).num_days();
        (diff.max(0), None)
    };

    let eval = calculate_layoff_stepback(
        user.strength_chart,
        user.strength_level,
        user.cardio_chart,
        user.cardio_level,
        days_inactive,
    );

    Ok(Json(LayoffStatusResponse {
        is_layoff: eval.is_layoff,
        days_inactive: eval.days_inactive,
        last_workout_date: last_date_str,
        severity: eval.severity.to_string(),
        current_strength_chart: eval.current_strength_chart,
        current_strength_level: eval.current_strength_level,
        current_strength_display: eval.current_strength_display,
        current_cardio_chart: eval.current_cardio_chart,
        current_cardio_level: eval.current_cardio_level,
        current_cardio_display: eval.current_cardio_display,
        recommended_strength_chart: eval.recommended_strength_chart,
        recommended_strength_level: eval.recommended_strength_level,
        recommended_strength_display: eval.recommended_strength_display,
        recommended_cardio_chart: eval.recommended_cardio_chart,
        recommended_cardio_level: eval.recommended_cardio_level,
        recommended_cardio_display: eval.recommended_cardio_display,
        rationale: eval.rationale.to_string(),
    }))
}

pub async fn evaluate_diagnostic(
    _auth: AuthUser,
    State((pool, _)): State<(PgPool, Config)>,
    Json(payload): Json<EvaluateDiagnosticRequest>,
) -> Result<Json<DiagnosticPlacementResult>, (StatusCode, Json<serde_json::Value>)> {
    let all_charts = sqlx::query_as::<_, ExerciseChart>(
        "SELECT * FROM exercise_charts ORDER BY chart ASC, level ASC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to retrieve charts: {}", e)})),
        )
    })?;

    let strength_reps = [
        payload.reps_1.max(0),
        payload.reps_2.max(0),
        payload.reps_3.max(0),
        payload.reps_4.max(0),
    ];

    let cardio_reps = payload.reps_5.unwrap_or(0).max(0);
    let cardio_duration = payload.cardio_duration_secs.unwrap_or(0).max(0);

    let result = evaluate_diagnostic_placement(
        &strength_reps,
        &payload.cardio_mode,
        cardio_reps,
        cardio_duration,
        &all_charts,
    );

    Ok(Json(result))
}

pub async fn apply_assessment(
    auth: AuthUser,
    State((pool, _)): State<(PgPool, Config)>,
    Json(payload): Json<ApplyAssessmentRequest>,
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
            Json(json!({"error": format!("Failed to update profile: {}", e)})),
        )
    })?;

    let overall_status = match payload.assessment_type.as_str() {
        "layoff_safe_reentry" => "Layoff Safe Re-entry",
        "diagnostic_placement" => "Diagnostic Placement",
        "novice_start" => "Novice Induction",
        _ => "Calibration Adjustment",
    };

    let notes = payload.notes.unwrap_or_else(|| {
        format!(
            "Calibration applied: Strength C{} {}, Cardio C{} {}",
            s_chart,
            get_level_display(s_level),
            c_chart,
            get_level_display(c_level)
        )
    });

    let _ = sqlx::query(
        r#"
        INSERT INTO workout_sessions 
        (user_id, strength_chart, strength_level, cardio_chart, cardio_level, verdict_strength, verdict_cardio, overall_status, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(auth.user_id)
    .bind(s_chart)
    .bind(s_level)
    .bind(c_chart)
    .bind(c_level)
    .bind(format!("Calibrated to C{} {}", s_chart, get_level_display(s_level)))
    .bind(format!("Calibrated to C{} {}", c_chart, get_level_display(c_level)))
    .bind(overall_status)
    .bind(notes)
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
