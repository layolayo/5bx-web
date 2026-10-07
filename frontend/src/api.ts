import {
  UserProfile,
  TodayWorkout,
  SubmitWorkoutPayload,
  WorkoutSubmissionResult,
  WorkoutSessionHistory,
  BadgesResponse,
  LayoffStatus,
  EvaluateDiagnosticPayload,
  DiagnosticPlacementResult,
  ApplyAssessmentPayload,
} from './types';

const BASE_URL = '/api';

export async function fetchMe(): Promise<UserProfile | null> {
  try {
    const res = await fetch(`${BASE_URL}/auth/me`);
    if (!res.ok) return null;
    const data = await res.json();
    localStorage.setItem('5bx_user_profile', JSON.stringify(data));
    return data;
  } catch (e) {
    const cached = localStorage.getItem('5bx_user_profile');
    return cached ? JSON.parse(cached) : null;
  }
}

export async function loginUser(username_or_email: string, password: string): Promise<UserProfile> {
  const res = await fetch(`${BASE_URL}/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ username_or_email, password }),
  });
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || 'Login failed');
  }
  const data = await res.json();
  localStorage.setItem('5bx_user_profile', JSON.stringify(data));
  return data;
}

export async function registerUser(payload: any): Promise<UserProfile> {
  const res = await fetch(`${BASE_URL}/auth/register`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || 'Registration failed');
  }
  const data = await res.json();
  localStorage.setItem('5bx_user_profile', JSON.stringify(data));
  return data;
}

export async function logoutUser(): Promise<void> {
  await fetch(`${BASE_URL}/auth/logout`, { method: 'POST' });
  localStorage.removeItem('5bx_user_profile');
}

export async function fetchTodayWorkout(): Promise<TodayWorkout> {
  try {
    const res = await fetch(`${BASE_URL}/workout/today`);
    if (!res.ok) throw new Error('Failed to load workout');
    const data = await res.json();
    localStorage.setItem('5bx_today_workout', JSON.stringify(data));
    return data;
  } catch (e) {
    const cached = localStorage.getItem('5bx_today_workout');
    if (cached) return JSON.parse(cached);
    throw e;
  }
}

export async function submitWorkout(payload: SubmitWorkoutPayload): Promise<WorkoutSubmissionResult> {
  try {
    const res = await fetch(`${BASE_URL}/workout/submit`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });
    if (!res.ok) {
      const err = await res.json();
      throw new Error(err.error || 'Failed to submit workout');
    }
    return await res.json();
  } catch (e) {
    // If offline, save in queue
    const queue = JSON.parse(localStorage.getItem('5bx_offline_queue') || '[]');
    queue.push(payload);
    localStorage.setItem('5bx_offline_queue', JSON.stringify(queue));
    throw new Error('You appear to be offline. Your workout has been saved locally and will synchronise when reconnected.');
  }
}

export async function fetchHistory(): Promise<WorkoutSessionHistory[]> {
  try {
    const res = await fetch(`${BASE_URL}/workout/history?limit=30`);
    if (!res.ok) return [];
    const data = await res.json();
    localStorage.setItem('5bx_history', JSON.stringify(data));
    return data;
  } catch (e) {
    const cached = localStorage.getItem('5bx_history');
    return cached ? JSON.parse(cached) : [];
  }
}

export async function fetchBadges(): Promise<BadgesResponse> {
  try {
    const res = await fetch(`${BASE_URL}/user/badges`);
    if (!res.ok) return { earned_badges: [], targets: [] };
    const data = await res.json();
    localStorage.setItem('5bx_badges', JSON.stringify(data));
    return data;
  } catch (e) {
    const cached = localStorage.getItem('5bx_badges');
    return cached ? JSON.parse(cached) : { earned_badges: [], targets: [] };
  }
}

export async function adjustLevels(strength_chart: number, strength_level: number, cardio_chart: number, cardio_level: number, reason?: string): Promise<UserProfile> {
  const res = await fetch(`${BASE_URL}/user/levels`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ strength_chart, strength_level, cardio_chart, cardio_level, reason }),
  });
  if (!res.ok) throw new Error('Failed to update levels');
  const data = await res.json();
  localStorage.setItem('5bx_user_profile', JSON.stringify(data));
  return data;
}

export async function fetchSystemCharts(): Promise<import('./types').SystemChartsData> {
  try {
    const res = await fetch(`${BASE_URL}/charts`);
    if (!res.ok) return { charts: [], instructions: [] };
    const data = await res.json();
    localStorage.setItem('5bx_system_charts', JSON.stringify(data));
    return data;
  } catch (e) {
    const cached = localStorage.getItem('5bx_system_charts');
    return cached ? JSON.parse(cached) : { charts: [], instructions: [] };
  }
}

export async function updateSessionNotes(id: number, notes: string): Promise<void> {
  const res = await fetch(`${BASE_URL}/workout/history/${id}/notes`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ notes }),
  });
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || 'Failed to update debrief notes');
  }
}

export async function deleteSession(id: number, revertLevel: boolean): Promise<{ was_latest: boolean; reverted: boolean }> {
  const res = await fetch(`${BASE_URL}/workout/history/${id}?revert_level=${revertLevel}`, {
    method: 'DELETE',
  });
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || 'Failed to remove session');
  }
  return await res.json();
}

export async function fetchLayoffStatus(): Promise<LayoffStatus | null> {
  try {
    const res = await fetch(`${BASE_URL}/user/layoff-status`);
    if (!res.ok) return null;
    return await res.json();
  } catch (e) {
    return null;
  }
}

export async function evaluateDiagnostic(payload: EvaluateDiagnosticPayload): Promise<DiagnosticPlacementResult> {
  const res = await fetch(`${BASE_URL}/assessment/evaluate`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || 'Failed to evaluate diagnostic placement');
  }
  return await res.json();
}

export async function applyAssessment(payload: ApplyAssessmentPayload): Promise<UserProfile> {
  const res = await fetch(`${BASE_URL}/assessment/apply`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  });
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || 'Failed to apply assessment calibration');
  }
  const data = await res.json();
  localStorage.setItem('5bx_user_profile', JSON.stringify(data));
  return data;
}

export async function changePassword(current_password: string, new_password: string): Promise<void> {
  const res = await fetch(`${BASE_URL}/auth/change-password`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ current_password, new_password }),
  });
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || 'Failed to update password');
  }
}

export async function deleteAccount(password: string): Promise<void> {
  const res = await fetch(`${BASE_URL}/auth/account`, {
    method: 'DELETE',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ password }),
  });
  if (!res.ok) {
    const err = await res.json();
    throw new Error(err.error || 'Failed to delete account');
  }
  localStorage.removeItem('5bx_user_profile');
  localStorage.removeItem('5bx_today_workout');
  localStorage.removeItem('5bx_history');
  localStorage.removeItem('5bx_badges');
}


