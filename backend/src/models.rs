use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub dob: NaiveDate,
    pub strength_chart: i32,
    pub strength_level: i32,
    pub cardio_chart: i32,
    pub cardio_level: i32,
    pub goal_chart: i32,
    pub goal_level: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub dob: String,
    pub age: i32,
    pub strength_chart: i32,
    pub strength_level: i32,
    pub strength_level_display: String,
    pub cardio_chart: i32,
    pub cardio_level: i32,
    pub cardio_level_display: String,
    pub goal_chart: i32,
    pub goal_level: i32,
    pub goal_level_display: String,
    pub age_target_chart: i32,
    pub age_target_level: i32,
    pub age_target_display: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub dob: String, // "YYYY-MM-DD"
    pub initial_strength_chart: Option<i32>,
    pub initial_strength_level: Option<i32>,
    pub initial_cardio_chart: Option<i32>,
    pub initial_cardio_level: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub email: Option<String>,
    pub dob: Option<String>,
    pub goal_chart: Option<i32>,
    pub goal_level: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ManualLevelAdjustmentRequest {
    pub strength_chart: i32,
    pub strength_level: i32,
    pub cardio_chart: i32,
    pub cardio_level: i32,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExerciseChart {
    pub id: i32,
    pub chart: i32,
    pub level: i32,
    pub ex1: i32,
    pub ex2: i32,
    pub ex3: i32,
    pub ex4: i32,
    pub ex5: i32,
    pub ex5_run: i32,
    pub ex5_walk: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExerciseInstruction {
    pub id: i32,
    pub chart: i32,
    pub exercise: i32,
    pub name: String,
    pub instructions: String,
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyExerciseCard {
    pub exercise_number: i32,
    pub name: String,
    pub instructions: String,
    pub image_path: String,
    pub target_reps: i32,
    pub time_limit_seconds: i32,
    pub is_cardio: bool,
    pub alt_run_time_seconds: i32,
    pub alt_walk_time_seconds: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodayWorkoutResponse {
    pub user_id: i32,
    pub username: String,
    pub date: String,
    pub strength_chart: i32,
    pub strength_level: i32,
    pub strength_display: String,
    pub cardio_chart: i32,
    pub cardio_level: i32,
    pub cardio_display: String,
    pub exercises: Vec<DailyExerciseCard>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitWorkoutRequest {
    pub reps_1: i32,
    pub reps_2: i32,
    pub reps_3: i32,
    pub reps_4: i32,
    pub reps_5: i32,
    pub cardio_mode: String, // "stationary", "run", "walk"
    pub cardio_duration_secs: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewlyAwardedBadge {
    pub key: String,
    pub title: String,
    pub badge_type: String,
    pub image_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkoutSubmissionResult {
    pub session_id: i32,
    pub old_strength: String,
    pub new_strength: String,
    pub strength_verdict: String,
    pub old_cardio: String,
    pub new_cardio: String,
    pub cardio_verdict: String,
    pub overall_status: String,
    pub new_badges: Vec<NewlyAwardedBadge>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkoutSessionRow {
    pub id: i32,
    pub user_id: i32,
    pub timestamp: DateTime<Utc>,
    pub strength_chart: i32,
    pub strength_level: i32,
    pub cardio_chart: i32,
    pub cardio_level: i32,
    pub reps_1: i32,
    pub reps_2: i32,
    pub reps_3: i32,
    pub reps_4: i32,
    pub reps_5: i32,
    pub cardio_mode: String,
    pub cardio_duration_secs: i32,
    pub verdict_strength: String,
    pub verdict_cardio: String,
    pub overall_status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserBadgeRow {
    pub id: i32,
    pub user_id: i32,
    pub badge_key: String,
    pub badge_title: String,
    pub badge_type: String,
    pub image_name: Option<String>,
    pub earned_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemChartsResponse {
    pub charts: Vec<ExerciseChart>,
    pub instructions: Vec<ExerciseInstruction>,
}
