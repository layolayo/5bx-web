<script setup lang="ts">
import { ref, onMounted } from 'vue';
import Navbar from './components/Navbar.vue';
import TodayWorkoutView from './components/TodayWorkout.vue';
import PrintSheet from './components/PrintSheet.vue';
import HistoryList from './components/HistoryList.vue';
import BadgesModal from './components/BadgesModal.vue';
import TimerModal from './components/TimerModal.vue';
import LoginModal from './components/LoginModal.vue';
import LevelAdjustModal from './components/LevelAdjustModal.vue';
import {
  UserProfile,
  TodayWorkout,
  WorkoutSessionHistory,
  BadgesResponse,
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
const activeTab = ref('workout'); // 'workout' | 'sheet' | 'history' | 'badges'
const isOffline = ref(!navigator.onLine);

// Modals
const showTimer = ref(false);
const showLogin = ref(false);
const showAdjust = ref(false);
const showBadges = ref(false);
const celebrationResult = ref<WorkoutSubmissionResult | null>(null);

async function loadData() {
  try {
    profile.value = await fetchMe();
    if (profile.value) {
      workout.value = await fetchTodayWorkout();
      history.value = await fetchHistory();
      badges.value = await fetchBadges();
    } else {
      showLogin.value = true;
    }
  } catch (e) {
    console.error('Error loading 5BX data:', e);
  }
}

async function handleLogin(payload: { username_or_email: string; password: string }) {
  try {
    profile.value = await loginUser(payload.username_or_email, payload.password);
    showLogin.value = false;
    workout.value = await fetchTodayWorkout();
    history.value = await fetchHistory();
    badges.value = await fetchBadges();
  } catch (e: any) {
    alert(e.message || 'Login failed');
  }
}

async function handleRegister(payload: any) {
  try {
    profile.value = await registerUser(payload);
    showLogin.value = false;
    workout.value = await fetchTodayWorkout();
    history.value = await fetchHistory();
    badges.value = await fetchBadges();
  } catch (e: any) {
    alert(e.message || 'Registration failed');
  }
}

async function handleLogout() {
  await logoutUser();
  profile.value = null;
  workout.value = null;
  history.value = [];
  showLogin.value = true;
}

async function handleSubmitWorkout(payload: SubmitWorkoutPayload) {
  try {
    const res = await submitWorkout(payload);
    celebrationResult.value = res;
    showTimer.value = false;

    // Refresh data
    profile.value = await fetchMe();
    workout.value = await fetchTodayWorkout();
    history.value = await fetchHistory();
    badges.value = await fetchBadges();
  } catch (e: any) {
    alert(e.message || 'Failed to submit workout');
  }
}

async function handleAdjustSave(payload: { s_chart: number; s_level: number; c_chart: number; c_level: number; reason: string }) {
  try {
    profile.value = await adjustLevels(payload.s_chart, payload.s_level, payload.c_chart, payload.c_level, payload.reason);
    workout.value = await fetchTodayWorkout();
    history.value = await fetchHistory();
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

onMounted(() => {
  window.addEventListener('online', () => (isOffline.value = false));
  window.addEventListener('offline', () => (isOffline.value = true));

  // Register service worker if available
  if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('/sw.js').catch((err) => {
      console.log('SW registration skipped:', err);
    });
  }

  loadData();
});
</script>

<template>
  <div class="min-h-screen bg-slate-900 text-slate-100 flex flex-col">
    <!-- Offline status badge -->
    <div v-if="isOffline" class="no-print bg-amber-600 text-white text-xs font-bold py-1 px-4 text-center">
      Offline Mode Active — You can train and print your workout form without internet signal.
    </div>

    <!-- Navigation Header -->
    <Navbar
      :profile="profile"
      :active-tab="activeTab"
      @navigate="navigate"
      @open-login="showLogin = true"
      @open-adjust="showAdjust = true"
      @logout="handleLogout"
    />

    <!-- Main Content Area -->
    <main class="flex-1">
      <div v-if="!profile && !showLogin" class="p-12 text-center text-slate-400">
        Please sign in to view your personalized daily training schedule.
      </div>

      <!-- Tab: Today's Workout -->
      <TodayWorkoutView
        v-if="profile && workout && activeTab === 'workout'"
        :workout="workout"
        :profile="profile"
        @start-timer="showTimer = true"
        @open-sheet="activeTab = 'sheet'"
        @log-manual="showTimer = true"
      />

      <!-- Tab: Single-Sheet Printable Form -->
      <PrintSheet
        v-if="profile && workout && activeTab === 'sheet'"
        :workout="workout"
        :profile="profile"
        @close="activeTab = 'workout'"
      />

      <!-- Tab: Workout History -->
      <HistoryList
        v-if="profile && activeTab === 'history'"
        :history="history"
      />
    </main>

    <!-- Modals -->
    <TimerModal
      v-if="showTimer && workout"
      :workout="workout"
      @close="showTimer = false"
      @submit="handleSubmitWorkout"
    />

    <LoginModal
      v-if="showLogin"
      @close="showLogin = false"
      @login="handleLogin"
      @register="handleRegister"
    />

    <LevelAdjustModal
      v-if="showAdjust && profile"
      :profile="profile"
      @close="showAdjust = false"
      @save="handleAdjustSave"
    />

    <BadgesModal
      v-if="showBadges"
      :badges="badges"
      @close="showBadges = false"
    />

    <!-- Celebration Modal (On promotion / verdict) -->
    <div v-if="celebrationResult" class="fixed inset-0 z-50 bg-black/85 backdrop-blur-md flex items-center justify-center p-4">
      <div class="bg-slate-800 border-2 border-emerald-500 rounded-3xl w-full max-w-md p-6 text-center shadow-2xl animate-bounce-once">
        <span class="text-5xl">🎖️</span>
        <h2 class="text-2xl font-black text-white mt-2">Workout Graded!</h2>
        <p class="text-xs text-slate-300 mb-4">Official Royal Canadian Air Force 5BX Verdict</p>

        <div class="space-y-3 bg-slate-900/80 p-4 rounded-2xl border border-slate-700 text-left mb-5">
          <div>
            <span class="text-xs text-slate-400 font-semibold uppercase block">Strength Result:</span>
            <div class="text-base font-bold text-emerald-400">{{ celebrationResult.strength_verdict }}</div>
            <div class="text-xs text-slate-300">New Position: {{ celebrationResult.new_strength }}</div>
          </div>

          <div class="border-t border-slate-800 pt-2">
            <span class="text-xs text-slate-400 font-semibold uppercase block">Cardio Result:</span>
            <div class="text-base font-bold text-blue-400">{{ celebrationResult.cardio_verdict }}</div>
            <div class="text-xs text-slate-300">New Position: {{ celebrationResult.new_cardio }}</div>
          </div>
        </div>

        <div v-if="celebrationResult.new_badges.length > 0" class="mb-5 bg-amber-500/20 border border-amber-500/40 p-3 rounded-xl text-left">
          <span class="text-xs font-bold text-amber-300 uppercase block mb-1">New Milestone Unlocked!</span>
          <div v-for="b in celebrationResult.new_badges" :key="b.key" class="text-sm font-bold text-white flex items-center gap-2">
            <span>🏆</span>
            <span>{{ b.title }}</span>
          </div>
        </div>

        <button
          @click="celebrationResult = null"
          class="w-full py-3 bg-emerald-600 hover:bg-emerald-500 text-white font-bold rounded-xl shadow-lg text-base transition-colors"
        >
          Acknowledge & Continue
        </button>
      </div>
    </div>
  </div>
</template>
