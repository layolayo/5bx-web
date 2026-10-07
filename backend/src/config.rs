use std::env;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub host: String,
    pub port: u16,
    pub app_env: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        // Attempt to load .env file if present
        let _ = dotenvy::dotenv();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be specified in the environment or .env file".to_string())?;

        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "fivebx_development_secret_key_change_in_production!".to_string());

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8085".to_string())
            .parse::<u16>()
            .map_err(|e| format!("Invalid PORT specified: {}", e))?;

        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

        if app_env == "production" && jwt_secret.contains("development_secret") {
            return Err("JWT_SECRET must be set to a secure unique random key in production mode".to_string());
        }

        Ok(Config {
            database_url,
            jwt_secret,
            host,
            port,
            app_env,
        })
    }
}
