# Royal Canadian Air Force 5BX Web Application

> **Five Basic Exercises. Eleven Minutes a Day. No Equipment Required.**

A high-performance, responsive web application and Progressive Web App (PWA) port of the classic RCAF 5BX fitness plan, powered by a **Rust backend** (`axum` + `tokio` + `sqlx`) and a **Vue 3 frontend**.

---

## Key Features

1. **Split Ladder Progression**:
   * Independent tracking for **Strength** (Exercises 1–4) and **Cardio** (Exercise 5: Stationary run, Outdoor run, or Walk).
   * 6 Charts, 12 Levels each (72 levels total, D- through A+).
   * **The Golden Rule**: Meeting current targets earns an automatic promotion.
   * **Leapfrog Logic**: Exceeding current targets advances multiple levels within the chart.
   * **3-Strikes Demotion**: 3 consecutive sessions missing targets triggers a graceful level reduction to match ability.

2. **Single-Sheet Printable Form (`/sheet`)**:
   * Specifically formatted using CSS `@media print` to guarantee a clean layout on **strictly one single A4 or Letter page**.
   * Contains user's today targets, exercise diagrams, posture cues, and write-in boxes for pencil recording when training in gym facilities with zero internet signal.

3. **Offline Progressive Web App (PWA)**:
   * Service worker caches the application and exercise database for offline execution on smartphones and tablets.
   * Completed reps are saved locally and synchronised to the server upon reconnection.

4. **11-Minute Guided Timer**:
   * Automated cadence and interval timing (2m, 1m, 1m, 1m, 6m).
   * Native sound chimes synthesised via browser Web Audio API (zero external sound files needed).

5. **Milestones & Achievements**:
   * Age-specific maintenance targets.
   * Flying Crew Elite targets.
   * Superman milestones (surpassing younger age group standards).

---

## Low-Bandwidth Mobile Deployment

The deployment pipeline in `scripts/deploy.sh` is specifically designed to minimise mobile cellular data usage:

```bash
# Mode 1: Remote Build on Penguinplex (Recommended for Mobile Network - under 30 KB transfer)
./scripts/deploy.sh

# Mode 2: Local Compile on Workstation + Compressed Push (~5 MB transfer)
./scripts/deploy.sh --local
```

---

## Configuration & Environment

All sensitive secrets are isolated in an untracked `.env` file (strictly excluded from Git):

```bash
cp .env.example .env
```

| Variable | Description |
| :--- | :--- |
| `DATABASE_URL` | PostgreSQL connection string (`postgres://...`) |
| `JWT_SECRET` | Cryptographic secret for signing session tokens |
| `PORT` | Listening port (default `8085`) |
| `HOST` | Binding address (`0.0.0.0`) |
| `APP_ENV` | Environment (`development` or `production`) |

---

## Local Development

```bash
# 1. Build frontend
cd frontend
npm install
npm run build

# 2. Run Rust server
cd ../backend
cargo run
```
