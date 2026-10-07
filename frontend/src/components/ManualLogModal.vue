<script setup lang="ts">
import { ref, computed } from 'vue';
import { TodayWorkout, SubmitWorkoutPayload } from '../types';

const props = defineProps<{
  workout: TodayWorkout;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'submit', payload: SubmitWorkoutPayload): void;
}>();

const reps1 = ref(props.workout.exercises[0]?.target_reps || 0);
const reps2 = ref(props.workout.exercises[1]?.target_reps || 0);
const reps3 = ref(props.workout.exercises[2]?.target_reps || 0);
const reps4 = ref(props.workout.exercises[3]?.target_reps || 0);
const reps5 = ref(props.workout.exercises[4]?.target_reps || 0);
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

function submitLog() {
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

      <!-- Current Target Standards Banner -->
      <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800 flex items-center justify-between mb-5 text-xs">
        <div>
          <span class="text-[10px] text-slate-400 uppercase font-bold block">Assigned Mission Targets</span>
          <span class="text-white font-bold">{{ workout.strength_display }} / {{ workout.cardio_display }}</span>
        </div>
        <span class="text-emerald-400 font-mono text-[11px] bg-emerald-950/60 border border-emerald-500/30 px-2.5 py-1 rounded-full">
          Date: {{ workout.date }}
        </span>
      </div>

      <form @submit.prevent="submitLog" class="space-y-4">
        <!-- 4 Strength Exercises -->
        <div class="grid grid-cols-2 gap-3">
          <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800">
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

          <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800">
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

          <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800">
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

          <div class="bg-slate-950/80 p-3.5 rounded-2xl border border-slate-800">
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
        </div>

        <!-- Exercise 5 Cardio -->
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
              type="button"
              @click="cardioMode = 'stationary'"
              class="py-2 px-2 text-xs font-bold rounded-xl uppercase tracking-wider transition-all cursor-pointer flex flex-col items-center gap-0.5"
              :class="cardioMode === 'stationary' ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'bg-slate-900 text-slate-400 hover:text-white border border-slate-800'"
            >
              <span>👟</span>
              <span class="text-[11px]">Stationary</span>
            </button>
            <button
              type="button"
              @click="cardioMode = 'run'"
              class="py-2 px-2 text-xs font-bold rounded-xl uppercase tracking-wider transition-all cursor-pointer flex flex-col items-center gap-0.5"
              :class="cardioMode === 'run' ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'bg-slate-900 text-slate-400 hover:text-white border border-slate-800'"
            >
              <span>🏃</span>
              <span class="text-[11px]">1-Mile Run</span>
            </button>
            <button
              type="button"
              @click="cardioMode = 'walk'"
              class="py-2 px-2 text-xs font-bold rounded-xl uppercase tracking-wider transition-all cursor-pointer flex flex-col items-center gap-0.5"
              :class="cardioMode === 'walk' ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'bg-slate-900 text-slate-400 hover:text-white border border-slate-800'"
            >
              <span>🚶</span>
              <span class="text-[11px]">2-Mile Walk</span>
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
            <input
              v-model.number="reps5"
              type="number"
              required
              min="0"
              class="w-full bg-slate-900 text-white font-black text-xl p-2.5 rounded-xl border border-slate-700 text-center focus:border-cyan-500 focus:outline-none"
            />
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
