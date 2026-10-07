<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { TodayWorkout, SubmitWorkoutPayload } from '../types';
import { playCountdownBeep, playTransitionChime, playCelebrationChime } from '../audio';

const props = defineProps<{
  workout: TodayWorkout;
  isGuest?: boolean;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'submit', payload: SubmitWorkoutPayload): void;
}>();

// Timer State
const currentExerciseIndex = ref(0);
const secondsRemaining = ref(props.workout.exercises[0].time_limit_seconds);
const isPaused = ref(false);
const isFinished = ref(false);
let timerInterval: any = null;

// Reps Input State for completion
const reps1 = ref(props.workout.exercises[0].target_reps);
const reps2 = ref(props.workout.exercises[1].target_reps);
const reps3 = ref(props.workout.exercises[2].target_reps);
const reps4 = ref(props.workout.exercises[3].target_reps);
const reps5 = ref(props.workout.exercises[4].target_reps);
const cardioMode = ref<'stationary' | 'run' | 'walk'>('stationary');
const cardioMinutes = ref<number | ''>('');
const cardioSeconds = ref<number | ''>('');
const notes = ref('');

const currentCardioTarget = computed(() => {
  const ex5 = props.workout.exercises[4];
  if (!ex5) return 0;
  if (cardioMode.value === 'run') return ex5.alt_run_time_seconds;
  if (cardioMode.value === 'walk') return ex5.alt_walk_time_seconds;
  return ex5.target_reps;
});

const totalSecondsEntered = computed(() => {
  const m = typeof cardioMinutes.value === 'number' ? cardioMinutes.value : 0;
  const s = typeof cardioSeconds.value === 'number' ? cardioSeconds.value : 0;
  return m * 60 + s;
});

function formatDuration(sec: number) {
  const m = Math.floor(sec / 60);
  const s = sec % 60;
  return s > 0 ? `${m}m ${s}s` : `${m}m`;
}

const currentExercise = computed(() => props.workout.exercises[currentExerciseIndex.value]);

const formattedTime = computed(() => {
  const m = Math.floor(secondsRemaining.value / 60);
  const s = secondsRemaining.value % 60;
  return `${m}:${s.toString().padStart(2, '0')}`;
});

const totalSecondsPassed = computed(() => {
  let passed = 0;
  for (let i = 0; i < currentExerciseIndex.value; i++) {
    passed += props.workout.exercises[i].time_limit_seconds;
  }
  passed += (currentExercise.value.time_limit_seconds - secondsRemaining.value);
  return passed;
});

const progressPercent = computed(() => {
  const total = 660; // 11 minutes total (120+60+60+60+360)
  return Math.min(100, Math.round((totalSecondsPassed.value / total) * 100));
});

function tick() {
  if (isPaused.value || isFinished.value) return;

  if (secondsRemaining.value > 0) {
    secondsRemaining.value--;

    // Sound cues on last 3 seconds
    if (secondsRemaining.value === 3 || secondsRemaining.value === 2) {
      playCountdownBeep(false);
    } else if (secondsRemaining.value === 1) {
      playCountdownBeep(true);
    }
  } else {
    // Move to next exercise
    if (currentExerciseIndex.value < props.workout.exercises.length - 1) {
      playTransitionChime();
      currentExerciseIndex.value++;
      secondsRemaining.value = props.workout.exercises[currentExerciseIndex.value].time_limit_seconds;
    } else {
      // Completed all 5 exercises
      playCelebrationChime();
      isFinished.value = true;
      clearInterval(timerInterval);
    }
  }
}

function togglePause() {
  isPaused.value = !isPaused.value;
}

function nextExercise() {
  if (currentExerciseIndex.value < props.workout.exercises.length - 1) {
    playTransitionChime();
    currentExerciseIndex.value++;
    secondsRemaining.value = props.workout.exercises[currentExerciseIndex.value].time_limit_seconds;
  } else {
    finishWorkout();
  }
}

function finishWorkout() {
  playCelebrationChime();
  isFinished.value = true;
  clearInterval(timerInterval);
}

function submitResults() {
  const finalDuration = cardioMode.value === 'stationary' ? 0 : totalSecondsEntered.value;
  emit('submit', {
    reps_1: reps1.value,
    reps_2: reps2.value,
    reps_3: reps3.value,
    reps_4: reps4.value,
    reps_5: reps5.value,
    cardio_mode: cardioMode.value,
    cardio_duration_secs: finalDuration,
    notes: notes.value,
  });
}

onMounted(() => {
  timerInterval = setInterval(tick, 1000);
});

onUnmounted(() => {
  if (timerInterval) clearInterval(timerInterval);
});
</script>

<template>
  <div class="fixed inset-0 z-50 bg-slate-950/90 backdrop-blur-md flex items-center justify-center p-4">
    <!-- Active Timer Cockpit Screen -->
    <div v-if="!isFinished" class="bg-slate-900 border border-slate-700/80 rounded-3xl w-full max-w-xl overflow-hidden shadow-2xl flex flex-col relative">
      <!-- Glow ambient accent -->
      <div class="absolute -top-20 left-1/2 -translate-x-1/2 w-64 h-32 bg-cyan-500/15 blur-2xl pointer-events-none"></div>

      <!-- Top Bar -->
      <div class="bg-slate-950/80 p-4 border-b border-slate-800 flex justify-between items-center relative z-10">
        <div class="flex items-center gap-2">
          <span class="w-2 h-2 rounded-full bg-cyan-400 animate-pulse"></span>
          <span class="text-xs font-bold uppercase tracking-wider text-cyan-400">
            Movement {{ currentExerciseIndex + 1 }} of 5
          </span>
          <span class="text-slate-600">•</span>
          <span class="text-xs font-bold text-slate-300">{{ currentExercise.name }}</span>
        </div>
        <button
          @click="emit('close')"
          class="w-8 h-8 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-white flex items-center justify-center text-xs transition-colors cursor-pointer"
        >
          ✕
        </button>
      </div>

      <!-- Main Body -->
      <div class="p-6 sm:p-8 flex flex-col items-center text-center relative z-10">
        <!-- Exercise Illustration -->
        <div class="w-48 h-32 bg-white rounded-2xl p-2.5 flex items-center justify-center shadow-inner mb-6 border border-slate-300">
          <img :src="'/images/' + currentExercise.image_path" :alt="currentExercise.name" class="max-h-full max-w-full object-contain" />
        </div>

        <!-- Big Countdown Timer -->
        <div class="text-7xl font-black font-mono tracking-tight text-white mb-2 tabular-nums">
          {{ formattedTime }}
        </div>

        <!-- Target Info -->
        <div class="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full bg-slate-950 border border-slate-800 text-sm font-bold text-slate-300 mb-5">
          <span>Cadence Target:</span>
          <span class="text-emerald-400 font-mono">{{ currentExercise.target_reps }} {{ currentExercise.is_cardio ? 'steps' : 'reps' }}</span>
        </div>

        <!-- Technique instructions -->
        <div class="w-full bg-slate-950/80 p-3.5 rounded-xl border border-slate-800 text-xs text-slate-300 text-left mb-6 max-h-24 overflow-y-auto">
          <span class="text-[10px] font-bold text-slate-500 uppercase tracking-wider block mb-1">Posture Cue</span>
          {{ currentExercise.instructions }}
        </div>

        <!-- Progress bar -->
        <div class="w-full mb-6">
          <div class="flex justify-between text-[11px] font-mono text-slate-400 mb-1.5">
            <span>Overall Mission Cadence</span>
            <span class="text-cyan-400 font-bold">{{ progressPercent }}% ({{ Math.floor(totalSecondsPassed / 60) }}m {{ totalSecondsPassed % 60 }}s / 11m)</span>
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
            :class="isPaused ? 'bg-emerald-500 hover:bg-emerald-400 text-slate-950' : 'bg-slate-800 hover:bg-slate-700 text-amber-400 border border-slate-700'"
          >
            {{ isPaused ? '▶ Resume' : '⏸ Pause' }}
          </button>
          <button
            @click="nextExercise"
            class="flex-1 py-3.5 rounded-xl font-black text-sm uppercase tracking-wider bg-gradient-to-r from-cyan-500 to-blue-600 hover:from-cyan-400 hover:to-blue-500 text-slate-950 shadow-md shadow-cyan-500/20 transition-all cursor-pointer"
          >
            {{ currentExerciseIndex < 4 ? 'Next Movement →' : 'Finish Workout 🎖️' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Finish / Rep Entry Screen -->
    <div v-else class="bg-slate-900 border border-slate-700/80 rounded-3xl w-full max-w-lg overflow-hidden shadow-2xl flex flex-col p-6 sm:p-8 max-h-[90vh] overflow-y-auto relative">
      <div class="text-center mb-6">
        <div class="w-14 h-14 rounded-2xl bg-emerald-500/20 border border-emerald-500/40 text-emerald-400 flex items-center justify-center text-3xl mx-auto mb-3">
          🎖️
        </div>
        <h2 class="text-2xl font-black text-white uppercase tracking-tight">11 Minutes Complete!</h2>
        <p class="text-xs text-slate-300 mt-1">
          Record your actual completed reps to calculate your official RCAF verdict and ladder progression.
        </p>
      </div>

      <div class="space-y-4">
        <!-- Ex 1-4 Inputs -->
        <div class="grid grid-cols-2 gap-3">
          <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800">
            <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block mb-1">Ex 1: Bends</label>
            <div class="flex items-center gap-2">
              <input v-model.number="reps1" type="number" class="w-full bg-slate-900 text-white font-black text-xl p-2 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none" />
              <span class="text-xs text-slate-500 font-mono">/ {{ workout.exercises[0].target_reps }}</span>
            </div>
          </div>

          <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800">
            <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block mb-1">Ex 2: Sit-Ups</label>
            <div class="flex items-center gap-2">
              <input v-model.number="reps2" type="number" class="w-full bg-slate-900 text-white font-black text-xl p-2 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none" />
              <span class="text-xs text-slate-500 font-mono">/ {{ workout.exercises[1].target_reps }}</span>
            </div>
          </div>

          <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800">
            <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block mb-1">Ex 3: Back Arches</label>
            <div class="flex items-center gap-2">
              <input v-model.number="reps3" type="number" class="w-full bg-slate-900 text-white font-black text-xl p-2 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none" />
              <span class="text-xs text-slate-500 font-mono">/ {{ workout.exercises[2].target_reps }}</span>
            </div>
          </div>

          <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800">
            <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block mb-1">Ex 4: Push-Ups</label>
            <div class="flex items-center gap-2">
              <input v-model.number="reps4" type="number" class="w-full bg-slate-900 text-white font-black text-xl p-2 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none" />
              <span class="text-xs text-slate-500 font-mono">/ {{ workout.exercises[3].target_reps }}</span>
            </div>
          </div>
        </div>

        <!-- Ex 5 Cardio -->
        <div class="bg-slate-950/80 p-4 rounded-2xl border border-slate-800">
          <div class="flex items-center justify-between mb-2">
            <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block">
              Ex 5: Cardio Track Discipline
            </label>
            <span class="text-[10px] font-mono text-cyan-400">
              {{ cardioMode === 'stationary' ? 'Indoor 6-Min' : (cardioMode === 'run' ? '1-Mile Run' : '2-Mile Walk') }}
            </span>
          </div>

          <div class="grid grid-cols-3 gap-1.5 mb-3">
            <button
              v-for="mode in ['stationary', 'run', 'walk']"
              :key="mode"
              type="button"
              @click="cardioMode = mode as any"
              class="py-2 px-2 text-xs font-bold rounded-xl uppercase tracking-wider transition-all cursor-pointer flex flex-col items-center gap-0.5"
              :class="cardioMode === mode ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'bg-slate-900 text-slate-400 hover:text-white border border-slate-800'"
            >
              <span>{{ mode === 'stationary' ? '👟' : (mode === 'run' ? '🏃' : '🚶') }}</span>
              <span class="text-[11px]">{{ mode === 'stationary' ? 'Stationary' : (mode === 'run' ? '1-Mile Run' : '2-Mile Walk') }}</span>
            </button>
          </div>

          <!-- Target Banner for Ex 5 -->
          <div class="bg-slate-900/90 p-2.5 rounded-xl border border-slate-800 mb-3 text-xs flex justify-between items-center">
            <span class="text-slate-400 text-[11px]">Required Standard:</span>
            <span class="font-mono font-bold text-cyan-300">
              <template v-if="cardioMode === 'stationary'">
                {{ workout.exercises[4]?.target_reps }} steps (+ 10 scissor jumps / 75 steps)
              </template>
              <template v-else-if="cardioMode === 'run'">
                Under {{ formatDuration(workout.exercises[4]?.alt_run_time_seconds) }} ({{ workout.exercises[4]?.alt_run_time_seconds }}s)
              </template>
              <template v-else>
                Under {{ formatDuration(workout.exercises[4]?.alt_walk_time_seconds) }} ({{ workout.exercises[4]?.alt_walk_time_seconds }}s)
              </template>
            </span>
          </div>

          <!-- Stationary Input -->
          <div v-if="cardioMode === 'stationary'" class="flex items-center gap-2">
            <input v-model.number="reps5" type="number" required min="0" class="w-full bg-slate-900 text-white font-black text-xl p-2.5 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none" />
            <span class="text-xs text-slate-400 shrink-0">steps completed</span>
          </div>

          <!-- Run / Walk Minutes & Seconds Input -->
          <div v-else class="space-y-2">
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="text-[10px] text-slate-400 block mb-1">Minutes</label>
                <input
                  v-model.number="cardioMinutes"
                  type="number"
                  required
                  min="0"
                  max="120"
                  placeholder="0"
                  class="w-full bg-slate-900 text-white font-black text-xl p-2.5 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none"
                />
              </div>
              <div>
                <label class="text-[10px] text-slate-400 block mb-1">Seconds</label>
                <input
                  v-model.number="cardioSeconds"
                  type="number"
                  required
                  min="0"
                  max="59"
                  placeholder="0"
                  class="w-full bg-slate-900 text-white font-black text-xl p-2.5 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none"
                />
              </div>
            </div>

            <!-- Live Validation Indicator -->
            <div v-if="totalSecondsEntered > 0" class="text-xs font-mono p-2 rounded-lg" :class="totalSecondsEntered <= currentCardioTarget ? 'bg-emerald-950/60 text-emerald-300 border border-emerald-500/30' : 'bg-amber-950/60 text-amber-300 border border-amber-500/30'">
              <span v-if="totalSecondsEntered <= currentCardioTarget">
                ✓ Recorded time: {{ totalSecondsEntered }}s — Standard met!
              </span>
              <span v-else>
                ⚠️ Recorded time: {{ totalSecondsEntered }}s — Exceeds {{ currentCardioTarget }}s standard.
              </span>
            </div>
          </div>
        </div>

        <!-- Session Notes -->
        <div>
          <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block mb-1">Session Debrief Notes (Optional)</label>
          <input v-model="notes" type="text" placeholder="Felt strong, slight soreness, good cadence..." class="w-full bg-slate-950 text-white p-3 rounded-xl border border-slate-800 text-xs focus:border-cyan-500 focus:outline-none" />
        </div>

        <!-- Actions -->
        <div class="flex gap-3 pt-2">
          <button @click="emit('close')" class="flex-1 py-3.5 rounded-xl bg-slate-800 text-slate-300 font-bold hover:bg-slate-700 text-xs uppercase tracking-wider transition-colors cursor-pointer">
            Dismiss
          </button>
          <button
            @click="submitResults"
            class="flex-2 py-3.5 rounded-xl bg-gradient-to-r from-emerald-500 to-teal-600 hover:from-emerald-400 hover:to-teal-500 text-slate-950 font-black text-xs uppercase tracking-wider shadow-lg shadow-emerald-500/20 transition-all cursor-pointer"
          >
            {{ isGuest ? 'Finish Guest Session' : 'Submit & Calculate Verdict 🎖️' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
