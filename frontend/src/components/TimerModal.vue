<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { TodayWorkout, SubmitWorkoutPayload } from '../types';
import { playCountdownBeep, playTransitionChime, playCelebrationChime } from '../audio';

const props = defineProps<{
  workout: TodayWorkout;
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
const cardioDuration = ref(0);
const notes = ref('');

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
      // Finished all 5 exercises!
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
  isFinished.value = true;
  clearInterval(timerInterval);
}

function submitResults() {
  emit('submit', {
    reps_1: reps1.value,
    reps_2: reps2.value,
    reps_3: reps3.value,
    reps_4: reps4.value,
    reps_5: reps5.value,
    cardio_mode: cardioMode.value,
    cardio_duration_secs: cardioDuration.value,
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
  <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4">
    <!-- Active Timer Screen -->
    <div v-if="!isFinished" class="bg-slate-800 border border-slate-700 rounded-2xl w-full max-w-xl overflow-hidden shadow-2xl flex flex-col">
      <!-- Top Bar -->
      <div class="bg-slate-900/60 p-4 border-b border-slate-700 flex justify-between items-center">
        <div>
          <span class="text-xs font-semibold uppercase tracking-wider text-blue-400">Exercise {{ currentExerciseIndex + 1 }} of 5</span>
          <h2 class="text-xl font-bold text-white">{{ currentExercise.name }}</h2>
        </div>
        <button @click="emit('close')" class="text-slate-400 hover:text-white p-2">✕</button>
      </div>

      <!-- Main Body -->
      <div class="p-6 flex flex-col items-center text-center">
        <!-- Exercise Illustration -->
        <div class="w-48 h-32 bg-white rounded-xl p-2 flex items-center justify-center shadow-inner mb-4">
          <img :src="'/images/' + currentExercise.image_path" :alt="currentExercise.name" class="max-h-full max-w-full object-contain" />
        </div>

        <!-- Big Countdown Timer -->
        <div class="text-6xl font-black font-mono tracking-tight text-white mb-2">
          {{ formattedTime }}
        </div>

        <!-- Target Info -->
        <div class="text-base text-slate-300 font-medium mb-4">
          Target: <span class="text-emerald-400 font-bold">{{ currentExercise.target_reps }} {{ currentExercise.is_cardio ? 'steps' : 'reps' }}</span>
        </div>

        <!-- Technique instructions -->
        <p class="text-sm text-slate-400 max-w-md line-clamp-3 mb-6 bg-slate-900/40 p-3 rounded-lg border border-slate-700/50">
          {{ currentExercise.instructions }}
        </p>

        <!-- Progress bar -->
        <div class="w-full bg-slate-700 rounded-full h-2.5 mb-6 overflow-hidden">
          <div class="bg-blue-500 h-2.5 rounded-full transition-all duration-300" :style="{ width: progressPercent + '%' }"></div>
        </div>

        <!-- Controls -->
        <div class="flex gap-4 w-full">
          <button
            @click="togglePause"
            class="flex-1 py-3 px-4 rounded-xl font-bold text-white shadow transition-colors"
            :class="isPaused ? 'bg-emerald-600 hover:bg-emerald-500' : 'bg-amber-600 hover:bg-amber-500'"
          >
            {{ isPaused ? 'Resume' : 'Pause' }}
          </button>
          <button
            @click="nextExercise"
            class="flex-1 py-3 px-4 rounded-xl font-bold bg-blue-600 hover:bg-blue-500 text-white shadow transition-colors"
          >
            {{ currentExerciseIndex < 4 ? 'Next Movement →' : 'Finish Session' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Finish / Rep Entry Screen -->
    <div v-else class="bg-slate-800 border border-slate-700 rounded-2xl w-full max-w-lg overflow-hidden shadow-2xl flex flex-col p-6 max-h-[90vh] overflow-y-auto">
      <div class="text-center mb-4">
        <span class="text-4xl">🎉</span>
        <h2 class="text-2xl font-bold text-white mt-1">11 Minutes Complete!</h2>
        <p class="text-sm text-slate-400">Record your actual performance to calculate your progression.</p>
      </div>

      <div class="space-y-4">
        <!-- Ex 1-4 Inputs -->
        <div class="grid grid-cols-2 gap-3">
          <div class="bg-slate-900/70 p-3 rounded-xl border border-slate-700">
            <label class="text-xs text-slate-400 font-semibold block mb-1">Ex 1: Flexibility</label>
            <div class="flex items-center gap-2">
              <input v-model.number="reps1" type="number" class="w-full bg-slate-800 text-white font-bold text-lg p-2 rounded border border-slate-600 text-center" />
              <span class="text-xs text-slate-400">/ {{ workout.exercises[0].target_reps }}</span>
            </div>
          </div>

          <div class="bg-slate-900/70 p-3 rounded-xl border border-slate-700">
            <label class="text-xs text-slate-400 font-semibold block mb-1">Ex 2: Sit-ups</label>
            <div class="flex items-center gap-2">
              <input v-model.number="reps2" type="number" class="w-full bg-slate-800 text-white font-bold text-lg p-2 rounded border border-slate-600 text-center" />
              <span class="text-xs text-slate-400">/ {{ workout.exercises[1].target_reps }}</span>
            </div>
          </div>

          <div class="bg-slate-900/70 p-3 rounded-xl border border-slate-700">
            <label class="text-xs text-slate-400 font-semibold block mb-1">Ex 3: Back Arch</label>
            <div class="flex items-center gap-2">
              <input v-model.number="reps3" type="number" class="w-full bg-slate-800 text-white font-bold text-lg p-2 rounded border border-slate-600 text-center" />
              <span class="text-xs text-slate-400">/ {{ workout.exercises[2].target_reps }}</span>
            </div>
          </div>

          <div class="bg-slate-900/70 p-3 rounded-xl border border-slate-700">
            <label class="text-xs text-slate-400 font-semibold block mb-1">Ex 4: Push-ups</label>
            <div class="flex items-center gap-2">
              <input v-model.number="reps4" type="number" class="w-full bg-slate-800 text-white font-bold text-lg p-2 rounded border border-slate-600 text-center" />
              <span class="text-xs text-slate-400">/ {{ workout.exercises[3].target_reps }}</span>
            </div>
          </div>
        </div>

        <!-- Ex 5 Cardio -->
        <div class="bg-slate-900/70 p-3 rounded-xl border border-slate-700">
          <label class="text-xs text-slate-400 font-semibold block mb-1">Ex 5: Cardio Track</label>
          <div class="flex gap-2 mb-2">
            <button
              v-for="mode in ['stationary', 'run', 'walk']"
              :key="mode"
              type="button"
              @click="cardioMode = mode as any"
              class="flex-1 py-1.5 px-2 text-xs font-semibold rounded capitalize"
              :class="cardioMode === mode ? 'bg-blue-600 text-white' : 'bg-slate-800 text-slate-400'"
            >
              {{ mode }}
            </button>
          </div>

          <div v-if="cardioMode === 'stationary'" class="flex items-center gap-2">
            <input v-model.number="reps5" type="number" class="w-full bg-slate-800 text-white font-bold text-lg p-2 rounded border border-slate-600 text-center" />
            <span class="text-xs text-slate-400">steps (Target {{ workout.exercises[4].target_reps }})</span>
          </div>
          <div v-else class="flex items-center gap-2">
            <input v-model.number="cardioDuration" type="number" placeholder="Seconds" class="w-full bg-slate-800 text-white font-bold text-lg p-2 rounded border border-slate-600 text-center" />
            <span class="text-xs text-slate-400">Total Seconds Taken</span>
          </div>
        </div>

        <!-- Notes -->
        <div>
          <label class="text-xs text-slate-400 font-semibold block mb-1">Session Notes (Optional)</label>
          <input v-model="notes" type="text" placeholder="Felt strong, slight soreness, etc." class="w-full bg-slate-900 text-white p-2 rounded-lg border border-slate-700 text-sm" />
        </div>

        <!-- Actions -->
        <div class="flex gap-3 pt-2">
          <button @click="emit('close')" class="flex-1 py-3 rounded-xl bg-slate-700 text-slate-300 font-semibold hover:bg-slate-600">
            Cancel
          </button>
          <button @click="submitResults" class="flex-1 py-3 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white font-bold shadow-lg">
            Submit & Grade Workout
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
