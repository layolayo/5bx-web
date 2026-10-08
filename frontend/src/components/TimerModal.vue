<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import { TodayWorkout, SubmitWorkoutPayload, ExerciseChartRow, WorkoutSessionHistory } from '../types';
import { fetchSystemCharts } from '../api';
import { playCountdownBeep, playTransitionChime, playCelebrationChime } from '../audio';
import { usePrecisionTimer } from '../composables/usePrecisionTimer';
import { getLastPerformance, getNextRungTarget, calculateTreadmillSpeed, formatDurationMmSs } from '../telemetry';

const props = defineProps<{
  workout: TodayWorkout;
  history?: WorkoutSessionHistory[];
  isGuest?: boolean;
  initialCardioMode?: 'stationary' | 'run' | 'walk';
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'submit', payload: SubmitWorkoutPayload): void;
}>();

// Workout Step State Machine: 'ready' | 'countdown' | 'running' | 'record' | 'finished'
type WorkoutStage = 'ready' | 'countdown' | 'running' | 'record' | 'finished';
const stage = ref<WorkoutStage>('ready');

const currentExerciseIndex = ref(0);
const secondsRemaining = ref(props.workout.exercises[0].time_limit_seconds);
const countdownDisplay = ref<number | string>(3);
let countdownTimeout: any = null;

// System charts cache for progressive next-level targets
const systemCharts = ref<ExerciseChartRow[]>([]);

// Precision Wall-Clock Timer with Screen Wake Lock
const precisionTimer = usePrecisionTimer();

// Recorded Reps State for each exercise (Ex 1 to 5)
const recordedReps = ref<number[]>([
  props.workout.exercises[0].target_reps,
  props.workout.exercises[1].target_reps,
  props.workout.exercises[2].target_reps,
  props.workout.exercises[3].target_reps,
  props.workout.exercises[4].target_reps,
]);

// Cardio Discipline State
const cardioMode = ref<'stationary' | 'run' | 'walk'>(props.initialCardioMode || 'stationary');
const cardioTimeString = ref('');
const notes = ref('');

// Sync initial time when exercise index or cardio mode changes
function resetExerciseTime() {
  const ex = props.workout.exercises[currentExerciseIndex.value];
  if (!ex) return;

  if (currentExerciseIndex.value === 4) {
    if (cardioMode.value === 'run') {
      secondsRemaining.value = ex.alt_run_time_seconds || 360;
    } else if (cardioMode.value === 'walk') {
      secondsRemaining.value = ex.alt_walk_time_seconds || 360;
    } else {
      secondsRemaining.value = ex.time_limit_seconds || 360;
    }
  } else {
    secondsRemaining.value = ex.time_limit_seconds;
  }
}

watch(currentExerciseIndex, () => {
  resetExerciseTime();
});

watch(cardioMode, () => {
  if (currentExerciseIndex.value === 4) {
    resetExerciseTime();
  }
});

const currentExercise = computed(() => props.workout.exercises[currentExerciseIndex.value]);

const cardioDistanceMiles = computed(() => {
  if (cardioMode.value === 'run') {
    return props.workout.cardio_chart === 1 ? 0.5 : 1.0;
  }
  if (cardioMode.value === 'walk') {
    return props.workout.cardio_chart === 1 ? 1.0 : 2.0;
  }
  return 0;
});

const currentCardioTargetSeconds = computed(() => {
  const ex5 = props.workout.exercises[4];
  if (!ex5) return 0;
  if (cardioMode.value === 'run') return ex5.alt_run_time_seconds;
  if (cardioMode.value === 'walk') return ex5.alt_walk_time_seconds;
  return ex5.time_limit_seconds;
});

const treadmillSpeed = computed(() => {
  const targetSec = currentCardioTargetSeconds.value;
  if (!targetSec || targetSec <= 0 || cardioDistanceMiles.value <= 0) return { mph: 0, kph: 0 };
  const hours = targetSec / 3600.0;
  const mph = cardioDistanceMiles.value / hours;
  const kph = mph * 1.60934;
  return { mph, kph };
});

function parseTimeStringToSeconds(str: string): number {
  if (!str) return 0;
  const trimmed = str.trim();
  if (trimmed.includes(':')) {
    const parts = trimmed.split(':');
    const mins = parseInt(parts[0], 10) || 0;
    const secs = parseInt(parts[1], 10) || 0;
    return mins * 60 + secs;
  }
  const num = parseFloat(trimmed);
  if (!isNaN(num)) {
    if (num > 100) return Math.round(num);
    return Math.round(num * 60);
  }
  return 0;
}

const totalCardioSecondsEntered = computed(() => parseTimeStringToSeconds(cardioTimeString.value));

const achievedPace = computed(() => {
  const sec = totalCardioSecondsEntered.value;
  if (sec <= 0 || cardioDistanceMiles.value <= 0) return { mph: 0, kph: 0 };
  const hours = sec / 3600.0;
  const mph = cardioDistanceMiles.value / hours;
  const kph = mph * 1.60934;
  return { mph, kph };
});

function formatDuration(sec: number) {
  const m = Math.floor(sec / 60);
  const s = sec % 60;
  return s > 0 ? `${m}m ${s}s` : `${m}m`;
}

const formattedCountdown = computed(() => {
  return precisionTimer.formattedRemaining.value;
});

const progressPercent = computed(() => {
  let passed = 0;
  for (let i = 0; i < currentExerciseIndex.value; i++) {
    passed += props.workout.exercises[i].time_limit_seconds;
  }
  if (stage.value === 'running') {
    passed += precisionTimer.elapsedSeconds.value;
  } else if (stage.value === 'record' || stage.value === 'finished') {
    passed += currentExercise.value.time_limit_seconds;
  }
  return Math.min(100, Math.round((passed / 660) * 100));
});

// Telemetry helpers for current exercise
const currentPreviousPerformance = computed(() => {
  return getLastPerformance(props.history || [], currentExerciseIndex.value + 1, cardioMode.value);
});

const currentNextRungTarget = computed(() => {
  if (currentExerciseIndex.value < 4) {
    return getNextRungTarget(
      systemCharts.value,
      props.workout.strength_chart,
      props.workout.strength_level,
      currentExerciseIndex.value + 1
    );
  }
  return getNextRungTarget(
    systemCharts.value,
    props.workout.cardio_chart,
    props.workout.cardio_level,
    5,
    cardioMode.value
  );
});

// START INITIATION WITH 3-2-1 COUNTDOWN
function initiateExercise() {
  stage.value = 'countdown';
  countdownDisplay.value = 3;
  playCountdownBeep(false);

  countdownTimeout = setTimeout(() => {
    countdownDisplay.value = 2;
    playCountdownBeep(false);

    countdownTimeout = setTimeout(() => {
      countdownDisplay.value = 1;
      playCountdownBeep(false);

      countdownTimeout = setTimeout(() => {
        countdownDisplay.value = 'GO!';
        playCountdownBeep(true);

        countdownTimeout = setTimeout(() => {
          stage.value = 'running';
          precisionTimer.startTimer(secondsRemaining.value, completeExercise);
        }, 600);
      }, 1000);
    }, 1000);
  }, 1000);
}

function completeExercise() {
  const elapsed = precisionTimer.elapsedSeconds.value;
  precisionTimer.stopTimer();
  playTransitionChime();
  if (currentExerciseIndex.value === 4 && cardioMode.value !== 'stationary') {
    const m = Math.floor(elapsed / 60);
    const s = elapsed % 60;
    cardioTimeString.value = `${m}:${s.toString().padStart(2, '0')}`;
  }
  stage.value = 'record';
}

function togglePause() {
  precisionTimer.togglePause();
}

// Adjust reps via buttons
function adjustRep(delta: number) {
  const current = recordedReps.value[currentExerciseIndex.value] || 0;
  recordedReps.value[currentExerciseIndex.value] = Math.max(0, current + delta);
}

// Confirm recorded reps and advance
function confirmExerciseAndProceed() {
  if (currentExerciseIndex.value < 4) {
    currentExerciseIndex.value++;
    stage.value = 'ready';
  } else {
    playCelebrationChime();
    stage.value = 'finished';
  }
}

function submitFinalWorkout() {
  const finalDuration = cardioMode.value === 'stationary' ? 0 : totalCardioSecondsEntered.value;
  emit('submit', {
    reps_1: recordedReps.value[0],
    reps_2: recordedReps.value[1],
    reps_3: recordedReps.value[2],
    reps_4: recordedReps.value[3],
    reps_5: recordedReps.value[4],
    cardio_mode: cardioMode.value,
    cardio_duration_secs: finalDuration,
    notes: notes.value,
  });
}

onMounted(async () => {
  resetExerciseTime();
  try {
    const data = await fetchSystemCharts();
    if (data?.charts) systemCharts.value = data.charts;
  } catch (e) {
    console.error('Failed to load system charts in TimerModal:', e);
  }
});

onUnmounted(() => {
  precisionTimer.stopTimer();
  if (countdownTimeout) clearTimeout(countdownTimeout);
});
</script>

<template>
  <div class="fixed inset-0 z-50 bg-slate-950/90 backdrop-blur-md flex items-center justify-center p-4 animate-in fade-in duration-200">
    
    <!-- STAGE 1: GET READY / INITIATE SCREEN -->
    <div
      v-if="stage === 'ready'"
      class="bg-slate-900 border border-slate-700/80 rounded-3xl w-full max-w-xl overflow-hidden shadow-2xl flex flex-col relative max-h-[92vh]"
    >
      <div class="absolute -top-20 left-1/2 -translate-x-1/2 w-64 h-32 bg-emerald-500/15 blur-2xl pointer-events-none"></div>

      <!-- Top Bar -->
      <div class="bg-slate-950/80 p-4 border-b border-slate-800 flex justify-between items-center relative z-10">
        <div class="flex items-center gap-2">
          <span class="w-2.5 h-2.5 rounded-full bg-emerald-400"></span>
          <span class="text-xs font-bold uppercase tracking-wider text-emerald-400">
            Exercise {{ currentExerciseIndex + 1 }} of 5
          </span>
          <span class="text-slate-600">•</span>
          <span class="text-xs font-bold text-slate-300">{{ currentExercise.name }}</span>
        </div>
        <button
          @click="emit('close')"
          class="w-8 h-8 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-white flex items-center justify-center text-xs transition-colors cursor-pointer"
          aria-label="Close"
        >
          ✕
        </button>
      </div>

      <!-- Briefing Body -->
      <div class="p-6 sm:p-8 flex flex-col items-center text-center relative z-10 overflow-y-auto">
        <!-- Posture Illustration -->
        <div class="w-52 h-36 bg-white rounded-2xl p-2.5 flex items-center justify-center shadow-inner mb-5 border border-slate-300">
          <img :src="'/images/' + currentExercise.image_path" :alt="currentExercise.name" class="max-h-full max-w-full object-contain" />
        </div>

        <h3 class="text-2xl font-black text-white tracking-tight uppercase mb-1">
          {{ currentExercise.name }}
        </h3>
        <p class="text-xs text-slate-400 mb-4 max-w-md leading-relaxed">
          {{ currentExercise.instructions }}
        </p>

        <!-- CARDIO TRACK CHOICE INTERCEPT (On Exercise 5) -->
        <div v-if="currentExerciseIndex === 4" class="w-full bg-slate-950/90 p-4 rounded-2xl border border-cyan-500/30 mb-5 text-left">
          <div class="flex items-center justify-between mb-3">
            <span class="text-[11px] font-bold uppercase tracking-wider text-cyan-400 flex items-center gap-1.5">
              <span>🏃</span> Select Cardio Discipline:
            </span>
            <span class="text-[10px] text-slate-400 font-mono">Chart {{ workout.cardio_chart }}</span>
          </div>

          <!-- 3-Choice Discipline Cards -->
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-2">
            <!-- Stationary -->
            <button
              type="button"
              @click="cardioMode = 'stationary'"
              :class="[
                'p-3 rounded-xl border text-left transition-all cursor-pointer flex flex-col justify-between',
                cardioMode === 'stationary'
                  ? 'bg-cyan-500/20 border-cyan-400 text-white shadow-md'
                  : 'bg-slate-900 border-slate-800 text-slate-400 hover:border-slate-700'
              ]"
            >
              <div class="text-xs font-black uppercase flex items-center gap-1">
                <span>👟</span> Stationary
              </div>
              <div class="text-[11px] font-bold text-cyan-300 mt-1">
                {{ workout.exercises[4]?.target_reps }} steps
              </div>
              <div class="text-[10px] text-slate-500 mt-0.5">6 Mins indoor run</div>
            </button>

            <!-- 1-Mile Run -->
            <button
              type="button"
              @click="cardioMode = 'run'"
              :class="[
                'p-3 rounded-xl border text-left transition-all cursor-pointer flex flex-col justify-between',
                cardioMode === 'run'
                  ? 'bg-cyan-500/20 border-cyan-400 text-white shadow-md'
                  : 'bg-slate-900 border-slate-800 text-slate-400 hover:border-slate-700'
              ]"
            >
              <div class="text-xs font-black uppercase flex items-center gap-1">
                <span>🏃</span> {{ workout.cardio_chart === 1 ? '0.5-Mi Run' : '1-Mi Run' }}
              </div>
              <div class="text-[11px] font-bold text-cyan-300 mt-1">
                &lt; {{ formatDuration(workout.exercises[4]?.alt_run_time_seconds) }}
              </div>
              <div class="text-[10px] text-slate-500 mt-0.5">
                {{ workout.cardio_chart === 1 ? '0.5 mi (0.8 km)' : '1.0 mi (1.6 km)' }} run
              </div>
            </button>

            <!-- 2-Mile Walk -->
            <button
              type="button"
              @click="cardioMode = 'walk'"
              :class="[
                'p-3 rounded-xl border text-left transition-all cursor-pointer flex flex-col justify-between',
                cardioMode === 'walk'
                  ? 'bg-cyan-500/20 border-cyan-400 text-white shadow-md'
                  : 'bg-slate-900 border-slate-800 text-slate-400 hover:border-slate-700'
              ]"
            >
              <div class="text-xs font-black uppercase flex items-center gap-1">
                <span>🚶</span> {{ workout.cardio_chart === 1 ? '1-Mi Walk' : '2-Mi Walk' }}
              </div>
              <div class="text-[11px] font-bold text-cyan-300 mt-1">
                &lt; {{ formatDuration(workout.exercises[4]?.alt_walk_time_seconds) }}
              </div>
              <div class="text-[10px] text-slate-500 mt-0.5">
                {{ workout.cardio_chart === 1 ? '1.0 mi (1.6 km)' : '2.0 mi (3.2 km)' }} walk
              </div>
            </button>
          </div>

          <!-- Speed banner if Run/Walk selected -->
          <div v-if="cardioMode !== 'stationary' && treadmillSpeed.mph > 0" class="mt-3 p-2.5 rounded-xl bg-slate-900 border border-cyan-500/30 text-xs flex items-center justify-between font-mono">
            <span class="text-slate-400">Treadmill Target Speed:</span>
            <span class="font-bold text-cyan-300">{{ treadmillSpeed.kph.toFixed(1) }} km/h ({{ treadmillSpeed.mph.toFixed(1) }} mph)</span>
          </div>
        </div>

        <!-- Telemetry Cluster: Previous Sortie | Today's Standard | Next Rung Target -->
        <div class="w-full grid grid-cols-3 gap-2 bg-slate-950/80 p-3 rounded-2xl border border-slate-800 text-left mb-4 font-mono text-xs">
          <div class="border-r border-slate-800 pr-2">
            <span class="text-[10px] text-slate-500 uppercase font-bold block mb-0.5">Previous Sortie</span>
            <span class="text-amber-400 font-bold">
              {{ currentPreviousPerformance?.display || 'None logged' }}
            </span>
          </div>
          <div class="border-r border-slate-800 pr-2 pl-1">
            <span class="text-[10px] text-slate-500 uppercase font-bold block mb-0.5">Today's Standard</span>
            <span class="text-emerald-400 font-bold">
              <template v-if="currentExerciseIndex < 4">
                {{ currentExercise.target_reps }} reps
              </template>
              <template v-else-if="cardioMode === 'stationary'">
                {{ currentExercise.target_reps }} steps
              </template>
              <template v-else-if="cardioMode === 'run'">
                ≤ {{ formatDuration(currentExercise.alt_run_time_seconds) }}
              </template>
              <template v-else>
                ≤ {{ formatDuration(currentExercise.alt_walk_time_seconds) }}
              </template>
            </span>
          </div>
          <div class="pl-1">
            <span class="text-[10px] text-slate-500 uppercase font-bold block mb-0.5">Next Rung Target</span>
            <span class="text-cyan-400 font-bold">
              {{ currentNextRungTarget?.display || 'Max Standard' }}
            </span>
          </div>
        </div>

        <!-- Target Envelope Badge & Screen Wake Lock status -->
        <div class="flex flex-col items-center gap-2 mb-6">
          <div class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-slate-950 border border-slate-800 text-sm font-bold text-slate-300 shadow-inner">
            <span class="text-slate-400">Mission Envelope:</span>
            <span class="text-emerald-400 font-mono">
              <template v-if="currentExerciseIndex < 4">
                {{ currentExercise.target_reps }} reps in {{ formatDuration(currentExercise.time_limit_seconds) }}
              </template>
              <template v-else-if="cardioMode === 'stationary'">
                {{ currentExercise.target_reps }} steps in 6m 00s
              </template>
              <template v-else-if="cardioMode === 'run'">
                Beat {{ formatDuration(currentExercise.alt_run_time_seconds) }}
              </template>
              <template v-else>
                Beat {{ formatDuration(currentExercise.alt_walk_time_seconds) }}
              </template>
            </span>
          </div>

          <div v-if="precisionTimer.isWakeLockActive.value" class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-emerald-950/70 border border-emerald-500/30 text-[11px] font-mono font-bold text-emerald-400">
            <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
            <span>📱 Screen Stay-Awake Active</span>
          </div>
        </div>

        <!-- Big INITIATE Button -->
        <button
          @click="initiateExercise"
          class="w-full py-4 rounded-2xl bg-gradient-to-r from-emerald-500 via-teal-500 to-cyan-500 hover:from-emerald-400 hover:to-cyan-400 active:scale-[0.98] text-slate-950 font-black text-base uppercase tracking-wider shadow-xl shadow-emerald-500/20 flex items-center justify-center gap-2.5 transition-transform cursor-pointer"
        >
          <span class="text-xl">🟢</span>
          <span>INITIATE EXERCISE (Start Countdown)</span>
        </button>

        <button
          @click="completeExercise"
          class="mt-3 text-xs text-slate-500 hover:text-slate-300 underline cursor-pointer"
        >
          Already completed? Enter reps directly →
        </button>
      </div>
    </div>

    <!-- STAGE 2: 3-2-1 COUNTDOWN OVERLAY -->
    <div
      v-else-if="stage === 'countdown'"
      class="bg-slate-900/95 border-2 border-cyan-500 rounded-3xl w-full max-w-sm p-12 text-center shadow-2xl flex flex-col items-center justify-center animate-in zoom-in-95 duration-150"
    >
      <div class="text-xs font-black uppercase tracking-widest text-cyan-400 mb-4">
        GET SET — {{ currentExercise.name }}
      </div>
      <div class="text-9xl font-black font-mono text-white tracking-tighter tabular-nums drop-shadow-[0_0_35px_rgba(6,182,212,0.6)]">
        {{ countdownDisplay }}
      </div>
      <div class="text-xs font-bold text-slate-400 mt-6 tracking-wider uppercase">
        Prepare to begin
      </div>
    </div>

    <!-- STAGE 3: ACTIVE RUNNING TIMER SCREEN -->
    <div
      v-else-if="stage === 'running'"
      class="bg-slate-900 border border-slate-700/80 rounded-3xl w-full max-w-xl overflow-hidden shadow-2xl flex flex-col relative"
    >
      <div class="absolute -top-20 left-1/2 -translate-x-1/2 w-64 h-32 bg-cyan-500/15 blur-2xl pointer-events-none"></div>

      <!-- Top Bar -->
      <div class="bg-slate-950/80 p-4 border-b border-slate-800 flex justify-between items-center relative z-10">
        <div class="flex items-center gap-2">
          <span class="w-2.5 h-2.5 rounded-full bg-cyan-400 animate-pulse"></span>
          <span class="text-xs font-bold uppercase tracking-wider text-cyan-400">
            Exercise {{ currentExerciseIndex + 1 }} of 5
          </span>
          <span class="text-slate-600">•</span>
          <span class="text-xs font-bold text-slate-300">{{ currentExercise.name }}</span>
        </div>
        <button
          @click="emit('close')"
          class="w-8 h-8 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-white flex items-center justify-center text-xs transition-colors cursor-pointer"
          aria-label="Close"
        >
          ✕
        </button>
      </div>

      <!-- Main Body -->
      <div class="p-6 sm:p-8 flex flex-col items-center text-center relative z-10">
        <!-- Exercise Illustration -->
        <div class="w-48 h-32 bg-white rounded-2xl p-2.5 flex items-center justify-center shadow-inner mb-5 border border-slate-300">
          <img :src="'/images/' + currentExercise.image_path" :alt="currentExercise.name" class="max-h-full max-w-full object-contain" />
        </div>

        <!-- Big Countdown Timer & Live Elapsed -->
        <div class="text-7xl font-black font-mono tracking-tight text-white mb-1 tabular-nums">
          {{ formattedCountdown }}
        </div>
        <div class="text-xs font-mono text-slate-400 mb-3">
          Elapsed: <span class="text-cyan-400 font-bold">{{ precisionTimer.formattedElapsed.value }}</span>
        </div>

        <!-- Screen Wake Lock indicator -->
        <div v-if="precisionTimer.isWakeLockActive.value" class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full bg-emerald-950/70 border border-emerald-500/30 text-[10px] font-mono font-bold text-emerald-400 mb-4">
          <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
          <span>📱 Stay-Awake Active</span>
        </div>

        <!-- Cadence Target & Telemetry strip -->
        <div class="w-full bg-slate-950 border border-slate-800 rounded-xl p-2.5 mb-5 flex flex-wrap items-center justify-around gap-2 text-xs font-mono">
          <div v-if="currentPreviousPerformance" class="text-slate-400">
            Previous: <span class="text-amber-400 font-bold">{{ currentPreviousPerformance.display }}</span>
          </div>
          <div class="text-slate-300">
            Target: <span class="text-emerald-400 font-bold">
              <template v-if="currentExerciseIndex < 4">
                {{ currentExercise.target_reps }} reps
              </template>
              <template v-else-if="cardioMode === 'stationary'">
                {{ currentExercise.target_reps }} steps
              </template>
              <template v-else>
                &lt; {{ formatDuration(currentCardioTargetSeconds) }}
              </template>
            </span>
          </div>
          <div v-if="currentNextRungTarget" class="text-slate-400">
            Next Rung: <span class="text-cyan-400 font-bold">{{ currentNextRungTarget.display }}</span>
          </div>
        </div>

        <!-- Speed banner if Run/Walk -->
        <div v-if="currentExerciseIndex === 4 && cardioMode !== 'stationary' && treadmillSpeed.mph > 0" class="w-full mb-4 p-2 rounded-xl bg-slate-950 border border-cyan-500/30 text-xs font-mono text-cyan-300 flex items-center justify-between">
          <span class="text-slate-400">Treadmill Target Speed:</span>
          <span class="font-bold">≥ {{ treadmillSpeed.kph.toFixed(1) }} km/h ({{ treadmillSpeed.mph.toFixed(1) }} mph)</span>
        </div>

        <!-- Technique instructions -->
        <div class="w-full bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 text-xs text-slate-300 text-left mb-5 max-h-20 overflow-y-auto">
          <span class="text-[10px] font-bold text-slate-500 uppercase tracking-wider block mb-1">Posture Cue</span>
          {{ currentExercise.instructions }}
        </div>

        <!-- Progress bar -->
        <div class="w-full mb-6">
          <div class="flex justify-between text-[11px] font-mono text-slate-400 mb-1.5">
            <span>Overall Cadence</span>
            <span class="text-cyan-400 font-bold">{{ progressPercent }}%</span>
          </div>
          <div class="w-full bg-slate-950 rounded-full h-2.5 overflow-hidden border border-slate-800">
            <div
              class="h-full bg-gradient-to-r from-cyan-500 via-teal-400 to-emerald-400 rounded-full transition-all duration-500"
              :style="{ width: progressPercent + '%' }"
            ></div>
          </div>
        </div>

        <!-- Controls -->
        <div class="flex gap-3 w-full">
          <button
            @click="togglePause"
            class="flex-1 py-3.5 rounded-xl font-black text-sm uppercase tracking-wider shadow transition-all cursor-pointer"
            :class="precisionTimer.isPaused.value ? 'bg-emerald-500 hover:bg-emerald-400 text-slate-950' : 'bg-slate-800 hover:bg-slate-700 text-amber-400 border border-slate-700'"
          >
            {{ precisionTimer.isPaused.value ? '▶ Resume' : '⏸ Pause' }}
          </button>
          <button
            @click="completeExercise"
            class="flex-1 py-3.5 rounded-xl font-black text-sm uppercase tracking-wider bg-gradient-to-r from-cyan-500 to-blue-600 hover:from-cyan-400 hover:to-blue-500 text-slate-950 shadow-md shadow-cyan-500/20 transition-all cursor-pointer"
          >
            I'm Done Early →
          </button>
        </div>
      </div>
    </div>

    <!-- STAGE 4: INTER-EXERCISE REP RECORDING PROMPT -->
    <div
      v-else-if="stage === 'record'"
      class="bg-slate-900 border-2 border-emerald-500/80 rounded-3xl w-full max-w-lg p-6 sm:p-8 shadow-2xl flex flex-col relative animate-in zoom-in-95 duration-150"
    >
      <div class="text-center mb-5">
        <div class="w-12 h-12 rounded-xl bg-emerald-500/20 border border-emerald-500/40 text-emerald-400 flex items-center justify-center text-2xl mx-auto mb-2">
          ✓
        </div>
        <h3 class="text-xl font-black text-white uppercase tracking-tight">
          Exercise {{ currentExerciseIndex + 1 }} Complete!
        </h3>
        <p class="text-xs text-slate-400 mt-0.5">
          Record your actual score for <strong>{{ currentExercise.name }}</strong>
        </p>
      </div>

      <!-- Telemetry Context: Previous Sortie | Current Target | Next Level -->
      <div class="w-full bg-slate-950/70 border border-slate-800 rounded-xl p-2.5 mb-4 flex flex-wrap items-center justify-around gap-2 text-xs font-mono">
        <div v-if="currentPreviousPerformance" class="text-slate-400">
          Previous: <span class="text-amber-400 font-bold">{{ currentPreviousPerformance.display }}</span>
        </div>
        <div class="text-slate-300">
          Standard: <span class="text-emerald-400 font-bold">
            <template v-if="currentExerciseIndex < 4">
              {{ currentExercise.target_reps }} reps
            </template>
            <template v-else-if="cardioMode === 'stationary'">
              {{ currentExercise.target_reps }} steps
            </template>
            <template v-else>
              &lt; {{ formatDuration(currentCardioTargetSeconds) }}
            </template>
          </span>
        </div>
        <div v-if="currentNextRungTarget" class="text-slate-400">
          Next Rung: <span class="text-cyan-400 font-bold">{{ currentNextRungTarget.display }}</span>
        </div>
      </div>

      <!-- Rep input for Ex 1-4 -->
      <div v-if="currentExerciseIndex < 4" class="bg-slate-950 p-5 rounded-2xl border border-slate-800 space-y-4 mb-6">
        <div class="flex items-center justify-between text-xs text-slate-400 font-mono">
          <span>Target Requirement:</span>
          <span class="text-emerald-400 font-bold text-sm">{{ currentExercise.target_reps }} reps</span>
        </div>

        <div class="flex items-center justify-center gap-3">
          <button
            type="button"
            @click="adjustRep(-1)"
            class="w-12 h-12 rounded-xl bg-slate-900 border border-slate-700 text-white font-black text-xl hover:bg-slate-800 transition cursor-pointer"
          >
            -
          </button>

          <input
            v-model.number="recordedReps[currentExerciseIndex]"
            type="number"
            min="0"
            max="150"
            class="w-32 bg-slate-900 text-white font-mono font-black text-3xl p-2 rounded-xl border border-slate-700 text-center focus:border-emerald-500 focus:outline-none"
          />

          <button
            type="button"
            @click="adjustRep(+1)"
            class="w-12 h-12 rounded-xl bg-slate-900 border border-slate-700 text-white font-black text-xl hover:bg-slate-800 transition cursor-pointer"
          >
            +
          </button>
        </div>

        <!-- Quick bump buttons -->
        <div class="flex justify-center gap-2 pt-1">
          <button
            type="button"
            @click="recordedReps[currentExerciseIndex] = currentExercise.target_reps"
            class="px-3 py-1 rounded-lg bg-slate-900 border border-slate-800 text-[11px] text-slate-400 hover:text-white"
          >
            Reset to Target ({{ currentExercise.target_reps }})
          </button>
          <button
            type="button"
            @click="adjustRep(+5)"
            class="px-3 py-1 rounded-lg bg-slate-900 border border-slate-800 text-[11px] text-emerald-400 hover:text-emerald-300 font-bold"
          >
            +5 Bonus
          </button>
        </div>
      </div>

      <!-- Ex 5 Cardio Recording -->
      <div v-else class="bg-slate-950 p-5 rounded-2xl border border-slate-800 space-y-4 mb-6">
        <!-- Stationary Steps -->
        <div v-if="cardioMode === 'stationary'" class="space-y-3">
          <div class="flex items-center justify-between text-xs text-slate-400 font-mono">
            <span>Target Steps:</span>
            <span class="text-cyan-400 font-bold text-sm">{{ currentExercise.target_reps }} steps</span>
          </div>

          <div class="flex items-center justify-center gap-3">
            <button
              type="button"
              @click="recordedReps[4] = Math.max(0, recordedReps[4] - 10)"
              class="w-12 h-12 rounded-xl bg-slate-900 border border-slate-700 text-white font-black text-sm hover:bg-slate-800 transition cursor-pointer"
            >
              -10
            </button>

            <input
              v-model.number="recordedReps[4]"
              type="number"
              min="0"
              max="1500"
              step="10"
              class="w-36 bg-slate-900 text-cyan-300 font-mono font-black text-3xl p-2 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none"
            />

            <button
              type="button"
              @click="recordedReps[4] += 10"
              class="w-12 h-12 rounded-xl bg-slate-900 border border-slate-700 text-white font-black text-sm hover:bg-slate-800 transition cursor-pointer"
            >
              +10
            </button>
          </div>
        </div>

        <!-- Timed Run/Walk mm:ss -->
        <div v-else class="space-y-3">
          <div class="flex items-center justify-between text-xs text-slate-400 font-mono">
            <span>Standard Time:</span>
            <span class="text-cyan-400 font-bold text-sm">Under {{ formatDuration(currentCardioTargetSeconds) }}</span>
          </div>

          <div v-if="cardioTimeString" class="text-[11px] font-mono text-cyan-400 font-bold text-center bg-cyan-950/40 border border-cyan-500/20 py-1 rounded-lg">
            Elapsed Time Captured: {{ cardioTimeString }}
          </div>

          <div>
            <label class="text-[10px] text-slate-400 font-bold uppercase tracking-wider block mb-1">
              Time Taken (mm:ss)
            </label>
            <input
              v-model="cardioTimeString"
              type="text"
              placeholder="e.g. 07:30"
              class="w-full bg-slate-900 text-cyan-300 font-mono font-black text-2xl p-2.5 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none tracking-widest"
            />
          </div>

          <!-- Pace Indicator -->
          <div
            v-if="totalCardioSecondsEntered > 0"
            class="p-2.5 rounded-xl bg-slate-900 border border-slate-800 text-xs font-mono flex items-center justify-between"
          >
            <span :class="totalCardioSecondsEntered <= currentCardioTargetSeconds ? 'text-emerald-400 font-bold' : 'text-amber-400 font-bold'">
              {{ totalCardioSecondsEntered <= currentCardioTargetSeconds ? '✓ Target Beaten' : '⚠️ Over Target' }}
            </span>
            <span class="text-white font-bold">
              {{ achievedPace.kph.toFixed(1) }} km/h ({{ achievedPace.mph.toFixed(1) }} mph)
            </span>
          </div>
        </div>
      </div>

      <!-- Action Button -->
      <button
        @click="confirmExerciseAndProceed"
        class="w-full py-4 rounded-2xl bg-gradient-to-r from-emerald-500 via-teal-500 to-cyan-500 hover:from-emerald-400 hover:to-cyan-400 text-slate-950 font-black text-sm uppercase tracking-wider shadow-xl shadow-emerald-500/20 transition cursor-pointer"
      >
        {{ currentExerciseIndex < 4 ? `Confirm & Proceed to Exercise ${currentExerciseIndex + 2} →` : 'Confirm & Review Mission Debrief 🎖️' }}
      </button>
    </div>

    <!-- STAGE 5: FINAL DEBRIEF & SUBMISSION SCREEN -->
    <div
      v-else-if="stage === 'finished'"
      class="bg-slate-900 border border-slate-700/80 rounded-3xl w-full max-w-lg overflow-hidden shadow-2xl flex flex-col p-6 sm:p-8 max-h-[92vh] overflow-y-auto relative animate-in fade-in duration-200"
    >
      <div class="text-center mb-5">
        <div class="w-14 h-14 rounded-2xl bg-emerald-500/20 border border-emerald-500/40 text-emerald-400 flex items-center justify-center text-3xl mx-auto mb-2">
          🎖️
        </div>
        <h2 class="text-2xl font-black text-white uppercase tracking-tight">Mission Accomplished!</h2>
        <p class="text-xs text-slate-300 mt-0.5">
          All 5 exercises logged. Review your report card before grading.
        </p>
      </div>

      <!-- Reps Summary Table -->
      <div class="bg-slate-950 rounded-2xl border border-slate-800 overflow-hidden mb-5">
        <div class="p-3 bg-slate-900/60 border-b border-slate-800 text-[10px] font-bold uppercase tracking-wider text-slate-400 flex justify-between">
          <span>Exercise</span>
          <span>Performance</span>
        </div>

        <div
          v-for="(ex, i) in workout.exercises.slice(0, 4)"
          :key="ex.exercise_number"
          class="p-3 border-b border-slate-800/60 flex items-center justify-between text-xs"
        >
          <span class="text-slate-300 font-semibold">{{ ex.exercise_number }}. {{ ex.name }}</span>
          <span class="font-mono font-bold text-emerald-400">
            {{ recordedReps[i] }} reps <span class="text-slate-600 font-normal">/ {{ ex.target_reps }}</span>
          </span>
        </div>

        <!-- Cardio Summary -->
        <div class="p-3 flex items-center justify-between text-xs bg-slate-900/30">
          <span class="text-slate-300 font-semibold">5. Cardio ({{ cardioMode }})</span>
          <span class="font-mono font-bold text-cyan-400">
            <template v-if="cardioMode === 'stationary'">
              {{ recordedReps[4] }} steps <span class="text-slate-600 font-normal">/ {{ workout.exercises[4]?.target_reps }}</span>
            </template>
            <template v-else>
              {{ cardioTimeString || 'Recorded' }}
            </template>
          </span>
        </div>
      </div>

      <!-- Debrief notes -->
      <div class="mb-5">
        <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
          Flight Debrief Notes (Optional)
        </label>
        <input
          v-model="notes"
          type="text"
          placeholder="Felt strong, good cadence, smooth form..."
          class="w-full bg-slate-950 text-white p-3 rounded-xl border border-slate-800 text-xs focus:border-cyan-500 focus:outline-none"
        />
      </div>

      <!-- Actions -->
      <div class="flex gap-3">
        <button
          @click="emit('close')"
          class="flex-1 py-3.5 rounded-xl bg-slate-800 text-slate-300 font-bold hover:bg-slate-700 text-xs uppercase tracking-wider transition cursor-pointer"
        >
          Dismiss
        </button>
        <button
          @click="submitFinalWorkout"
          class="flex-2 py-3.5 rounded-xl bg-gradient-to-r from-emerald-500 to-teal-600 hover:from-emerald-400 hover:to-teal-500 text-slate-950 font-black text-xs uppercase tracking-wider shadow-lg shadow-emerald-500/20 transition cursor-pointer"
        >
          {{ isGuest ? 'Finish Guest Session' : 'Submit & Calculate Verdict 🎖️' }}
        </button>
      </div>
    </div>

  </div>
</template>
