-- Schema Initialisation for 5BX Web Application
-- PostgreSQL 18+

CREATE TABLE IF NOT EXISTS exercise_charts (
    id SERIAL PRIMARY KEY,
    chart INT NOT NULL CHECK (chart BETWEEN 1 AND 6),
    level INT NOT NULL CHECK (level BETWEEN 1 AND 12),
    ex1 INT NOT NULL,
    ex2 INT NOT NULL,
    ex3 INT NOT NULL,
    ex4 INT NOT NULL,
    ex5 INT NOT NULL,
    ex5_run INT NOT NULL DEFAULT 0,
    ex5_walk INT NOT NULL DEFAULT 0,
    CONSTRAINT uq_chart_level UNIQUE (chart, level)
);

CREATE TABLE IF NOT EXISTS exercise_instructions (
    id SERIAL PRIMARY KEY,
    chart INT NOT NULL CHECK (chart BETWEEN 1 AND 6),
    exercise INT NOT NULL,
    name VARCHAR(100) NOT NULL,
    instructions TEXT NOT NULL,
    image_path VARCHAR(100) NOT NULL,
    CONSTRAINT uq_chart_exercise UNIQUE (chart, exercise)
);

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    dob DATE NOT NULL,
    strength_chart INT NOT NULL DEFAULT 1 CHECK (strength_chart BETWEEN 1 AND 6),
    strength_level INT NOT NULL DEFAULT 1 CHECK (strength_level BETWEEN 1 AND 12),
    cardio_chart INT NOT NULL DEFAULT 1 CHECK (cardio_chart BETWEEN 1 AND 6),
    cardio_level INT NOT NULL DEFAULT 1 CHECK (cardio_level BETWEEN 1 AND 12),
    goal_chart INT NOT NULL DEFAULT 2 CHECK (goal_chart BETWEEN 1 AND 6),
    goal_level INT NOT NULL DEFAULT 6 CHECK (goal_level BETWEEN 1 AND 12),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS workout_sessions (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    strength_chart INT NOT NULL CHECK (strength_chart BETWEEN 1 AND 6),
    strength_level INT NOT NULL CHECK (strength_level BETWEEN 1 AND 12),
    cardio_chart INT NOT NULL CHECK (cardio_chart BETWEEN 1 AND 6),
    cardio_level INT NOT NULL CHECK (cardio_level BETWEEN 1 AND 12),
    reps_1 INT NOT NULL DEFAULT 0,
    reps_2 INT NOT NULL DEFAULT 0,
    reps_3 INT NOT NULL DEFAULT 0,
    reps_4 INT NOT NULL DEFAULT 0,
    reps_5 INT NOT NULL DEFAULT 0,
    cardio_mode VARCHAR(20) NOT NULL DEFAULT 'stationary',
    cardio_duration_secs INT NOT NULL DEFAULT 0,
    verdict_strength VARCHAR(150) NOT NULL,
    verdict_cardio VARCHAR(150) NOT NULL,
    overall_status VARCHAR(50) NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_badges (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    badge_key VARCHAR(100) NOT NULL,
    badge_title VARCHAR(200) NOT NULL,
    badge_type VARCHAR(50) NOT NULL,
    image_name VARCHAR(100),
    earned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_user_badge UNIQUE (user_id, badge_key)
);

CREATE INDEX IF NOT EXISTS idx_sessions_user ON workout_sessions(user_id, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_badges_user ON user_badges(user_id);
