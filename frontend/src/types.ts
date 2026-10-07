export interface UserProfile {
  id: number;
  username: string;
  email: string;
  dob: string;
  age: number;
  strength_chart: number;
  strength_level: number;
  strength_level_display: string;
  cardio_chart: number;
  cardio_level: number;
  cardio_level_display: string;
  goal_chart: number;
  goal_level: number;
  goal_level_display: string;
  age_target_chart: number;
  age_target_level: number;
  age_target_display: string;
}

export interface DailyExerciseCard {
  exercise_number: number;
  name: string;
  instructions: string;
  image_path: string;
  target_reps: number;
  time_limit_seconds: number;
  is_cardio: boolean;
  alt_run_time_seconds: number;
  alt_walk_time_seconds: number;
}

export interface TodayWorkout {
  user_id: number;
  username: string;
  date: string;
  strength_chart: number;
  strength_level: number;
  strength_display: string;
  cardio_chart: number;
  cardio_level: number;
  cardio_display: string;
  exercises: DailyExerciseCard[];
}

export interface SubmitWorkoutPayload {
  reps_1: number;
  reps_2: number;
  reps_3: number;
  reps_4: number;
  reps_5: number;
  cardio_mode: string;
  cardio_duration_secs?: number;
  notes?: string;
}

export interface NewlyAwardedBadge {
  key: string;
  title: string;
  badge_type: string;
  image_name: string;
}

export interface WorkoutSubmissionResult {
  session_id: number;
  old_strength: string;
  new_strength: string;
  strength_verdict: string;
  old_cardio: string;
  new_cardio: string;
  cardio_verdict: string;
  overall_status: string;
  new_badges: NewlyAwardedBadge[];
}

export interface WorkoutSessionHistory {
  id: number;
  user_id: number;
  timestamp: string;
  strength_chart: number;
  strength_level: number;
  cardio_chart: number;
  cardio_level: number;
  reps_1: number;
  reps_2: number;
  reps_3: number;
  reps_4: number;
  reps_5: number;
  cardio_mode: string;
  cardio_duration_secs: number;
  verdict_strength: string;
  verdict_cardio: string;
  overall_status: string;
  notes?: string;
  created_at: string;
}

export interface UserBadge {
  id: number;
  user_id: number;
  badge_key: string;
  badge_title: string;
  badge_type: string;
  image_name?: string;
  earned_at: string;
}

export interface MilestoneTarget {
  category: string;
  title: string;
  chart: number;
  level: number;
  level_display: string;
  is_achieved: boolean;
  image_name: string;
}

export interface BadgesResponse {
  earned_badges: UserBadge[];
  targets: MilestoneTarget[];
}
