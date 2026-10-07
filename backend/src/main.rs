use axum::{
    routing::{get, post, put},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

mod auth;
mod config;
mod db;
mod engine;
mod handlers;
mod models;

use config::Config;
use handlers::{
    auth_handlers::{login, logout, me, register},
    badge_handlers::get_user_badges,
    profile_handlers::{manual_level_adjustment, update_profile},
    static_handlers::static_handler,
    workout_handlers::{get_all_charts, get_today_workout, get_workout_history, submit_workout},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise structured tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,fivebx_server=debug".into()),
        )
        .init();

    info!("Initialising 5BX Fitness Plan Server...");

    let config = Config::from_env().map_err(|e| {
        eprintln!("Configuration Error: {}", e);
        e
    })?;

    info!("Connecting to PostgreSQL database...");
    let pool = db::create_pool(&config.database_url).await.map_err(|e| {
        eprintln!("Failed to connect to database: {}", e);
        e
    })?;

    info!("Database connection established successfully.");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let state = (pool.clone(), config.clone());

    let api_routes = Router::new()
        .route("/health", get(health_check))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(me))
        .route("/workout/today", get(get_today_workout))
        .route("/workout/submit", post(submit_workout))
        .route("/workout/history", get(get_workout_history))
        .route("/charts", get(get_all_charts))
        .route("/user/profile", put(update_profile))
        .route("/user/levels", post(manual_level_adjustment))
        .route("/user/badges", get(get_user_badges));

    let app = Router::new()
        .nest("/api", api_routes)
        .fallback(static_handler)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    info!("5BX Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "healthy",
        "service": "5bx-web",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
