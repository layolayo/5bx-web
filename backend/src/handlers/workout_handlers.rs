use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

use crate::{
    auth::AuthUser,
    config::Config,
    engine::{
        calculate_age, check_earned_milestones, evaluate_cardio_progression,
        evaluate_strength_progression, get_level_display, TIME_LIMITS,
    },
    models::{
        DailyExerciseCard, ExerciseChart, ExerciseInstruction,
        SubmitWorkoutRequest, TodayWorkoutResponse, User, WorkoutSessionRow,
        WorkoutSubmissionResult,
    },
};

pub async fn get_today_workout(
    auth: AuthUser,
    State((pool, _)): State<(PgPool, Config)>,
) -> Result<Json<TodayWorkoutResponse>, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(auth.user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "User account not found."})),
            )
        })?;

    // Load Strength targets (Ex 1-4)
    let strength_targets = sqlx::query_as::<_, ExerciseChart>(
        "SELECT * FROM exercise_charts WHERE chart = $1 AND level = $2",
    )
    .bind(user.strength_chart)
    .bind(user.strength_level)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to retrieve strength targets: {}", e)})),
        )
    })?;

    // Load Cardio targets (Ex 5)
    let cardio_targets = sqlx::query_as::<_, ExerciseChart>(
        "SELECT * FROM exercise_charts WHERE chart = $1 AND level = $2",
    )
    .bind(user.cardio_chart)
    .bind(user.cardio_level)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to retrieve cardio targets: {}", e)})),
        )
    })?;

    // Load instructions for strength chart and cardio chart
    let instructions = sqlx::query_as::<_, ExerciseInstruction>(
        "SELECT * FROM exercise_instructions WHERE (chart = $1 AND exercise <= 4) OR (chart = $2 AND exercise >= 5)",
    )
    .bind(user.strength_chart)
    .bind(user.cardio_chart)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to retrieve exercise instructions: {}", e)})),
        )
    })?;

    let mut exercises = Vec::new();

    // Exercise 1 to 4 (Strength)
    for i in 1..=4 {
        let inst = instructions
            .iter()
            .find(|x| x.chart == user.strength_chart && x.exercise == i);

        let target_reps = match i {
            1 => strength_targets.ex1,
            2 => strength_targets.ex2,
            3 => strength_targets.ex3,
            4 => strength_targets.ex4,
            _ => 10,
        };

        exercises.push(DailyExerciseCard {
            exercise_number: i,
            name: inst.map(|x| x.name.clone()).unwrap_or_else(|| format!("Exercise {}", i)),
            instructions: inst.map(|x| x.instructions.clone()).unwrap_or_default(),
            image_path: inst.map(|x| x.image_path.clone()).unwrap_or_else(|| format!("c{}_ex{}.png", user.strength_chart, i)),
            target_reps,
            time_limit_seconds: TIME_LIMITS[(i - 1) as usize],
            is_cardio: false,
            alt_run_time_seconds: 0,
            alt_walk_time_seconds: 0,
        });
    }

    // Exercise 5 (Cardio)
    let inst5 = instructions
        .iter()
        .find(|x| x.chart == user.cardio_chart && x.exercise == 5);

    exercises.push(DailyExerciseCard {
        exercise_number: 5,
        name: inst5.map(|x| x.name.clone()).unwrap_or_else(|| "Cardio Run / Steps".to_string()),
        instructions: inst5.map(|x| x.instructions.clone()).unwrap_or_default(),
        image_path: inst5.map(|x| x.image_path.clone()).unwrap_or_else(|| format!("c{}_ex5.png", user.cardio_chart)),
        target_reps: cardio_targets.ex5,
        time_limit_seconds: TIME_LIMITS[4],
        is_cardio: true,
        alt_run_time_seconds: cardio_targets.ex5_run,
        alt_walk_time_seconds: cardio_targets.ex5_walk,
    });

    let today_str = Utc::now().format("%A, %d %B %Y").to_string();

    Ok(Json(TodayWorkoutResponse {
        user_id: user.id,
        username: user.username,
        date: today_str,
        strength_chart: user.strength_chart,
        strength_level: user.strength_level,
        strength_display: format!("Chart {} Level {}", user.strength_chart, get_level_display(user.strength_level)),
        cardio_chart: user.cardio_chart,
        cardio_level: user.cardio_level,
        cardio_display: format!("Chart {} Level {}", user.cardio_chart, get_level_display(user.cardio_level)),
        exercises,
    }))
}

pub async fn submit_workout(
    auth: AuthUser,
    State((pool, _)): State<(PgPool, Config)>,
    Json(payload): Json<SubmitWorkoutRequest>,
) -> Result<Json<WorkoutSubmissionResult>, (StatusCode, Json<serde_json::Value>)> {
    let mut tx = pool.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Transaction error: {}", e)})),
        )
    })?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1 FOR UPDATE")
        .bind(auth.user_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("User lookup error: {}", e)})),
            )
        })?;

    // Load all chart rows for user's strength & cardio chart
    let strength_chart_levels = sqlx::query_as::<_, ExerciseChart>(
        "SELECT * FROM exercise_charts WHERE chart = $1 ORDER BY level ASC",
    )
    .bind(user.strength_chart)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to load strength chart targets: {}", e)})),
        )
    })?;

    let cardio_chart_levels = sqlx::query_as::<_, ExerciseChart>(
        "SELECT * FROM exercise_charts WHERE chart = $1 ORDER BY level ASC",
    )
    .bind(user.cardio_chart)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to load cardio chart targets: {}", e)})),
        )
    })?;

    // Check consecutive fails in recent sessions
    let recent_sessions = sqlx::query_as::<_, WorkoutSessionRow>(
        "SELECT * FROM workout_sessions WHERE user_id = $1 ORDER BY timestamp DESC LIMIT 3",
    )
    .bind(user.id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("History lookup error: {}", e)})),
        )
    })?;

    let mut strength_fails = 0;
    for s in &recent_sessions {
        if s.verdict_strength.contains("MAINTAIN") {
            strength_fails += 1;
        } else {
            break;
        }
    }

    let mut cardio_fails = 0;
    for s in &recent_sessions {
        if s.verdict_cardio.contains("MAINTAIN") {
            cardio_fails += 1;
        } else {
            break;
        }
    }

    // Evaluate Strength
    let strength_reps = [payload.reps_1, payload.reps_2, payload.reps_3, payload.reps_4];
    let s_eval = evaluate_strength_progression(
        user.strength_chart,
        user.strength_level,
        &strength_reps,
        &strength_chart_levels,
        strength_fails,
    );

    // Evaluate Cardio
    let c_duration = payload.cardio_duration_secs.unwrap_or(0);
    let c_eval = evaluate_cardio_progression(
        user.cardio_chart,
        user.cardio_level,
        &payload.cardio_mode,
        payload.reps_5,
        c_duration,
        &cardio_chart_levels,
        cardio_fails,
    );

    let overall_status = if s_eval.status == "UP" || c_eval.status == "UP" {
        "Promoted".to_string()
    } else if s_eval.status == "DOWN" || c_eval.status == "DOWN" {
        "Demoted".to_string()
    } else {
        "Maintained".to_string()
    };

    // Update user record
    sqlx::query(
        r#"
        UPDATE users 
        SET strength_chart = $1, strength_level = $2, cardio_chart = $3, cardio_level = $4, updated_at = NOW()
        WHERE id = $5
        "#,
    )
    .bind(s_eval.new_chart)
    .bind(s_eval.new_level)
    .bind(c_eval.new_chart)
    .bind(c_eval.new_level)
    .bind(user.id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update user levels: {}", e)})),
        )
    })?;

    // Record session into workout_sessions
    let session_id = sqlx::query_scalar::<_, i32>(
        r#"
        INSERT INTO workout_sessions 
        (user_id, strength_chart, strength_level, cardio_chart, cardio_level, reps_1, reps_2, reps_3, reps_4, reps_5, cardio_mode, cardio_duration_secs, verdict_strength, verdict_cardio, overall_status, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        RETURNING id
        "#,
    )
    .bind(user.id)
    .bind(user.strength_chart)
    .bind(user.strength_level)
    .bind(user.cardio_chart)
    .bind(user.cardio_level)
    .bind(payload.reps_1)
    .bind(payload.reps_2)
    .bind(payload.reps_3)
    .bind(payload.reps_4)
    .bind(payload.reps_5)
    .bind(&payload.cardio_mode)
    .bind(c_duration)
    .bind(&s_eval.verdict)
    .bind(&c_eval.verdict)
    .bind(&overall_status)
    .bind(&payload.notes)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to record workout session: {}", e)})),
        )
    })?;

    // Check Milestones and Badges
    let existing_badges = sqlx::query_scalar::<_, String>(
        "SELECT badge_key FROM user_badges WHERE user_id = $1",
    )
    .bind(user.id)
    .fetch_all(&mut *tx)
    .await
    .unwrap_or_default();

    let age = calculate_age(user.dob);
    let newly_earned = check_earned_milestones(
        age,
        s_eval.new_chart,
        s_eval.new_level,
        c_eval.new_chart,
        c_eval.new_level,
        &existing_badges,
    );

    for badge in &newly_earned {
        let _ = sqlx::query(
            r#"
            INSERT INTO user_badges (user_id, badge_key, badge_title, badge_type, image_name)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (user_id, badge_key) DO NOTHING
            "#,
        )
        .bind(user.id)
        .bind(&badge.key)
        .bind(&badge.title)
        .bind(&badge.badge_type)
        .bind(&badge.image_name)
        .execute(&mut *tx)
        .await;
    }

    tx.commit().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Commit error: {}", e)})),
        )
    })?;

    let old_s_disp = format!("C{} {}", user.strength_chart, get_level_display(user.strength_level));
    let new_s_disp = format!("C{} {}", s_eval.new_chart, get_level_display(s_eval.new_level));
    let old_c_disp = format!("C{} {}", user.cardio_chart, get_level_display(user.cardio_level));
    let new_c_disp = format!("C{} {}", c_eval.new_chart, get_level_display(c_eval.new_level));

    Ok(Json(WorkoutSubmissionResult {
        session_id,
        old_strength: old_s_disp,
        new_strength: new_s_disp,
        strength_verdict: s_eval.verdict,
        old_cardio: old_c_disp,
        new_cardio: new_c_disp,
        cardio_verdict: c_eval.verdict,
        overall_status,
        new_badges: newly_earned,
    }))
}

#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    pub limit: Option<i64>,
}

pub async fn get_workout_history(
    auth: AuthUser,
    State((pool, _)): State<(PgPool, Config)>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<Vec<WorkoutSessionRow>>, (StatusCode, Json<serde_json::Value>)> {
    let limit = query.limit.unwrap_or(50).clamp(1, 100);

    let rows = sqlx::query_as::<_, WorkoutSessionRow>(
        "SELECT * FROM workout_sessions WHERE user_id = $1 ORDER BY timestamp DESC LIMIT $2",
    )
    .bind(auth.user_id)
    .bind(limit)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to retrieve history: {}", e)})),
        )
    })?;

    Ok(Json(rows))
}

pub async fn get_all_charts(
    State((pool, _)): State<(PgPool, Config)>,
) -> Result<Json<crate::models::SystemChartsResponse>, (StatusCode, Json<serde_json::Value>)> {
    let charts = sqlx::query_as::<_, crate::models::ExerciseChart>(
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

    let instructions = sqlx::query_as::<_, crate::models::ExerciseInstruction>(
        "SELECT * FROM exercise_instructions ORDER BY chart ASC, exercise ASC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to retrieve instructions: {}", e)})),
        )
    })?;

    Ok(Json(crate::models::SystemChartsResponse {
        charts,
        instructions,
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateSessionNotesRequest {
    pub notes: String,
}

pub async fn update_session_notes(
    auth: AuthUser,
    Path(id): Path<i32>,
    State((pool, _)): State<(PgPool, Config)>,
    Json(payload): Json<UpdateSessionNotesRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let res = sqlx::query(
        "UPDATE workout_sessions SET notes = $1 WHERE id = $2 AND user_id = $3",
    )
    .bind(payload.notes)
    .bind(id)
    .bind(auth.user_id)
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update notes: {}", e)})),
        )
    })?;

    if res.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Session record not found or unauthorised."})),
        ));
    }

    Ok(Json(json!({"status": "success", "message": "Debrief notes updated."})))
}

#[derive(Debug, Deserialize)]
pub struct DeleteSessionQuery {
    pub revert_level: Option<bool>,
}

pub async fn delete_workout_session(
    auth: AuthUser,
    Path(id): Path<i32>,
    Query(query): Query<DeleteSessionQuery>,
    State((pool, _)): State<(PgPool, Config)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut tx = pool.begin().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Transaction error: {}", e)})),
        )
    })?;

    // Find the session to delete
    let session = sqlx::query_as::<_, WorkoutSessionRow>(
        "SELECT * FROM workout_sessions WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(auth.user_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Workout session record not found."})),
        )
    })?;

    // Check if this was the latest session
    let latest_session_id = sqlx::query_scalar::<_, i32>(
        "SELECT id FROM workout_sessions WHERE user_id = $1 ORDER BY timestamp DESC, id DESC LIMIT 1",
    )
    .bind(auth.user_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Lookup error: {}", e)})),
        )
    })?;

    let is_latest = latest_session_id == Some(id);
    let revert = query.revert_level.unwrap_or(false);

    if is_latest && revert {
        // Revert user's strength & cardio chart/level to the state prior to this session
        sqlx::query(
            "UPDATE users SET strength_chart = $1, strength_level = $2, cardio_chart = $3, cardio_level = $4, updated_at = NOW() WHERE id = $5",
        )
        .bind(session.strength_chart)
        .bind(session.strength_level)
        .bind(session.cardio_chart)
        .bind(session.cardio_level)
        .bind(auth.user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to revert user levels: {}", e)})),
            )
        })?;
    }

    // Delete the session record
    sqlx::query("DELETE FROM workout_sessions WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(auth.user_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete session: {}", e)})),
            )
        })?;

    tx.commit().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Commit error: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "status": "success",
        "message": "Workout session record removed.",
        "was_latest": is_latest,
        "reverted": is_latest && revert
    })))
}

