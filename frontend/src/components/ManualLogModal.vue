<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { TodayWorkout, SubmitWorkoutPayload, ExerciseChartRow, WorkoutSessionHistory } from '../types';
import { fetchSystemCharts } from '../api';
import { getLastPerformance, getNextRungTarget, calculateTreadmillSpeed, formatDurationMmSs } from '../telemetry';

const props = defineProps<{
  workout: TodayWorkout;
  history?: WorkoutSessionHistory[];
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'submit', payload: SubmitWorkoutPayload): void;
}>();

const systemCharts = ref<ExerciseChartRow[]>([]);

onMounted(async () => {
  try {
    const data = await fetchSystemCharts();
    if (data?.charts) systemCharts.value = data.charts;
  } catch (e) {
    console.error('Failed to load system charts in ManualLogModal:', e);
  }
});

function getExPrevious(exNum: number) {
  return getLastPerformance(props.history || [], exNum, cardioMode.value);
}

function getExNextRung(exNum: number) {
  if (exNum < 5) {
    return getNextRungTarget(
      systemCharts.value,
      props.workout.strength_chart,
      props.workout.strength_level,
      exNum
    );
  }
  return getNextRungTarget(
    systemCharts.value,
    props.workout.cardio_chart,
    props.workout.cardio_level,
    5,
    cardioMode.value
  );
}

const reps1 = ref(props.workout.exercises[0]?.target_reps || 0);
const reps2 = ref(props.workout.exercises[1]?.target_reps || 0);
const reps3 = ref(props.workout.exercises[2]?.target_reps || 0);
const reps4 = ref(props.workout.exercises[3]?.target_reps || 0);
const reps5 = ref(props.workout.exercises[4]?.target_reps || 0);
const cardioMode = ref<'stationary' | 'run' | 'walk'>('stationary');
const cardioTimeString = ref('');
const notes = ref('');

function getLocalDateTimeString(date: Date = new Date()): string {
  const pad = (n: number) => n.toString().padStart(2, '0');
  const y = date.getFullYear();
  const m = pad(date.getMonth() + 1);
  const d = pad(date.getDate());
  const h = pad(date.getHours());
  const min = pad(date.getMinutes());
  return `${y}-${m}-${d}T${h}:${min}`;
}

const completedAtLocal = ref(getLocalDateTimeString());

function setToCurrentTime() {
  completedAtLocal.value = getLocalDateTimeString();
}

const isHistoricalDate = computed(() => {
  if (!completedAtLocal.value) return false;
  const chosen = new Date(completedAtLocal.value).getTime();
  const now = Date.now();
  return (now - chosen) > 30 * 60 * 1000;
});

const currentCardioTarget = computed(() => {
  const ex5 = props.workout.exercises[4];
  if (!ex5) return 0;
  if (cardioMode.value === 'run') return ex5.alt_run_time_seconds;
  if (cardioMode.value === 'walk') return ex5.alt_walk_time_seconds;
  return ex5.target_reps;
});

const cardioDistanceMiles = computed(() => {
  if (cardioMode.value === 'run') {
    return props.workout.cardio_chart === 1 ? 0.5 : 1.0;
  }
  if (cardioMode.value === 'walk') {
    return props.workout.cardio_chart === 1 ? 1.0 : 2.0;
  }
  return 0;
});

const runLabel = computed(() => {
  return props.workout.cardio_chart === 1 ? '0.5 mi (0.8 km)' : '1.0 mi (1.6 km)';
});

const walkLabel = computed(() => {
  return props.workout.cardio_chart === 1 ? '1.0 mi (1.6 km)' : '2.0 mi (3.2 km)';
});

const treadmillSpeed = computed(() => {
  const ex5 = props.workout.exercises.find((ex) => ex.exercise_number === 5);
  if (!ex5) return { mph: 0, kph: 0 };
  const targetSec = cardioMode.value === 'run' ? ex5.alt_run_time_seconds : ex5.alt_walk_time_seconds;
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

const totalSecondsEntered = computed(() => parseTimeStringToSeconds(cardioTimeString.value));

const achievedSpeed = computed(() => {
  const sec = totalSecondsEntered.value;
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

function submitLog() {
  const finalDuration = cardioMode.value === 'stationary' ? 0 : totalSecondsEntered.value;
  const isoCompletedAt = completedAtLocal.value ? new Date(completedAtLocal.value).toISOString() : undefined;
  emit('submit', {
    reps_1: reps1.value,
    reps_2: reps2.value,
    reps_3: reps3.value,
    reps_4: reps4.value,
    reps_5: reps5.value,
    cardio_mode: cardioMode.value,
    cardio_duration_secs: finalDuration,
    notes: notes.value,
    completed_at: isoCompletedAt,
  });
}
</script>

<template>
  <div class="fixed inset-0 z-50 bg-slate-950/85 backdrop-blur-md flex items-center justify-center p-4">
    <div class="bg-slate-900 border border-slate-700/80 rounded-3xl w-full max-w-lg p-6 sm:p-8 shadow-2xl relative overflow-hidden max-h-[90vh] overflow-y-auto">
      <!-- Glow ambient accent -->
      <div class="absolute -top-20 -right-20 w-48 h-48 bg-emerald-500/10 blur-2xl pointer-events-none"></div>

      <!-- Header -->
      <div class="flex justify-between items-center pb-4 border-b border-slate-800 mb-5">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-xl bg-emerald-500/20 border border-emerald-500/40 text-emerald-400 flex items-center justify-center text-lg font-black">
            ✍️
          </div>
          <div>
            <h2 class="text-xl font-black text-white uppercase tracking-tight">Log Offline Scorecard</h2>
            <p class="text-xs text-slate-400">Record reps completed from your pencil-and-paper workout sheet</p>
          </div>
        </div>
        <button
          @click="emit('close')"
          class="w-8 h-8 rounded-full bg-slate-800 text-slate-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
        >
          ✕
        </button>
      </div>

      <!-- Sortie Date & Time Configuration -->
      <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800 mb-5 text-xs">
        <div class="flex items-center justify-between mb-2">
          <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider flex items-center gap-1.5">
            <span>📅</span>
            <span>Sortie Date & Time</span>
          </label>
          <button
            type="button"
            @click="setToCurrentTime"
            class="text-[10px] font-mono font-bold text-emerald-400 hover:text-emerald-300 bg-emerald-950/60 border border-emerald-500/30 hover:border-emerald-400/50 px-2.5 py-1 rounded-lg transition-colors cursor-pointer flex items-center gap-1"
            title="Reset to current moment"
          >
            <span>⏱️</span>
            <span>Now</span>
          </button>
        </div>
        <div class="relative">
          <input
            v-model="completedAtLocal"
            type="datetime-local"
            class="w-full bg-slate-900 text-cyan-300 font-mono text-sm px-3.5 py-2.5 rounded-xl border border-slate-700 focus:border-cyan-500 focus:outline-none [color-scheme:dark]"
          />
        </div>
        <div class="flex items-center justify-between mt-2 pt-1.5 border-t border-slate-900 text-[10px]">
          <span class="text-slate-400">Assigned Standards: <strong class="text-slate-200">{{ workout.strength_display }} / {{ workout.cardio_display }}</strong></span>
          <span v-if="isHistoricalDate" class="text-amber-400 font-mono font-semibold flex items-center gap-1">
            <span>⚠️</span> Backdated Sortie
          </span>
          <span v-else class="text-emerald-400/80 font-mono">Present Sortie</span>
        </div>
      </div>

      <form @submit.prevent="submitLog" class="space-y-4">
        <!-- 4 Strength Exercises -->
        <div class="grid grid-cols-2 gap-3">
          <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800 flex flex-col justify-between">
            <div>
              <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
                Ex 1: Forward Bends
              </label>
              <div class="flex items-center gap-2">
                <input
                  v-model.number="reps1"
                  type="number"
                  required
                  min="0"
                  class="w-full bg-slate-900 text-white font-black text-xl p-2 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none"
                />
                <span class="text-xs text-slate-500 font-mono">/ {{ workout.exercises[0]?.target_reps }}</span>
              </div>
            </div>
            <div class="flex flex-wrap items-center justify-between text-[10px] font-mono mt-2 pt-1 border-t border-slate-900 gap-1">
              <span v-if="getExPrevious(1)" class="text-amber-400">
                Last: {{ getExPrevious(1)?.display }}
              </span>
              <span v-else class="text-slate-600">First sortie</span>
              <span v-if="getExNextRung(1)" class="text-cyan-400">
                Next: {{ getExNextRung(1)?.display }}
              </span>
            </div>
          </div>

          <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800 flex flex-col justify-between">
            <div>
              <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
                Ex 2: Sit-Ups
              </label>
              <div class="flex items-center gap-2">
                <input
                  v-model.number="reps2"
                  type="number"
                  required
                  min="0"
                  class="w-full bg-slate-900 text-white font-black text-xl p-2 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none"
                />
                <span class="text-xs text-slate-500 font-mono">/ {{ workout.exercises[1]?.target_reps }}</span>
              </div>
            </div>
            <div class="flex flex-wrap items-center justify-between text-[10px] font-mono mt-2 pt-1 border-t border-slate-900 gap-1">
              <span v-if="getExPrevious(2)" class="text-amber-400">
                Last: {{ getExPrevious(2)?.display }}
              </span>
              <span v-else class="text-slate-600">First sortie</span>
              <span v-if="getExNextRung(2)" class="text-cyan-400">
                Next: {{ getExNextRung(2)?.display }}
              </span>
            </div>
          </div>

          <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800 flex flex-col justify-between">
            <div>
              <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
                Ex 3: Back Arches
              </label>
              <div class="flex items-center gap-2">
                <input
                  v-model.number="reps3"
                  type="number"
                  required
                  min="0"
                  class="w-full bg-slate-900 text-white font-black text-xl p-2 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none"
                />
                <span class="text-xs text-slate-500 font-mono">/ {{ workout.exercises[2]?.target_reps }}</span>
              </div>
            </div>
            <div class="flex flex-wrap items-center justify-between text-[10px] font-mono mt-2 pt-1 border-t border-slate-900 gap-1">
              <span v-if="getExPrevious(3)" class="text-amber-400">
                Last: {{ getExPrevious(3)?.display }}
              </span>
              <span v-else class="text-slate-600">First sortie</span>
              <span v-if="getExNextRung(3)" class="text-cyan-400">
                Next: {{ getExNextRung(3)?.display }}
              </span>
            </div>
          </div>

          <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800 flex flex-col justify-between">
            <div>
              <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
                Ex 4: Push-Ups
              </label>
              <div class="flex items-center gap-2">
                <input
                  v-model.number="reps4"
                  type="number"
                  required
                  min="0"
                  class="w-full bg-slate-900 text-white font-black text-xl p-2 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none"
                />
                <span class="text-xs text-slate-500 font-mono">/ {{ workout.exercises[3]?.target_reps }}</span>
              </div>
            </div>
            <div class="flex flex-wrap items-center justify-between text-[10px] font-mono mt-2 pt-1 border-t border-slate-900 gap-1">
              <span v-if="getExPrevious(4)" class="text-amber-400">
                Last: {{ getExPrevious(4)?.display }}
              </span>
              <span v-else class="text-slate-600">First sortie</span>
              <span v-if="getExNextRung(4)" class="text-cyan-400">
                Next: {{ getExNextRung(4)?.display }}
              </span>
            </div>
          </div>
        </div>

        <!-- Exercise 5 Cardio -->
        <div class="bg-slate-950/80 p-4 rounded-2xl border border-slate-800">
          <div class="flex items-center justify-between mb-2">
            <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block">
              Ex 5: Cardio Track Discipline
            </label>
            <span class="text-[10px] font-mono text-cyan-400">
              {{ cardioMode === 'stationary' ? 'Indoor 6-Min' : (cardioMode === 'run' ? `${runLabel} Run` : `${walkLabel} Walk`) }}
            </span>
          </div>

          <div class="grid grid-cols-3 gap-1.5 mb-3">
            <button
              type="button"
              @click="cardioMode = 'stationary'"
              class="py-2 px-2 text-xs font-bold rounded-xl uppercase tracking-wider transition-all cursor-pointer flex flex-col items-center gap-0.5"
              :class="cardioMode === 'stationary' ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'bg-slate-900 text-slate-400 hover:text-white border border-slate-800'"
            >
              <span>👟</span>
              <span class="text-[11px]">Stationary</span>
              <span class="text-[9px] opacity-75">Run</span>
            </button>
            <button
              type="button"
              @click="cardioMode = 'run'"
              class="py-2 px-2 text-xs font-bold rounded-xl uppercase tracking-wider transition-all cursor-pointer flex flex-col items-center gap-0.5"
              :class="cardioMode === 'run' ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'bg-slate-900 text-slate-400 hover:text-white border border-slate-800'"
            >
              <span>🏃</span>
              <span class="text-[11px]">{{ runLabel }}</span>
              <span class="text-[9px] opacity-75">Run</span>
            </button>
            <button
              type="button"
              @click="cardioMode = 'walk'"
              class="py-2 px-2 text-xs font-bold rounded-xl uppercase tracking-wider transition-all cursor-pointer flex flex-col items-center gap-0.5"
              :class="cardioMode === 'walk' ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'bg-slate-900 text-slate-400 hover:text-white border border-slate-800'"
            >
              <span>🚶</span>
              <span class="text-[11px]">{{ walkLabel }}</span>
              <span class="text-[9px] opacity-75">Walk</span>
            </button>
          </div>

          <!-- Target Banner for Ex 5 -->
          <div class="bg-slate-900/90 p-2.5 rounded-xl border border-slate-800 mb-3 text-xs flex flex-col sm:flex-row sm:justify-between sm:items-center gap-1">
            <div class="flex items-center gap-2">
              <span class="text-slate-400 text-[11px]">Required Standard:</span>
              <span class="font-mono font-bold text-cyan-300">
                <template v-if="cardioMode === 'stationary'">
                  {{ workout.exercises[4]?.target_reps }} steps (+ 10 scissor jumps / 75 steps)
                </template>
                <template v-else-if="cardioMode === 'run'">
                  Under {{ formatDuration(workout.exercises[4]?.alt_run_time_seconds) }} ({{ runLabel }})
                </template>
                <template v-else>
                  Under {{ formatDuration(workout.exercises[4]?.alt_walk_time_seconds) }} ({{ walkLabel }})
                </template>
              </span>
            </div>
            <div v-if="cardioMode !== 'stationary' && treadmillSpeed.mph > 0" class="text-[10px] font-mono text-slate-400">
              Speed: <span class="text-cyan-300 font-bold">≥ {{ treadmillSpeed.kph.toFixed(1) }} km/h</span> (<span class="text-cyan-300 font-bold">{{ treadmillSpeed.mph.toFixed(1) }} mph</span>)
            </div>
          </div>

          <!-- Stationary Input -->
          <div v-if="cardioMode === 'stationary'" class="flex items-center gap-2">
            <input
              v-model.number="reps5"
              type="number"
              required
              min="0"
              class="w-full bg-slate-900 text-white font-black text-xl p-2.5 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none"
            />
            <span class="text-xs text-slate-400 shrink-0">steps completed</span>
          </div>

          <!-- Run / Walk mm:ss Time Input -->
          <div v-else class="space-y-2">
            <div>
              <label class="text-[10px] text-slate-400 font-bold uppercase tracking-wider block mb-1">
                Duration Taken (mm:ss)
              </label>
              <div class="relative">
                <input
                  v-model="cardioTimeString"
                  type="text"
                  required
                  placeholder="e.g. 07:30"
                  class="w-full bg-slate-900 text-cyan-300 font-mono font-black text-2xl p-3 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none tracking-widest placeholder:text-slate-600"
                />
                <span class="absolute right-3.5 top-1/2 -translate-y-1/2 text-[11px] font-mono text-slate-500">
                  mm:ss
                </span>
              </div>
              <span class="text-[10px] text-slate-500 mt-1 block text-center">
                Enter your total running or walking time (e.g. 07:30 or 7:30)
              </span>
            </div>

            <!-- Live Validation & Treadmill Speed Indicator -->
            <div
              v-if="totalSecondsEntered > 0"
              class="text-xs font-mono p-3 rounded-xl border transition-all"
              :class="totalSecondsEntered <= currentCardioTarget ? 'bg-emerald-950/70 text-emerald-300 border-emerald-500/40' : 'bg-amber-950/70 text-amber-300 border-amber-500/40'"
            >
              <div class="flex items-center justify-between font-bold">
                <span>
                  {{ totalSecondsEntered <= currentCardioTarget ? '✓ Standard Met!' : '⚠️ Standard Exceeded' }}
                </span>
                <span>{{ Math.floor(totalSecondsEntered / 60) }}m {{ totalSecondsEntered % 60 }}s ({{ totalSecondsEntered }}s)</span>
              </div>
              <div class="text-[11px] text-slate-300 mt-1.5 flex items-center justify-between">
                <span>Equivalent Pace:</span>
                <span class="font-bold text-white">
                  {{ achievedSpeed.kph.toFixed(1) }} km/h ({{ achievedSpeed.mph.toFixed(1) }} mph)
                </span>
              </div>
            </div>
          </div>

          <!-- Telemetry for Ex 5 -->
          <div class="flex flex-wrap items-center justify-between text-[10px] font-mono mt-3 pt-2 border-t border-slate-900 gap-1">
            <span v-if="getExPrevious(5)" class="text-amber-400">
              Previous Sortie: {{ getExPrevious(5)?.display }}
            </span>
            <span v-else class="text-slate-600">First sortie</span>
            <span v-if="getExNextRung(5)" class="text-cyan-400">
              Next Rung: {{ getExNextRung(5)?.display }}
            </span>
          </div>
        </div>

        <!-- Debrief Notes -->
        <div>
          <label class="text-[10px] font-bold text-slate-400 uppercase tracking-wider block mb-1">
            Flight Debrief Notes (Optional)
          </label>
          <input
            v-model="notes"
            type="text"
            placeholder="e.g. Completed during gym trip, felt strong, good cadence"
            class="w-full bg-slate-950 text-white p-3 rounded-xl border border-slate-800 text-xs focus:border-cyan-500 focus:outline-none"
          />
        </div>

        <!-- Standardised Buttons -->
        <div class="flex gap-3 pt-2">
          <button
            type="button"
            @click="emit('close')"
            class="btn-control-secondary flex-1 bg-slate-800 hover:bg-slate-700 text-slate-300"
          >
            Cancel
          </button>
          <button
            type="submit"
            class="btn-control-primary flex-2 bg-gradient-to-r from-emerald-500 to-teal-600 hover:from-emerald-400 hover:to-teal-500 text-slate-950 shadow-lg shadow-emerald-500/20"
          >
            Submit & Calculate Verdict 🎖️
          </button>
        </div>
      </form>
    </div>
  </div>
</template>
