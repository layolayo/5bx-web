<script setup lang="ts">
import { ref } from 'vue';
import { TodayWorkout, UserProfile, EarnedBadge } from '../types';

defineProps<{
  workout: TodayWorkout;
  profile: UserProfile | null;
  highestBadge?: EarnedBadge | null;
}>();

const emit = defineEmits<{
  (e: 'start-timer'): void;
  (e: 'open-sheet'): void;
  (e: 'log-manual'): void;
  (e: 'open-badges'): void;
}>();

const cardioChoice = ref<'stationary' | 'run' | 'walk'>('stationary');

function formatMinutesSeconds(seconds: number) {
  if (!seconds || seconds <= 0) return '0s';
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return s > 0 ? `${m}m ${s}s` : `${m}m`;
}

const muscleFocusMap: Record<number, string> = {
  1: 'Spine Decompression, Hamstrings & Lumbar Mobility',
  2: 'Abdominals, Hip Flexors & Core Fortification',
  3: 'Spinal Extensors, Gluteals & Posterior Chain',
  4: 'Pectorals, Anterior Deltoids & Triceps Power',
  5: 'Cardiovascular Conditioning, Calves & Aerobic Capacity'
};
</script>

<template>
  <div class="max-w-5xl mx-auto px-4 py-8 space-y-8">
    <!-- Athlete Cockpit Hero Panel -->
    <div class="relative overflow-hidden rounded-3xl bg-gradient-to-br from-slate-900 via-slate-850 to-slate-950 border border-slate-700/80 p-6 sm:p-8 shadow-2xl">
      <!-- Glow ambient accent -->
      <div class="absolute -top-24 right-0 w-96 h-96 bg-cyan-500/10 blur-3xl pointer-events-none"></div>

      <div class="flex flex-col lg:flex-row lg:items-center justify-between gap-6 relative z-10">
        <div>
          <div class="flex items-center gap-2 mb-2">
            <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-cyan-950/80 border border-cyan-500/30 text-cyan-300 text-[11px] font-bold uppercase tracking-wider">
              <span class="w-1.5 h-1.5 rounded-full bg-cyan-400 animate-pulse"></span>
              <span>11-Minute Daily Cadence</span>
            </span>
            <span class="text-xs font-mono text-slate-400">{{ workout.date }}</span>
          </div>

          <h2 class="text-3xl sm:text-4xl font-black text-white uppercase tracking-tight">
            Today's Mission Briefing
          </h2>
          <p class="text-sm text-slate-300 mt-1 max-w-xl">
            Five calibrated movements executed strictly in sequence. Complete all required targets to earn your next ladder promotion.
          </p>
        </div>

        <!-- Split Ladder Cockpit Meters -->
        <div class="flex flex-wrap sm:flex-nowrap gap-3">
          <!-- Strength Gauge -->
          <div class="flex-1 min-w-[150px] bg-slate-950/90 border border-slate-800 rounded-2xl p-4 flex flex-col justify-between">
            <div class="flex items-center justify-between mb-1">
              <span class="text-[10px] font-bold uppercase tracking-wider text-slate-400">Strength Track (Ex 1–4)</span>
              <span class="w-2 h-2 rounded-full bg-emerald-400"></span>
            </div>
            <div class="text-xl font-black text-emerald-400">{{ workout.strength_display }}</div>
            <div class="text-[10px] text-slate-500 font-mono mt-0.5">Golden Rule Active</div>
          </div>

          <!-- Cardio Gauge -->
          <div class="flex-1 min-w-[150px] bg-slate-950/90 border border-slate-800 rounded-2xl p-4 flex flex-col justify-between">
            <div class="flex items-center justify-between mb-1">
              <span class="text-[10px] font-bold uppercase tracking-wider text-slate-400">Cardio Track (Ex 5)</span>
              <span class="w-2 h-2 rounded-full bg-cyan-400"></span>
            </div>
            <div class="text-xl font-black text-cyan-400">{{ workout.cardio_display }}</div>
            <div class="text-[10px] text-slate-500 font-mono mt-0.5">Stationary / Stride Target</div>
          </div>
        </div>
      </div>

      <!-- Highest Flying Honour Ribbon -->
      <div v-if="highestBadge" class="mt-6 bg-slate-950/90 border border-amber-500/30 rounded-2xl p-4 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10 shadow-lg">
        <div class="flex items-center gap-3.5">
          <div class="w-12 h-12 rounded-xl bg-amber-500/10 border border-amber-500/30 p-1.5 flex items-center justify-center shrink-0">
            <img :src="'/images/badges/' + highestBadge.image_name" :alt="highestBadge.title" class="max-h-full max-w-full object-contain" />
          </div>
          <div>
            <div class="flex items-center gap-2">
              <span class="text-[10px] font-black uppercase tracking-wider text-amber-400 bg-amber-950/60 px-2 py-0.5 rounded border border-amber-500/20">
                Highest Flying Honour
              </span>
              <span class="text-[10px] font-mono text-emerald-400 font-bold">{{ highestBadge.status_text }}</span>
            </div>
            <h4 class="text-base font-black text-white leading-tight mt-0.5">{{ highestBadge.title }}</h4>
            <div class="text-[11px] text-slate-400 font-mono">{{ highestBadge.details }}</div>
          </div>
        </div>
        <button
          @click="emit('open-badges')"
          class="text-xs font-bold text-amber-400 hover:text-amber-300 underline self-end sm:self-center cursor-pointer"
        >
          View All Awards →
        </button>
      </div>

      <!-- Tactical Action Bar (Standardised Button Sizes) -->
      <div class="mt-8 pt-6 border-t border-slate-800/80 flex flex-col sm:flex-row items-stretch sm:items-center gap-3 relative z-10">
        <!-- Main Launch Workout Button -->
        <button
          @click="emit('start-timer')"
          class="btn-control-primary flex-1 bg-gradient-to-r from-emerald-500 via-teal-500 to-cyan-500 hover:from-emerald-400 hover:to-cyan-400 text-slate-950 shadow-xl shadow-emerald-500/20 pulse-radar"
        >
          <span class="text-lg">⏱️</span>
          <span>Launch 11-Minute Guided Session</span>
        </button>

        <!-- Secondary Gym Sheet Button -->
        <button
          @click="emit('open-sheet')"
          class="btn-control-secondary bg-slate-800 hover:bg-slate-700 border border-slate-700 text-white"
        >
          <span>📄</span>
          <span>Print Single-Sheet (Gym Form)</span>
        </button>

        <!-- Manual Rep Log Button (Opens Offline Scorecard Direct) -->
        <button
          @click="emit('log-manual')"
          class="btn-control-tertiary bg-slate-900 hover:bg-slate-800 border border-slate-800 text-slate-300 hover:text-white"
        >
          <span>✍️</span>
          <span>Log Offline Rung</span>
        </button>
      </div>
    </div>

    <!-- 5 Calibrated Movements Roster -->
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-xl font-black text-white uppercase tracking-tight">Today's Flight Roster</h3>
          <p class="text-xs text-slate-400">Total duration: 11 minutes without inter-exercise pauses</p>
        </div>
        <span class="text-xs font-mono text-cyan-400 bg-cyan-950/60 border border-cyan-500/30 px-3 py-1 rounded-full">
          Chart {{ workout.strength_chart }} Standards
        </span>
      </div>

      <div class="grid grid-cols-1 gap-4">
        <div
          v-for="ex in workout.exercises"
          :key="ex.exercise_number"
          class="glass-panel glass-panel-hover rounded-2xl p-5 flex flex-col md:flex-row items-start md:items-center justify-between gap-5 transition-all"
        >
          <!-- Left: Number & Illustration -->
          <div class="flex items-center gap-4 w-full md:w-auto">
            <!-- Exercise Number Badge -->
            <div class="w-10 h-10 rounded-xl bg-slate-950 border border-slate-800 flex items-center justify-center font-black text-sm text-cyan-400 shrink-0 shadow-inner">
              {{ ex.exercise_number }}
            </div>

            <!-- Historical RCAF Movement Diagram -->
            <div class="w-36 h-24 bg-white rounded-xl p-2 flex items-center justify-center shrink-0 shadow-inner border border-slate-300">
              <img :src="'/images/' + ex.image_path" :alt="ex.name" class="max-h-full max-w-full object-contain" />
            </div>

            <!-- Movement Identity on Mobile -->
            <div class="md:hidden flex-1">
              <h4 class="text-base font-bold text-white">{{ ex.name }}</h4>
              <div class="text-[10px] font-mono text-cyan-400">
                {{ ex.time_limit_seconds >= 60 ? (ex.time_limit_seconds / 60) + ' min' : ex.time_limit_seconds + ' sec' }}
              </div>
            </div>
          </div>

          <!-- Middle: Name, Muscle Focus & Posture Cues -->
          <div class="flex-1">
            <div class="hidden md:flex items-center gap-2.5 mb-1">
              <h4 class="text-lg font-black text-white">{{ ex.name }}</h4>
              <span class="text-[11px] font-mono font-bold text-amber-300 bg-amber-950/60 px-2 py-0.5 rounded border border-amber-500/20">
                {{ ex.time_limit_seconds >= 60 ? (ex.time_limit_seconds / 60) + ' Minutes' : ex.time_limit_seconds + ' Seconds' }}
              </span>
            </div>

            <div class="text-[11px] font-semibold text-cyan-400 uppercase tracking-wide mb-1.5">
              {{ muscleFocusMap[ex.exercise_number] }}
            </div>

            <p class="text-xs text-slate-300 leading-relaxed max-w-2xl">
              {{ ex.instructions }}
            </p>

            <!-- Movement 5 Interactive Cardio Discipline Switcher -->
            <div v-if="ex.exercise_number === 5" class="mt-3 pt-3 border-t border-slate-800/80">
              <div class="flex items-center justify-between mb-2">
                <span class="text-[10px] font-bold uppercase tracking-wider text-slate-400">Cardio Discipline Choice:</span>
                <span class="text-[10px] font-mono text-cyan-400">Indoor or Outdoor Track</span>
              </div>
              <div class="grid grid-cols-3 gap-1.5 sm:gap-2">
                <button
                  type="button"
                  @click="cardioChoice = 'stationary'"
                  class="py-2 px-2.5 rounded-xl text-[11px] font-bold tracking-tight transition-all flex flex-col sm:flex-row items-center justify-center gap-1 sm:gap-1.5 cursor-pointer"
                  :class="cardioChoice === 'stationary' ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'bg-slate-950/80 text-slate-400 hover:text-white border border-slate-800'"
                >
                  <span class="text-sm">👟</span>
                  <span>Stationary Run</span>
                </button>
                <button
                  type="button"
                  @click="cardioChoice = 'run'"
                  class="py-2 px-2.5 rounded-xl text-[11px] font-bold tracking-tight transition-all flex flex-col sm:flex-row items-center justify-center gap-1 sm:gap-1.5 cursor-pointer"
                  :class="cardioChoice === 'run' ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'bg-slate-950/80 text-slate-400 hover:text-white border border-slate-800'"
                >
                  <span class="text-sm">🏃</span>
                  <span>1-Mile Jog/Run</span>
                </button>
                <button
                  type="button"
                  @click="cardioChoice = 'walk'"
                  class="py-2 px-2.5 rounded-xl text-[11px] font-bold tracking-tight transition-all flex flex-col sm:flex-row items-center justify-center gap-1 sm:gap-1.5 cursor-pointer"
                  :class="cardioChoice === 'walk' ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'bg-slate-950/80 text-slate-400 hover:text-white border border-slate-800'"
                >
                  <span class="text-sm">🚶</span>
                  <span>2-Mile Walk</span>
                </button>
              </div>
            </div>
          </div>

          <!-- Right: Calibrated Target Pill -->
          <div class="w-full md:w-auto flex md:flex-col items-center md:items-end justify-between md:justify-center p-3 md:p-4 rounded-xl bg-slate-950/90 border border-slate-800 shrink-0 min-w-[140px]">
            <span class="text-[10px] font-bold uppercase tracking-wider text-slate-400">Required Target</span>
            
            <!-- Standard Exercises 1-4 -->
            <template v-if="ex.exercise_number < 5">
              <div class="text-2xl font-black text-white tabular-nums tracking-tight">
                {{ ex.target_reps }}
                <span class="text-xs font-normal text-slate-400">reps</span>
              </div>
            </template>

            <!-- Exercise 5 Cardio Dynamic Target based on selection -->
            <template v-else>
              <!-- Stationary Run Option -->
              <div v-if="cardioChoice === 'stationary'" class="text-right">
                <div class="text-2xl font-black text-white tabular-nums tracking-tight">
                  {{ ex.target_reps }}
                  <span class="text-xs font-normal text-slate-400">runs</span>
                </div>
                <div class="text-[11px] font-semibold text-cyan-400 mt-0.5">
                  + 10 scissor jumps / 75 steps
                </div>
              </div>

              <!-- 1-Mile Outdoor Jog/Run Option -->
              <div v-else-if="cardioChoice === 'run'" class="text-right">
                <div class="text-xl font-black text-amber-300 tabular-nums tracking-tight">
                  &lt; {{ formatMinutesSeconds(ex.alt_run_time_seconds) }}
                </div>
                <div class="text-[10px] font-mono text-slate-400 mt-0.5">
                  1-Mile Run ({{ ex.alt_run_time_seconds }}s)
                </div>
              </div>

              <!-- 2-Mile Walk Option -->
              <div v-else class="text-right">
                <div class="text-xl font-black text-teal-300 tabular-nums tracking-tight">
                  &lt; {{ formatMinutesSeconds(ex.alt_walk_time_seconds) }}
                </div>
                <div class="text-[10px] font-mono text-slate-400 mt-0.5">
                  2-Mile Walk ({{ ex.alt_walk_time_seconds }}s)
                </div>
              </div>
            </template>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
