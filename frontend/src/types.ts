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

export interface EarnedBadge {
  key: string;
  title: string;
  details: string;
  image_name: string;
  badge_type: string;
  status_text: string;
  score: number;
  is_highest: boolean;
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
  earned_badges: EarnedBadge[];
  highest_badge?: EarnedBadge | null;
  targets: MilestoneTarget[];
}

export interface ExerciseChartRow {
  id: number;
  chart: number;
  level: number;
  ex1: number;
  ex2: number;
  ex3: number;
  ex4: number;
  ex5: number;
  ex5_run: number;
  ex5_walk: number;
}

export interface ExerciseInstructionRow {
  id: number;
  chart: number;
  exercise: number;
  name: string;
  instructions: string;
  image_path: string;
}

export interface SystemChartsData {
  charts: ExerciseChartRow[];
  instructions: ExerciseInstructionRow[];
}

export interface LayoffStatus {
  is_layoff: boolean;
  days_inactive: number;
  last_workout_date?: string | null;
  severity: string;
  current_strength_chart: number;
  current_strength_level: number;
  current_strength_display: string;
  current_cardio_chart: number;
  current_cardio_level: number;
  current_cardio_display: string;
  recommended_strength_chart: number;
  recommended_strength_level: number;
  recommended_strength_display: string;
  recommended_cardio_chart: number;
  recommended_cardio_level: number;
  recommended_cardio_display: string;
  rationale: string;
}

export interface EvaluateDiagnosticPayload {
  candidate_chart?: number;
  reps_1: number;
  reps_2: number;
  reps_3: number;
  reps_4: number;
  cardio_mode: string;
  reps_5?: number;
  cardio_duration_secs?: number;
}

export interface DiagnosticPlacementResult {
  strength_chart: number;
  strength_level: number;
  strength_display: string;
  cardio_chart: number;
  cardio_level: number;
  cardio_display: string;
  summary: string;
}

export interface ApplyAssessmentPayload {
  strength_chart: number;
  strength_level: number;
  cardio_chart: number;
  cardio_level: number;
  assessment_type: string;
  notes?: string;
}

