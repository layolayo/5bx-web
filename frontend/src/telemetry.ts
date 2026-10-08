import { WorkoutSessionHistory, ExerciseChartRow } from './types';

export interface LastPerformance {
  reps?: number;
  durationSec?: number;
  mode?: string;
  date?: string;
  display: string;
}

export interface NextRungTarget {
  chart: number;
  level: number;
  levelDisplay: string;
  targetReps?: number;
  targetSeconds?: number;
  display: string;
  isMaxStandard: boolean;
}

const LEVEL_LETTERS = ['D-', 'D', 'D+', 'C-', 'C', 'C+', 'B-', 'B', 'B+', 'A-', 'A', 'A+'];

export function getLevelDisplayName(level: number): string {
  return LEVEL_LETTERS[level - 1] || `L${level}`;
}

export function formatDurationMmSs(seconds: number): string {
  if (!seconds || seconds <= 0) return '0s';
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return s > 0 ? `${m}m ${s.toString().padStart(2, '0')}s` : `${m}m`;
}

export function calculateTreadmillSpeed(distanceMiles: number, durationSeconds: number): {
  mph: number;
  kph: number;
  display: string;
} {
  if (distanceMiles <= 0 || durationSeconds <= 0) {
    return { mph: 0, kph: 0, display: '—' };
  }
  const hours = durationSeconds / 3600.0;
  const mph = distanceMiles / hours;
  const kph = mph * 1.60934;
  return {
    mph,
    kph,
    display: `${kph.toFixed(1)} km/h (${mph.toFixed(1)} mph)`,
  };
}

export function getLastPerformance(
  history: WorkoutSessionHistory[],
  exNum: number,
  cardioMode: 'stationary' | 'run' | 'walk' = 'stationary'
): LastPerformance | null {
  if (!history || history.length === 0) return null;

  for (const session of history) {
    if (exNum === 1 && session.reps_1 > 0) {
      return { reps: session.reps_1, date: session.created_at, display: `${session.reps_1} reps` };
    }
    if (exNum === 2 && session.reps_2 > 0) {
      return { reps: session.reps_2, date: session.created_at, display: `${session.reps_2} reps` };
    }
    if (exNum === 3 && session.reps_3 > 0) {
      return { reps: session.reps_3, date: session.created_at, display: `${session.reps_3} reps` };
    }
    if (exNum === 4 && session.reps_4 > 0) {
      return { reps: session.reps_4, date: session.created_at, display: `${session.reps_4} reps` };
    }
    if (exNum === 5) {
      if (cardioMode === 'stationary') {
        if (session.cardio_mode === 'stationary' && session.reps_5 > 0) {
          return { reps: session.reps_5, mode: 'stationary', date: session.created_at, display: `${session.reps_5} steps` };
        }
        if (session.reps_5 > 0) {
          return { reps: session.reps_5, mode: 'stationary', date: session.created_at, display: `${session.reps_5} steps` };
        }
      } else if (cardioMode === 'run') {
        if (session.cardio_mode === 'run' && session.cardio_duration_secs > 0) {
          return {
            durationSec: session.cardio_duration_secs,
            mode: 'run',
            date: session.created_at,
            display: `${formatDurationMmSs(session.cardio_duration_secs)} (Run)`
          };
        }
      } else if (cardioMode === 'walk') {
        if (session.cardio_mode === 'walk' && session.cardio_duration_secs > 0) {
          return {
            durationSec: session.cardio_duration_secs,
            mode: 'walk',
            date: session.created_at,
            display: `${formatDurationMmSs(session.cardio_duration_secs)} (Walk)`
          };
        }
      }
    }
  }

  // Fallback for Exercise 5 if specific mode was not found
  if (exNum === 5) {
    for (const session of history) {
      if (session.cardio_mode !== 'stationary' && session.cardio_duration_secs > 0) {
        return {
          durationSec: session.cardio_duration_secs,
          mode: session.cardio_mode,
          date: session.created_at,
          display: `${formatDurationMmSs(session.cardio_duration_secs)} (${session.cardio_mode === 'run' ? 'Run' : 'Walk'})`
        };
      }
      if (session.reps_5 > 0) {
        return { reps: session.reps_5, mode: 'stationary', date: session.created_at, display: `${session.reps_5} steps` };
      }
    }
  }

  return null;
}

export function getNextRungTarget(
  systemCharts: ExerciseChartRow[],
  chart: number,
  level: number,
  exNum: number,
  cardioMode: 'stationary' | 'run' | 'walk' = 'stationary'
): NextRungTarget | null {
  if (!systemCharts || systemCharts.length === 0 || chart <= 0 || level <= 0) return null;

  let nextChart = chart;
  let nextLevel = level + 1;

  if (nextLevel > 12) {
    if (nextChart < 6) {
      nextChart = nextChart + 1;
      nextLevel = 1;
    } else {
      return {
        chart: 6,
        level: 12,
        levelDisplay: 'Chart 6 Level A+',
        display: 'Maximum RCAF Standard Attained',
        isMaxStandard: true,
      };
    }
  }

  const row = systemCharts.find((c) => c.chart === nextChart && c.level === nextLevel);
  if (!row) return null;

  const levelName = getLevelDisplayName(nextLevel);
  const rungBadge = `Chart ${nextChart} Level ${levelName}`;

  if (exNum === 1) {
    return { chart: nextChart, level: nextLevel, levelDisplay: rungBadge, targetReps: row.ex1, display: `${row.ex1} reps (${levelName})`, isMaxStandard: false };
  }
  if (exNum === 2) {
    return { chart: nextChart, level: nextLevel, levelDisplay: rungBadge, targetReps: row.ex2, display: `${row.ex2} reps (${levelName})`, isMaxStandard: false };
  }
  if (exNum === 3) {
    return { chart: nextChart, level: nextLevel, levelDisplay: rungBadge, targetReps: row.ex3, display: `${row.ex3} reps (${levelName})`, isMaxStandard: false };
  }
  if (exNum === 4) {
    return { chart: nextChart, level: nextLevel, levelDisplay: rungBadge, targetReps: row.ex4, display: `${row.ex4} reps (${levelName})`, isMaxStandard: false };
  }
  if (exNum === 5) {
    if (cardioMode === 'stationary') {
      return { chart: nextChart, level: nextLevel, levelDisplay: rungBadge, targetReps: row.ex5, display: `${row.ex5} steps (${levelName})`, isMaxStandard: false };
    }
    if (cardioMode === 'run') {
      return { chart: nextChart, level: nextLevel, levelDisplay: rungBadge, targetSeconds: row.ex5_run, display: `< ${formatDurationMmSs(row.ex5_run)} (${levelName})`, isMaxStandard: false };
    }
    if (cardioMode === 'walk') {
      return { chart: nextChart, level: nextLevel, levelDisplay: rungBadge, targetSeconds: row.ex5_walk, display: `< ${formatDurationMmSs(row.ex5_walk)} (${levelName})`, isMaxStandard: false };
    }
  }

  return null;
}
