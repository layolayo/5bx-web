<script setup lang="ts">
import { ref } from 'vue';
import { TodayWorkout, UserProfile, EarnedBadge } from '../types';

const props = defineProps<{
  workout: TodayWorkout;
  profile: UserProfile | null;
  highestBadge?: EarnedBadge | null;
}>();

const emit = defineEmits<{
  (e: 'start-timer'): void;
  (e: 'open-sheet'): void;
  (e: 'log-manual'): void;
  (e: 'toggle-kiss'): void;
}>();

const expandedMovement = ref<number | null>(null);
const cardioChoice = ref<'stationary' | 'run' | 'walk'>('stationary');

function toggleExpand(num: number) {
  expandedMovement.value = expandedMovement.value === num ? null : num;
}

function formatMinutesSeconds(seconds: number) {
  if (!seconds || seconds <= 0) return '0s';
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return s > 0 ? `${m}m ${s}s` : `${m}m`;
}
</script>

<template>
  <div class="px-3 py-4 max-w-lg mx-auto space-y-4 pb-28">
    <!-- Top Status Strip -->
    <div class="bg-slate-900 border border-slate-800 rounded-2xl p-4 flex items-center justify-between">
      <div class="flex items-center gap-2.5">
        <div class="w-9 h-9 rounded-xl bg-cyan-500/20 border border-cyan-500/40 text-cyan-400 flex items-center justify-center font-black text-sm">
          5BX
        </div>
        <div>
          <div class="text-xs font-black text-white leading-tight">
            {{ profile ? profile.username : 'Guest Pilot' }}
          </div>
          <div class="text-[10px] text-slate-400 font-mono">
            {{ workout.date }}
          </div>
        </div>
      </div>

      <!-- Quick Rung Indicators -->
      <div class="flex items-center gap-1.5 font-mono text-[11px]">
        <span class="px-2 py-1 rounded-lg bg-emerald-950/80 border border-emerald-500/40 text-emerald-300 font-bold">
          S: {{ workout.strength_chart }}-{{ workout.strength_level }}
        </span>
        <span class="px-2 py-1 rounded-lg bg-cyan-950/80 border border-cyan-500/40 text-cyan-300 font-bold">
          C: {{ workout.cardio_chart }}-{{ workout.cardio_level }}
        </span>
      </div>
    </div>

    <!-- Giant Thumb-Friendly Launch Button (KISS Core) -->
    <button
      @click="emit('start-timer')"
      class="w-full py-5 px-6 rounded-2xl bg-gradient-to-r from-emerald-500 via-teal-500 to-cyan-500 hover:from-emerald-400 hover:to-cyan-400 active:scale-[0.98] text-slate-950 font-black text-lg uppercase tracking-tight shadow-xl shadow-emerald-500/20 flex items-center justify-center gap-3 transition-transform cursor-pointer"
    >
      <span class="text-2xl">⏱️</span>
      <span>START 11-MINUTES</span>
    </button>

    <!-- Secondary Quick Actions -->
    <div class="grid grid-cols-2 gap-2.5">
      <button
        @click="emit('log-manual')"
        class="py-3 px-3 rounded-xl bg-slate-900 border border-slate-800 hover:bg-slate-800 text-slate-200 text-xs font-bold flex items-center justify-center gap-1.5 transition-colors cursor-pointer"
      >
        <span>✍️</span>
        <span>Quick Log Reps</span>
      </button>

      <button
        @click="emit('open-sheet')"
        class="py-3 px-3 rounded-xl bg-slate-900 border border-slate-800 hover:bg-slate-800 text-slate-200 text-xs font-bold flex items-center justify-center gap-1.5 transition-colors cursor-pointer"
      >
        <span>📄</span>
        <span>Gym Form Sheet</span>
      </button>
    </div>

    <!-- 5-Step Vertical Checklist -->
    <div class="space-y-2.5">
      <div class="flex items-center justify-between px-1">
        <h3 class="text-xs font-black uppercase tracking-wider text-slate-400">
          5-Step Mission Checklist
        </h3>
        <span class="text-[10px] text-cyan-400 font-mono">11 Mins Total</span>
      </div>

      <div
        v-for="ex in workout.exercises"
        :key="ex.exercise_number"
        class="bg-slate-900/90 border border-slate-800 rounded-2xl p-3.5 transition-all"
        :class="{ 'border-cyan-500/40 bg-slate-900': expandedMovement === ex.exercise_number }"
      >
        <div class="flex items-center justify-between gap-3 cursor-pointer" @click="toggleExpand(ex.exercise_number)">
          <!-- Left: Number & Name -->
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-xl bg-slate-950 border border-slate-800 flex items-center justify-center font-mono font-bold text-xs text-cyan-400 shrink-0">
              {{ ex.exercise_number }}
            </div>
            <div>
              <div class="text-sm font-bold text-white">{{ ex.name }}</div>
              <div class="text-[10px] text-slate-400 font-mono">
                {{ ex.time_limit_seconds >= 60 ? (ex.time_limit_seconds / 60) + ' min' : ex.time_limit_seconds + ' sec' }}
              </div>
            </div>
          </div>

          <!-- Right: Target Badge -->
          <div class="text-right shrink-0">
            <!-- Non-cardio targets -->
            <div v-if="ex.exercise_number < 5" class="px-2.5 py-1 rounded-xl bg-slate-950 border border-slate-800 font-mono text-xs font-black text-emerald-400">
              {{ ex.target_reps }} reps
            </div>

            <!-- Cardio Target on step 5 -->
            <div v-else class="text-right">
              <span v-if="cardioChoice === 'stationary'" class="px-2.5 py-1 rounded-xl bg-slate-950 border border-slate-800 font-mono text-xs font-black text-cyan-400">
                {{ ex.target_reps }} steps
              </span>
              <span v-else-if="cardioChoice === 'run'" class="px-2.5 py-1 rounded-xl bg-slate-950 border border-slate-800 font-mono text-xs font-black text-amber-300">
                &lt; {{ formatMinutesSeconds(ex.alt_run_time_seconds) }}
              </span>
              <span v-else class="px-2.5 py-1 rounded-xl bg-slate-950 border border-slate-800 font-mono text-xs font-black text-teal-300">
                &lt; {{ formatMinutesSeconds(ex.alt_walk_time_seconds) }}
              </span>
            </div>
          </div>
        </div>

        <!-- Exercise 5 Interactive Cardio Discipline Pills (Mobile) -->
        <div v-if="ex.exercise_number === 5" class="mt-3 pt-2.5 border-t border-slate-800/80">
          <div class="flex items-center justify-between text-[10px] text-slate-400 uppercase font-bold mb-1.5">
            <span>Cardio Discipline</span>
            <span class="text-cyan-400 font-mono">{{ cardioChoice }}</span>
          </div>
          <div class="grid grid-cols-3 gap-1.5">
            <button
              type="button"
              @click.stop="cardioChoice = 'stationary'"
              class="py-1.5 px-1 rounded-lg text-[10px] font-bold text-center uppercase tracking-tight transition-all cursor-pointer"
              :class="cardioChoice === 'stationary' ? 'bg-cyan-500 text-slate-950 font-black' : 'bg-slate-950 text-slate-400 border border-slate-800'"
            >
              👟 Stationary
            </button>
            <button
              type="button"
              @click.stop="cardioChoice = 'run'"
              class="py-1.5 px-1 rounded-lg text-[10px] font-bold text-center uppercase tracking-tight transition-all cursor-pointer"
              :class="cardioChoice === 'run' ? 'bg-cyan-500 text-slate-950 font-black' : 'bg-slate-950 text-slate-400 border border-slate-800'"
            >
              🏃 1-Mi Run
            </button>
            <button
              type="button"
              @click.stop="cardioChoice = 'walk'"
              class="py-1.5 px-1 rounded-lg text-[10px] font-bold text-center uppercase tracking-tight transition-all cursor-pointer"
              :class="cardioChoice === 'walk' ? 'bg-cyan-500 text-slate-950 font-black' : 'bg-slate-950 text-slate-400 border border-slate-800'"
            >
              🚶 2-Mi Walk
            </button>
          </div>
        </div>

        <!-- Collapsible Technique Diagram & Cue -->
        <div v-if="expandedMovement === ex.exercise_number" class="mt-3 pt-3 border-t border-slate-800 space-y-2">
          <div class="w-full h-32 bg-white rounded-xl p-2 flex items-center justify-center border border-slate-300">
            <img :src="'/images/' + ex.image_path" :alt="ex.name" class="max-h-full max-w-full object-contain" />
          </div>
          <p class="text-xs text-slate-300 leading-relaxed bg-slate-950 p-2.5 rounded-xl border border-slate-800">
            {{ ex.instructions }}
          </p>
        </div>
      </div>
    </div>

    <!-- Switch to Desktop View Toggle Link -->
    <div class="text-center pt-2">
      <button
        @click="emit('toggle-kiss')"
        class="text-xs text-slate-400 hover:text-cyan-400 underline transition-colors cursor-pointer"
      >
        Switch to Standard Full Cockpit Mode
      </button>
    </div>

    <!-- Sticky Bottom Persistent Thumb Bar -->
    <div class="fixed bottom-0 left-0 right-0 p-3 bg-slate-950/95 backdrop-blur-lg border-t border-slate-800 flex gap-2 z-40 max-w-lg mx-auto">
      <button
        @click="emit('start-timer')"
        class="flex-1 py-3 px-4 rounded-xl bg-gradient-to-r from-emerald-500 to-teal-500 active:scale-95 text-slate-950 font-black text-xs uppercase tracking-wider flex items-center justify-center gap-1.5 shadow-lg shadow-emerald-500/20 cursor-pointer"
      >
        <span>⏱️</span>
        <span>Start 11-Min Session</span>
      </button>

      <button
        @click="emit('log-manual')"
        class="py-3 px-4 rounded-xl bg-slate-900 border border-slate-800 hover:bg-slate-800 text-white text-xs font-bold flex items-center justify-center gap-1.5 cursor-pointer"
      >
        <span>✍️</span>
        <span>Log</span>
      </button>
    </div>
  </div>
</template>
