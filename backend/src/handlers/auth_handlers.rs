use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::NaiveDate;
use serde_json::json;
use sqlx::PgPool;

use crate::{
    auth::{create_jwt, hash_password, verify_password, AuthUser},
    config::Config,
    engine::{calculate_age, get_age_goal, get_level_display},
    models::{LoginRequest, RegisterRequest, User, UserProfileResponse},
};

pub async fn register(
    State((pool, config)): State<(PgPool, Config)>,
    jar: CookieJar,
    Json(payload): Json<RegisterRequest>,
) -> Result<(CookieJar, impl IntoResponse), (StatusCode, Json<serde_json::Value>)> {
    let username = payload.username.trim().to_string();
    let email = payload.email.trim().to_lowercase();

    if username.len() < 3 || username.len() > 30 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Username must be between 3 and 30 characters in length."})),
        ));
    }

    if !email.contains('@') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "A valid email address is required."})),
        ));
    }

    if payload.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Password must be at least 8 characters long."})),
        ));
    }

    let dob = NaiveDate::parse_from_str(&payload.dob, "%Y-%m-%d").map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Date of birth must be formatted as YYYY-MM-DD."})),
        )
    })?;

    let pwd_hash = hash_password(&payload.password).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e})),
        )
    })?;

    let s_chart = payload.initial_strength_chart.unwrap_or(1).clamp(1, 6);
    let s_level = payload.initial_strength_level.unwrap_or(1).clamp(1, 12);
    let c_chart = payload.initial_cardio_chart.unwrap_or(1).clamp(1, 6);
    let c_level = payload.initial_cardio_level.unwrap_or(1).clamp(1, 12);

    let age = calculate_age(dob);
    let (goal_c, goal_l) = get_age_goal(age);

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, password_hash, dob, strength_chart, strength_level, cardio_chart, cardio_level, goal_chart, goal_level)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#,
    )
    .bind(&username)
    .bind(&email)
    .bind(&pwd_hash)
    .bind(dob)
    .bind(s_chart)
    .bind(s_level)
    .bind(c_chart)
    .bind(c_level)
    .bind(goal_c)
    .bind(goal_l)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("unique") {
            (
                StatusCode::CONFLICT,
                Json(json!({"error": "A user with this username or email already exists."})),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        }
    })?;

    let token = create_jwt(user.id, &user.username, &config.jwt_secret).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e})),
        )
    })?;

    let cookie = Cookie::build(("fivebx_session", token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::days(30))
        .build();

    let age = calculate_age(user.dob);
    let (target_c, target_l) = get_age_goal(age);

    let profile = UserProfileResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        dob: user.dob.to_string(),
        age,
        strength_chart: user.strength_chart,
        strength_level: user.strength_level,
        strength_level_display: get_level_display(user.strength_level).to_string(),
        cardio_chart: user.cardio_chart,
        cardio_level: user.cardio_level,
        cardio_level_display: get_level_display(user.cardio_level).to_string(),
        goal_chart: user.goal_chart,
        goal_level: user.goal_level,
        goal_level_display: get_level_display(user.goal_level).to_string(),
        age_target_chart: target_c,
        age_target_level: target_l,
        age_target_display: get_level_display(target_l).to_string(),
    };

    Ok((jar.add(cookie), (StatusCode::CREATED, Json(profile))))
}

pub async fn login(
    State((pool, config)): State<(PgPool, Config)>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), (StatusCode, Json<serde_json::Value>)> {
    let identifier = payload.username_or_email.trim();

    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE LOWER(username) = LOWER($1) OR LOWER(email) = LOWER($1)",
    )
    .bind(identifier)
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
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid username or password."})),
        )
    })?;

    if !verify_password(&payload.password, &user.password_hash) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid username or password."})),
        ));
    }

    let token = create_jwt(user.id, &user.username, &config.jwt_secret).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e})),
        )
    })?;

    let cookie = Cookie::build(("fivebx_session", token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::days(30))
        .build();

    let age = calculate_age(user.dob);
    let (target_c, target_l) = get_age_goal(age);

    let profile = UserProfileResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        dob: user.dob.to_string(),
        age,
        strength_chart: user.strength_chart,
        strength_level: user.strength_level,
        strength_level_display: get_level_display(user.strength_level).to_string(),
        cardio_chart: user.cardio_chart,
        cardio_level: user.cardio_level,
        cardio_level_display: get_level_display(user.cardio_level).to_string(),
        goal_chart: user.goal_chart,
        goal_level: user.goal_level,
        goal_level_display: get_level_display(user.goal_level).to_string(),
        age_target_chart: target_c,
        age_target_level: target_l,
        age_target_display: get_level_display(target_l).to_string(),
    };

    Ok((jar.add(cookie), Json(profile)))
}

pub async fn logout(jar: CookieJar) -> (CookieJar, impl IntoResponse) {
    let mut cookie = Cookie::new("fivebx_session", "");
    cookie.set_path("/");
    cookie.set_max_age(time::Duration::seconds(0));
    (jar.add(cookie), Json(json!({"message": "Successfully logged out."})))
}

pub async fn me(
    auth: AuthUser,
    State((pool, _)): State<(PgPool, Config)>,
) -> Result<Json<UserProfileResponse>, (StatusCode, Json<serde_json::Value>)> {
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

    let age = calculate_age(user.dob);
    let (target_c, target_l) = get_age_goal(age);

    Ok(Json(UserProfileResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        dob: user.dob.to_string(),
        age,
        strength_chart: user.strength_chart,
        strength_level: user.strength_level,
        strength_level_display: get_level_display(user.strength_level).to_string(),
        cardio_chart: user.cardio_chart,
        cardio_level: user.cardio_level,
        cardio_level_display: get_level_display(user.cardio_level).to_string(),
        goal_chart: user.goal_chart,
        goal_level: user.goal_level,
        goal_level_display: get_level_display(user.goal_level).to_string(),
        age_target_chart: target_c,
        age_target_level: target_l,
        age_target_display: get_level_display(target_l).to_string(),
    }))
}
