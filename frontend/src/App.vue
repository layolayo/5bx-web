<script setup lang="ts">
import { ref, onMounted } from 'vue';
import Navbar from './components/Navbar.vue';
import LandingHero from './components/LandingHero.vue';
import TodayWorkoutView from './components/TodayWorkout.vue';
import ChartBrowser from './components/ChartBrowser.vue';
import PrintSheet from './components/PrintSheet.vue';
import HistoryList from './components/HistoryList.vue';
import BadgesModal from './components/BadgesModal.vue';
import TimerModal from './components/TimerModal.vue';
import ManualLogModal from './components/ManualLogModal.vue';
import LoginModal from './components/LoginModal.vue';
import LevelAdjustModal from './components/LevelAdjustModal.vue';
import MobileCockpit from './components/MobileCockpit.vue';
import SessionDetailModal from './components/SessionDetailModal.vue';
import {
  UserProfile,
  TodayWorkout,
  WorkoutSessionHistory,
  BadgesResponse,
  EarnedBadge,
  SubmitWorkoutPayload,
  WorkoutSubmissionResult,
} from './types';
import {
  fetchMe,
  fetchTodayWorkout,
  fetchHistory,
  fetchBadges,
  submitWorkout,
  loginUser,
  registerUser,
  logoutUser,
  adjustLevels,
} from './api';

// Application State
const profile = ref<UserProfile | null>(null);
const workout = ref<TodayWorkout | null>(null);
const history = ref<WorkoutSessionHistory[]>([]);
const badges = ref<BadgesResponse>({ earned_badges: [], targets: [] });
const highestBadge = ref<EarnedBadge | null>(null);
const activeTab = ref('workout'); // 'workout' | 'charts' | 'sheet' | 'history' | 'badges'
const isOffline = ref(!navigator.onLine);

// Modals and Flow States
const showTimer = ref(false);
const showManualLog = ref(false);
const showLogin = ref(false);
const showAdjust = ref(false);
const showBadges = ref(false);
const loginPrefill = ref('');
const isGuestSession = ref(false);
const celebrationResult = ref<WorkoutSubmissionResult | null>(null);

// Fallback guest workout for unauthenticated test-drives
const guestWorkoutTemplate: TodayWorkout = {
  user_id: 0,
  username: 'Guest Pilot',
  date: new Date().toISOString().split('T')[0],
  strength_chart: 1,
  strength_level: 1,
  strength_display: 'Chart 1 • Level 1 (D-)',
  cardio_chart: 1,
  cardio_level: 1,
  cardio_display: 'Chart 1 • Level 1 (D-)',
  exercises: [
    {
      exercise_number: 1,
      name: 'Forward Bends',
      time_limit_seconds: 120,
      target_reps: 14,
      image_path: 'c1_ex1.png',
      instructions: 'Stand erect, feet 12 inches apart. Bend forward to touch the floor, then stretch up backward.',
      is_cardio: false,
      alt_run_time_seconds: 0,
      alt_walk_time_seconds: 0,
    },
    {
      exercise_number: 2,
      name: 'Sit-Ups',
      time_limit_seconds: 60,
      target_reps: 8,
      image_path: 'c1_ex2.png',
      instructions: 'Lie on back, feet 6 inches apart. Sit up smoothly, reach past toes, then roll down.',
      is_cardio: false,
      alt_run_time_seconds: 0,
      alt_walk_time_seconds: 0,
    },
    {
      exercise_number: 3,
      name: 'Back Arches',
      time_limit_seconds: 60,
      target_reps: 12,
      image_path: 'c1_ex3.png',
      instructions: 'Lie prone, hands under thighs. Raise head, chest, and thighs high off the ground.',
      is_cardio: false,
      alt_run_time_seconds: 0,
      alt_walk_time_seconds: 0,
    },
    {
      exercise_number: 4,
      name: 'Push-Ups (Knees)',
      time_limit_seconds: 60,
      target_reps: 4,
      image_path: 'c1_ex4.png',
      instructions: 'Push-ups from the knees, keeping trunk rigid and touching chin to floor.',
      is_cardio: false,
      alt_run_time_seconds: 0,
      alt_walk_time_seconds: 0,
    },
    {
      exercise_number: 5,
      name: 'Stationary Run',
      time_limit_seconds: 360,
      target_reps: 200,
      image_path: 'c1_ex5.png',
      instructions: 'Stationary run lifting feet 4 inches off the floor. 1 step every forward foot contact.',
      is_cardio: true,
      alt_run_time_seconds: 480,
      alt_walk_time_seconds: 1260,
    },
  ],
};

async function loadData() {
  try {
    profile.value = await fetchMe();
    if (profile.value) {
      workout.value = await fetchTodayWorkout();
      history.value = await fetchHistory();
      const bRes = await fetchBadges();
      badges.value = bRes;
      highestBadge.value = bRes.highest_badge || (bRes.earned_badges.length > 0 ? bRes.earned_badges[0] : null);
    }
  } catch (e) {
    console.error('Error loading 5BX data:', e);
  }
}

async function handleLogin(payload: { username_or_email: string; password: string }) {
  try {
    profile.value = await loginUser(payload.username_or_email, payload.password);
    showLogin.value = false;
    isGuestSession.value = false;
    workout.value = await fetchTodayWorkout();
    history.value = await fetchHistory();
    const bRes = await fetchBadges();
    badges.value = bRes;
    highestBadge.value = bRes.highest_badge || (bRes.earned_badges.length > 0 ? bRes.earned_badges[0] : null);
    activeTab.value = 'workout';
  } catch (e: any) {
    alert(e.message || 'Authentication failed');
  }
}

async function handleRegister(payload: any) {
  try {
    profile.value = await registerUser(payload);
    showLogin.value = false;
    isGuestSession.value = false;
    workout.value = await fetchTodayWorkout();
    history.value = await fetchHistory();
    const bRes = await fetchBadges();
    badges.value = bRes;
    highestBadge.value = bRes.highest_badge || (bRes.earned_badges.length > 0 ? bRes.earned_badges[0] : null);
    activeTab.value = 'workout';
  } catch (e: any) {
    alert(e.message || 'Registration failed');
  }
}

async function handleLogout() {
  await logoutUser();
  profile.value = null;
  workout.value = null;
  history.value = [];
  highestBadge.value = null;
  activeTab.value = 'workout';
}

function startGuestWorkout() {
  workout.value = guestWorkoutTemplate;
  isGuestSession.value = true;
  showTimer.value = true;
}

function handleOpenLoginWithPilot(pilot?: { username: string }) {
  loginPrefill.value = pilot ? pilot.username : '';
  showLogin.value = true;
}

async function handleSubmitWorkout(payload: SubmitWorkoutPayload) {
  if (isGuestSession.value || !profile.value) {
    showTimer.value = false;
    showManualLog.value = false;
    alert('Congratulations on completing your 11-minute test session! To record your progression and unlock badges, please sign in or register.');
    showLogin.value = true;
    return;
  }

  try {
    const res = await submitWorkout(payload);
    celebrationResult.value = res;
    showTimer.value = false;
    showManualLog.value = false;

    // Refresh telemetry
    profile.value = await fetchMe();
    workout.value = await fetchTodayWorkout();
    history.value = await fetchHistory();
    const bRes = await fetchBadges();
    badges.value = bRes;
    highestBadge.value = bRes.highest_badge || (bRes.earned_badges.length > 0 ? bRes.earned_badges[0] : null);
  } catch (e: any) {
    alert(e.message || 'Failed to record workout session');
  }
}

async function handleAdjustSave(payload: { s_chart: number; s_level: number; c_chart: number; c_level: number; reason: string }) {
  try {
    profile.value = await adjustLevels(payload.s_chart, payload.s_level, payload.c_chart, payload.c_level, payload.reason);
    workout.value = await fetchTodayWorkout();
    history.value = await fetchHistory();
    const bRes = await fetchBadges();
    badges.value = bRes;
    highestBadge.value = bRes.highest_badge || (bRes.earned_badges.length > 0 ? bRes.earned_badges[0] : null);
    showAdjust.value = false;
  } catch (e: any) {
    alert(e.message || 'Failed to adjust levels');
  }
}

function navigate(tab: string) {
  if (tab === 'badges') {
    showBadges.value = true;
  } else {
    activeTab.value = tab;
  }
}

// KISS Mode State & Methods
const isKissMode = ref(false);
const inspectSession = ref<WorkoutSessionHistory | null>(null);

function toggleKissMode() {
  isKissMode.value = !isKissMode.value;
  localStorage.setItem('5bx_kiss_mode', isKissMode.value ? 'true' : 'false');
}

function handleInspectSession(session: WorkoutSessionHistory) {
  inspectSession.value = session;
}

async function handleSessionUpdated() {
  history.value = await fetchHistory();
  if (inspectSession.value) {
    const updated = history.value.find(s => s.id === inspectSession.value!.id);
    if (updated) inspectSession.value = updated;
  }
}

async function handleSessionDeleted() {
  inspectSession.value = null;
  history.value = await fetchHistory();
  profile.value = await fetchMe();
  workout.value = await fetchTodayWorkout();
}

onMounted(() => {
  window.addEventListener('online', () => (isOffline.value = false));
  window.addEventListener('offline', () => (isOffline.value = true));

  // Initialise KISS Mode (check stored preference, default to mobile viewport < 640px)
  const storedKiss = localStorage.getItem('5bx_kiss_mode');
  if (storedKiss !== null) {
    isKissMode.value = storedKiss === 'true';
  } else {
    isKissMode.value = window.innerWidth < 640;
  }

  if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('/sw.js').catch((err) => {
      console.log('SW registration skipped:', err);
    });
  }

  loadData();
});
</script>

<template>
  <div class="min-h-screen bg-[#070a12] text-slate-100 flex flex-col font-sans selection:bg-cyan-500 selection:text-slate-950">
    <!-- Offline status notification -->
    <div v-if="isOffline" class="no-print bg-amber-500 text-slate-950 text-xs font-black py-1.5 px-4 text-center uppercase tracking-wider shadow">
      Offline Mode Active — You can train and print your workout form without internet signal.
    </div>

    <!-- Navigation Header -->
    <Navbar
      :profile="profile"
      :active-tab="activeTab"
      :is-kiss-mode="isKissMode"
      @navigate="navigate"
      @open-login="handleOpenLoginWithPilot()"
      @open-adjust="showAdjust = true"
      @logout="handleLogout"
      @toggle-kiss="toggleKissMode"
    />

    <!-- Main Dynamic Content -->
    <main class="flex-1 pb-16">
      <!-- Public Motivational Landing Page (when logged out and on workout tab) -->
      <LandingHero
        v-if="!profile && activeTab === 'workout'"
        @open-login="handleOpenLoginWithPilot"
        @start-guest-workout="startGuestWorkout"
        @open-sheet="activeTab = 'sheet'"
        @open-charts="activeTab = 'charts'"
      />

      <!-- Streamlined KISS Mobile Cockpit (Active when isKissMode is true) -->
      <MobileCockpit
        v-else-if="profile && workout && activeTab === 'workout' && isKissMode"
        :workout="workout"
        :profile="profile"
        :highest-badge="highestBadge"
        @start-timer="showTimer = true; isGuestSession = false"
        @open-sheet="activeTab = 'sheet'"
        @log-manual="showManualLog = true; isGuestSession = false"
        @toggle-kiss="toggleKissMode"
      />

      <!-- Authenticated Cockpit: Today's Mission (Full Desktop Mode) -->
      <TodayWorkoutView
        v-else-if="profile && workout && activeTab === 'workout'"
        :workout="workout"
        :profile="profile"
        :highest-badge="highestBadge"
        @start-timer="showTimer = true; isGuestSession = false"
        @open-sheet="activeTab = 'sheet'"
        @log-manual="showManualLog = true; isGuestSession = false"
        @open-badges="showBadges = true"
      />

      <!-- System Charts Browser (Charts 1-6, all 72 rungs) -->
      <ChartBrowser
        v-if="activeTab === 'charts'"
        :profile="profile"
        @close="activeTab = 'workout'"
      />

      <!-- Single-Sheet Printable Form (available to all) -->
      <PrintSheet
        v-if="activeTab === 'sheet'"
        :workout="workout || guestWorkoutTemplate"
        :profile="profile"
        @close="activeTab = 'workout'"
      />

      <!-- Flight Log: History List & Telemetry Graphs -->
      <HistoryList
        v-if="profile && activeTab === 'history'"
        :history="history"
        @inspect="handleInspectSession"
      />
    </main>

    <!-- Modals -->
    <!-- Active 11-Minute Timer Guided Session -->
    <TimerModal
      v-if="showTimer && (workout || guestWorkoutTemplate)"
      :workout="workout || guestWorkoutTemplate"
      :is-guest="isGuestSession"
      @close="showTimer = false"
      @submit="handleSubmitWorkout"
    />

    <!-- Dedicated Offline Scorecard Manual Log Modal -->
    <ManualLogModal
      v-if="showManualLog && (workout || guestWorkoutTemplate)"
      :workout="workout || guestWorkoutTemplate"
      @close="showManualLog = false"
      @submit="handleSubmitWorkout"
    />

    <!-- Pilot Sign In & Profile Creation -->
    <LoginModal
      v-if="showLogin"
      :initial-username="loginPrefill"
      @close="showLogin = false"
      @login="handleLogin"
      @register="handleRegister"
    />

    <!-- Manual Starting Level Adjustment -->
    <LevelAdjustModal
      v-if="showAdjust && profile"
      :profile="profile"
      @close="showAdjust = false"
      @save="handleAdjustSave"
    />

    <!-- Milestone Wings Trophy Room -->
    <BadgesModal
      v-if="showBadges"
      :badges="badges"
      @close="showBadges = false"
    />

    <!-- Celebration Modal (On promotion / verdict) -->
    <div v-if="celebrationResult" class="fixed inset-0 z-50 bg-slate-950/85 backdrop-blur-md flex items-center justify-center p-4">
      <div class="bg-slate-900 border-2 border-emerald-500 rounded-3xl w-full max-w-md p-6 sm:p-8 text-center shadow-2xl relative overflow-hidden">
        <!-- Glow ambient accent -->
        <div class="absolute -top-24 left-1/2 -translate-x-1/2 w-64 h-32 bg-emerald-500/20 blur-3xl pointer-events-none"></div>

        <div class="w-16 h-16 rounded-2xl bg-emerald-500/20 border border-emerald-500/40 text-emerald-400 flex items-center justify-center text-4xl mx-auto mb-3">
          🎖️
        </div>
        <h2 class="text-2xl font-black text-white uppercase tracking-tight">Workout Graded!</h2>
        <p class="text-xs text-slate-300 mb-5">Official Royal Canadian Air Force 5BX Verdict</p>

        <div class="space-y-3 bg-slate-950/80 p-4 rounded-2xl border border-slate-800 text-left mb-6">
          <div>
            <span class="text-[10px] text-slate-400 font-bold uppercase tracking-wider block">Strength Track Verdict:</span>
            <div class="text-base font-black text-emerald-400">{{ celebrationResult.strength_verdict }}</div>
            <div class="text-xs text-slate-300 font-mono mt-0.5">New Position: {{ celebrationResult.new_strength }}</div>
          </div>

          <div class="border-t border-slate-800 pt-3">
            <span class="text-[10px] text-slate-400 font-bold uppercase tracking-wider block">Cardio Track Verdict:</span>
            <div class="text-base font-black text-cyan-400">{{ celebrationResult.cardio_verdict }}</div>
            <div class="text-xs text-slate-300 font-mono mt-0.5">New Position: {{ celebrationResult.new_cardio }}</div>
          </div>
        </div>

        <div v-if="celebrationResult.new_badges.length > 0" class="mb-6 bg-amber-500/15 border border-amber-500/40 p-4 rounded-2xl text-left">
          <span class="text-[10px] font-black text-amber-300 uppercase tracking-wider block mb-1">New Milestone Unlocked!</span>
          <div v-for="b in celebrationResult.new_badges" :key="b.key" class="text-sm font-bold text-white flex items-center gap-2">
            <span>🏆</span>
            <span>{{ b.title }}</span>
          </div>
        </div>

        <button
          @click="celebrationResult = null"
          class="w-full btn-control-primary bg-gradient-to-r from-emerald-500 to-teal-600 hover:from-emerald-400 hover:to-teal-500 text-slate-950 shadow-lg shadow-emerald-500/20"
        >
          Acknowledge & Continue
        </button>
      </div>
    </div>

    <!-- Flight Debrief & Record Management Modal -->
    <SessionDetailModal
      v-if="inspectSession"
      :session="inspectSession"
      :is-latest="history.length > 0 && inspectSession.id === history[0].id"
      @close="inspectSession = null"
      @updated="handleSessionUpdated"
      @deleted="handleSessionDeleted"
    />
  </div>
</template>
