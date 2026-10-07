#!/usr/bin/env python3
import sqlite3
import sys

USER_DB = "/home/matthew/Projects/5bx-python/databases/user_progress.db"
HASH = "$2b$12$0iUfLNk/o7RR9.0FXXkkpOf1QuPC3AdB8VdPdV3CEzGS5SDHjZQza" # 'FiveBX2026!'

def clean_int(val, default=1):
    if val is None: return default
    s = str(val).replace("'", "").strip()
    try: return int(s)
    except: return default

def main():
    conn = sqlite3.connect(USER_DB)
    c = conn.cursor()

    print("-- Migrating Users from SQLite user_progress.db to PostgreSQL")
    c.execute("SELECT id, name, age, dob, current_chart, current_level, goal_chart, goal_level, strength_chart, strength_level, cardio_chart, cardio_level FROM users ORDER BY id")
    users = c.fetchall()

    id_map = {} # old_id -> new_id
    for u in users:
        old_id, name, age, dob, cur_c, cur_l, goal_c, goal_l, s_c, s_l, c_c, c_l = u
        email = f"{name.lower()}@5bx.local"
        dob_val = dob if dob else "1980-01-01"
        s_chart = clean_int(s_c, clean_int(cur_c, 1))
        s_level = clean_int(s_l, clean_int(cur_l, 1))
        c_chart = clean_int(c_c, clean_int(cur_c, 1))
        c_level = clean_int(c_l, clean_int(cur_l, 1))
        g_chart = clean_int(goal_c, 2)
        g_level = clean_int(goal_l, 6)

        sql = f"""
        INSERT INTO users (username, email, password_hash, dob, strength_chart, strength_level, cardio_chart, cardio_level, goal_chart, goal_level)
        VALUES ('{name}', '{email}', '{HASH}', '{dob_val}', {s_chart}, {s_level}, {c_chart}, {c_level}, {g_chart}, {g_level})
        ON CONFLICT (username) DO UPDATE SET 
            strength_chart = EXCLUDED.strength_chart,
            strength_level = EXCLUDED.strength_level,
            cardio_chart = EXCLUDED.cardio_chart,
            cardio_level = EXCLUDED.cardio_level,
            dob = EXCLUDED.dob
        RETURNING id;
        """
        print(sql)

    print("\n-- Migrating History")
    c.execute("SELECT user_id, timestamp, chart, level, verdict, ex1, ex2, ex3, ex4, ex5, ex5_type, ex5_duration, notes FROM history ORDER BY id ASC")
    history_rows = c.fetchall()

    for h in history_rows:
        uid, ts, chart, level, verdict, ex1, ex2, ex3, ex4, ex5, ex5_type, ex5_dur, notes = h
        # Get username of uid
        c.execute("SELECT name FROM users WHERE id = ?", (uid,))
        urow = c.fetchone()
        if not urow: continue
        uname = urow[0]

        # Parse split chart/level
        s_chart, c_chart = 1, 1
        chart_str = str(chart).replace("'", "").strip()
        if "/" in chart_str:
            parts = chart_str.split("/")
            s_chart = clean_int(parts[0], 1)
            c_chart = clean_int(parts[1], 1)
        else:
            s_chart = clean_int(chart_str, 1)
            c_chart = s_chart

        s_level, c_level = 1, 1
        level_str = str(level).replace("'", "").strip()
        if "/" in level_str:
            parts = level_str.split("/")
            s_level = clean_int(parts[0], 1)
            c_level = clean_int(parts[1], 1)
        else:
            s_level = clean_int(level_str, 1)
            c_level = s_level

        v_text = str(verdict or "").replace("'", "''")
        n_text = f"'{str(notes).replace('\'', '\'\'')}'" if notes else "NULL"
        mode = "stationary"
        if ex5_type:
            if "run" in ex5_type.lower(): mode = "run"
            elif "walk" in ex5_type.lower() or "jog" in ex5_type.lower(): mode = "walk"

        dur = clean_int(ex5_dur, 0)
        e1, e2, e3, e4, e5 = clean_int(ex1, 0), clean_int(ex2, 0), clean_int(ex3, 0), clean_int(ex4, 0), clean_int(ex5, 0)

        hist_sql = f"""
        INSERT INTO workout_sessions 
        (user_id, timestamp, strength_chart, strength_level, cardio_chart, cardio_level, reps_1, reps_2, reps_3, reps_4, reps_5, cardio_mode, cardio_duration_secs, verdict_strength, verdict_cardio, overall_status, notes)
        SELECT id, '{ts}', {s_chart}, {s_level}, {c_chart}, {c_level}, {e1}, {e2}, {e3}, {e4}, {e5}, '{mode}', {dur}, '{v_text}', '{v_text}', 'Completed', {n_text}
        FROM users WHERE username = '{uname}';
        """
        print(hist_sql)

if __name__ == "__main__":
    main()
