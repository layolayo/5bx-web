<script setup lang="ts">
import { ref, computed } from 'vue';
import { TodayWorkout, UserProfile, EarnedBadge, LayoffStatus } from '../types';

const props = defineProps<{
  workout: TodayWorkout;
  profile: UserProfile | null;
  highestBadge?: EarnedBadge | null;
  layoffStatus?: LayoffStatus | null;
}>();

const emit = defineEmits<{
  (e: 'start-timer'): void;
  (e: 'open-sheet'): void;
  (e: 'log-manual'): void;
  (e: 'open-badges'): void;
  (e: 'open-assessment'): void;
}>();

const cardioChoice = ref<'stationary' | 'run' | 'walk'>('stationary');

const strengthExercises = computed(() => props.workout.exercises.filter((ex) => ex.exercise_number <= 4));
const cardioExercise = computed(() => props.workout.exercises.find((ex) => ex.exercise_number === 5));

const cardioDistanceMiles = computed(() => {
  if (cardioChoice.value === 'run') {
    return props.workout.cardio_chart === 1 ? 0.5 : 1.0;
  }
  if (cardioChoice.value === 'walk') {
    return props.workout.cardio_chart === 1 ? 1.0 : 2.0;
  }
  return 0;
});

const treadmillSpeed = computed(() => {
  const ex5 = cardioExercise.value;
  if (!ex5) return { mph: 0, kph: 0 };
  const targetSec = cardioChoice.value === 'run' ? ex5.alt_run_time_seconds : ex5.alt_walk_time_seconds;
  if (!targetSec || targetSec <= 0) return { mph: 0, kph: 0 };
  const hours = targetSec / 3600.0;
  const mph = cardioDistanceMiles.value / hours;
  const kph = mph * 1.60934;
  return { mph, kph };
});

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

      <!-- Layoff Advisory Banner (Desktop) -->
      <div v-if="layoffStatus && layoffStatus.is_layoff" class="mt-6 bg-amber-500/10 border border-amber-500/30 rounded-2xl p-4 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3 text-xs text-amber-300 relative z-10 shadow-lg">
        <div class="flex items-center gap-3">
          <span class="text-2xl">⚠️</span>
          <div>
            <div class="font-bold text-sm text-amber-200">
              Absence Detected: {{ layoffStatus.days_inactive }} Days Inactive
            </div>
            <div class="text-xs text-amber-300/80 mt-0.5">
              {{ layoffStatus.rationale }}
            </div>
          </div>
        </div>
        <button
          @click="emit('open-assessment')"
          class="px-4 py-2 bg-amber-500 hover:bg-amber-400 text-slate-950 font-black rounded-xl text-xs transition cursor-pointer shrink-0 shadow"
        >
          Calibrate Re-entry Rung →
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
          <span>Gym Form Sheet</span>
        </button>

        <!-- Manual Rep Log Button (Opens Offline Scorecard Direct) -->
        <button
          @click="emit('log-manual')"
          class="btn-control-tertiary bg-slate-900 hover:bg-slate-800 border border-slate-800 text-slate-300 hover:text-white"
        >
          <span>✍️</span>
          <span>Log Offline Rung</span>
        </button>

        <!-- Assess Level / Layoff Re-entry Button -->
        <button
          @click="emit('open-assessment')"
          class="btn-control-tertiary bg-slate-900 hover:bg-slate-800 border border-slate-800 text-cyan-300 hover:text-white"
        >
          <span>🧭</span>
          <span>Assess Level</span>
        </button>
      </div>
    </div>

    <!-- Grouped Mission Rosters -->
    <div class="space-y-8">
      <!-- 1. STRENGTH TRACK (Movements 1 to 4 - Green / Emerald Theme) -->
      <div class="space-y-4">
        <!-- Strength Track Header Line -->
        <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-3 border-b-2 border-emerald-500/40 gap-2">
          <div class="flex items-center gap-3">
            <span class="w-3.5 h-3.5 rounded-full bg-emerald-400 shadow-md shadow-emerald-500/50 animate-pulse"></span>
            <div>
              <div class="flex items-center gap-2">
                <h3 class="text-xl font-black text-white uppercase tracking-tight">Strength Track</h3>
                <span class="text-[10px] font-mono font-bold bg-emerald-950/80 text-emerald-300 border border-emerald-500/40 px-2 py-0.5 rounded-full">
                  Movements 1 to 4 • 5 Minutes Total
                </span>
              </div>
              <p class="text-xs text-slate-400">Core stabilization, spinal mobility & upper body resistance. Golden Rule applies.</p>
            </div>
          </div>
          <span class="text-xs font-mono text-emerald-300 bg-emerald-950/60 border border-emerald-500/30 px-3 py-1 rounded-full self-start sm:self-auto">
            Chart {{ workout.strength_chart }} Standards
          </span>
        </div>

        <!-- 4 Strength Exercise Cards -->
        <div class="grid grid-cols-1 gap-3.5">
          <div
            v-for="ex in strengthExercises"
            :key="ex.exercise_number"
            class="glass-panel glass-panel-hover rounded-2xl p-5 border-l-4 border-l-emerald-500 flex flex-col md:flex-row items-start md:items-center justify-between gap-5 transition-all"
          >
            <!-- Left: Number & Illustration -->
            <div class="flex items-center gap-4 w-full md:w-auto">
              <div class="w-10 h-10 rounded-xl bg-slate-950 border border-emerald-500/30 flex items-center justify-center font-black text-sm text-emerald-400 shrink-0 shadow-inner">
                {{ ex.exercise_number }}
              </div>

              <div class="w-36 h-24 bg-white rounded-xl p-2 flex items-center justify-center shrink-0 shadow-inner border border-slate-300">
                <img :src="'/images/' + ex.image_path" :alt="ex.name" class="max-h-full max-w-full object-contain" />
              </div>

              <div class="md:hidden flex-1">
                <h4 class="text-base font-bold text-white">{{ ex.name }}</h4>
                <div class="text-[10px] font-mono text-emerald-400">
                  {{ ex.time_limit_seconds >= 60 ? (ex.time_limit_seconds / 60) + ' min' : ex.time_limit_seconds + ' sec' }}
                </div>
              </div>
            </div>

            <!-- Middle: Name, Muscle Focus & Posture Cues -->
            <div class="flex-1">
              <div class="hidden md:flex items-center gap-2.5 mb-1">
                <h4 class="text-lg font-black text-white">{{ ex.name }}</h4>
                <span class="text-[11px] font-mono font-bold text-emerald-300 bg-emerald-950/60 px-2 py-0.5 rounded border border-emerald-500/20">
                  {{ ex.time_limit_seconds >= 60 ? (ex.time_limit_seconds / 60) + ' Minutes' : ex.time_limit_seconds + ' Seconds' }}
                </span>
              </div>

              <div class="text-[11px] font-semibold text-emerald-400 uppercase tracking-wide mb-1.5">
                {{ muscleFocusMap[ex.exercise_number] }}
              </div>

              <p class="text-xs text-slate-300 leading-relaxed max-w-2xl">
                {{ ex.instructions }}
              </p>
            </div>

            <!-- Right: Calibrated Target Pill -->
            <div class="w-full md:w-auto flex md:flex-col items-center md:items-end justify-between md:justify-center p-3 md:p-4 rounded-xl bg-slate-950/90 border border-slate-800 shrink-0 min-w-[130px]">
              <span class="text-[10px] font-bold uppercase tracking-wider text-slate-400">Required Target</span>
              <div class="text-2xl font-black text-emerald-400 tabular-nums tracking-tight">
                {{ ex.target_reps }}
                <span class="text-xs font-normal text-slate-400">reps</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 2. CARDIO TRACK (Movement 5 - Cyan Theme) -->
      <div v-if="cardioExercise" class="space-y-4">
        <!-- Cardio Track Header Line -->
        <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-3 border-b-2 border-cyan-500/40 gap-2">
          <div class="flex items-center gap-3">
            <span class="w-3.5 h-3.5 rounded-full bg-cyan-400 shadow-md shadow-cyan-500/50 animate-pulse"></span>
            <div>
              <div class="flex items-center gap-2">
                <h3 class="text-xl font-black text-white uppercase tracking-tight">Cardio Track</h3>
                <span class="text-[10px] font-mono font-bold bg-cyan-950/80 text-cyan-300 border border-cyan-500/40 px-2 py-0.5 rounded-full">
                  Movement 5 • 6 Minutes / Calibrated Stride Target
                </span>
              </div>
              <p class="text-xs text-slate-400">Cardiovascular conditioning & aerobic capacity. Choose stationary or running machine / outdoor.</p>
            </div>
          </div>
          <span class="text-xs font-mono text-cyan-300 bg-cyan-950/60 border border-cyan-500/30 px-3 py-1 rounded-full self-start sm:self-auto">
            Chart {{ workout.cardio_chart }} Standards
          </span>
        </div>

        <!-- Cardio Exercise Card -->
        <div class="glass-panel glass-panel-hover rounded-2xl p-5 border-l-4 border-l-cyan-500 transition-all">
          <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-5">
            <!-- Left: Number & Diagram -->
            <div class="flex items-center gap-4 w-full md:w-auto">
              <div class="w-10 h-10 rounded-xl bg-slate-950 border border-cyan-500/30 flex items-center justify-center font-black text-sm text-cyan-400 shrink-0 shadow-inner">
                5
              </div>

              <div class="w-36 h-24 bg-white rounded-xl p-2 flex items-center justify-center shrink-0 shadow-inner border border-slate-300">
                <img :src="'/images/' + cardioExercise.image_path" :alt="cardioExercise.name" class="max-h-full max-w-full object-contain" />
              </div>

              <div class="md:hidden flex-1">
                <h4 class="text-base font-bold text-white">{{ cardioExercise.name }}</h4>
                <div class="text-[10px] font-mono text-cyan-400">6 Minutes</div>
              </div>
            </div>

            <!-- Middle: Description & Discipline Selector -->
            <div class="flex-1 w-full">
              <div class="hidden md:flex items-center gap-2.5 mb-1">
                <h4 class="text-lg font-black text-white">{{ cardioExercise.name }}</h4>
                <span class="text-[11px] font-mono font-bold text-cyan-300 bg-cyan-950/60 px-2 py-0.5 rounded border border-cyan-500/20">
                  6 Minutes
                </span>
              </div>

              <div class="text-[11px] font-semibold text-cyan-400 uppercase tracking-wide mb-1.5">
                {{ muscleFocusMap[5] }}
              </div>

              <p class="text-xs text-slate-300 leading-relaxed max-w-2xl">
                {{ cardioExercise.instructions }}
              </p>

              <!-- Interactive Cardio Discipline Switcher -->
              <div class="mt-4 pt-3 border-t border-slate-800/80">
                <div class="flex items-center justify-between mb-2">
                  <span class="text-[10px] font-bold uppercase tracking-wider text-slate-400">Choose Cardio Discipline:</span>
                  <span class="text-[10px] font-mono text-cyan-400">Indoor or Running Machine</span>
                </div>
                <div class="grid grid-cols-3 gap-2">
                  <button
                    type="button"
                    @click="cardioChoice = 'stationary'"
                    class="py-2 px-2.5 rounded-xl text-xs font-bold tracking-tight transition-all flex flex-col sm:flex-row items-center justify-center gap-1 sm:gap-1.5 cursor-pointer"
                    :class="cardioChoice === 'stationary' ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'bg-slate-950/80 text-slate-400 hover:text-white border border-slate-800'"
                  >
                    <span class="text-sm">👟</span>
                    <span>Stationary Run</span>
                  </button>
                  <button
                    type="button"
                    @click="cardioChoice = 'run'"
                    class="py-2 px-2.5 rounded-xl text-xs font-bold tracking-tight transition-all flex flex-col sm:flex-row items-center justify-center gap-1 sm:gap-1.5 cursor-pointer"
                    :class="cardioChoice === 'run' ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'bg-slate-950/80 text-slate-400 hover:text-white border border-slate-800'"
                  >
                    <span class="text-sm">🏃</span>
                    <span>1-Mile Jog/Run</span>
                  </button>
                  <button
                    type="button"
                    @click="cardioChoice = 'walk'"
                    class="py-2 px-2.5 rounded-xl text-xs font-bold tracking-tight transition-all flex flex-col sm:flex-row items-center justify-center gap-1 sm:gap-1.5 cursor-pointer"
                    :class="cardioChoice === 'walk' ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'bg-slate-950/80 text-slate-400 hover:text-white border border-slate-800'"
                  >
                    <span class="text-sm">🚶</span>
                    <span>2-Mile Walk</span>
                  </button>
                </div>
              </div>
            </div>

            <!-- Right: Dynamic Target Pill -->
            <div class="w-full md:w-auto flex md:flex-col items-center md:items-end justify-between md:justify-center p-4 rounded-xl bg-slate-950/90 border border-slate-800 shrink-0 min-w-[160px]">
              <span class="text-[10px] font-bold uppercase tracking-wider text-slate-400">Required Target</span>

              <!-- Stationary Option Target -->
              <div v-if="cardioChoice === 'stationary'" class="text-right">
                <div class="text-2xl font-black text-cyan-400 tabular-nums tracking-tight">
                  {{ cardioExercise.target_reps }}
                  <span class="text-xs font-normal text-slate-400">runs</span>
                </div>
                <div class="text-[11px] font-semibold text-cyan-300 mt-0.5">
                  + 10 scissor jumps / 75 steps
                </div>
              </div>

              <!-- 1-Mile Run Option Target -->
              <div v-else-if="cardioChoice === 'run'" class="text-right">
                <div class="text-xl font-black text-amber-300 tabular-nums tracking-tight">
                  &lt; {{ formatMinutesSeconds(cardioExercise.alt_run_time_seconds) }}
                </div>
                <div class="text-[10px] font-mono text-slate-400 mt-0.5">
                  {{ cardioDistanceMiles }} Mile ({{ cardioExercise.alt_run_time_seconds }}s)
                </div>
              </div>

              <!-- 2-Mile Walk Option Target -->
              <div v-else class="text-right">
                <div class="text-xl font-black text-teal-300 tabular-nums tracking-tight">
                  &lt; {{ formatMinutesSeconds(cardioExercise.alt_walk_time_seconds) }}
                </div>
                <div class="text-[10px] font-mono text-slate-400 mt-0.5">
                  {{ cardioDistanceMiles }} Miles ({{ cardioExercise.alt_walk_time_seconds }}s)
                </div>
              </div>
            </div>
          </div>

          <!-- Treadmill Speed Calibration Banner (Run / Walk) -->
          <div
            v-if="cardioChoice !== 'stationary' && treadmillSpeed.mph > 0"
            class="mt-4 bg-gradient-to-r from-cyan-950/70 via-slate-900 to-cyan-950/50 border border-cyan-500/40 rounded-xl p-3.5 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3 shadow-lg"
          >
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-xl bg-cyan-500/20 border border-cyan-500/40 flex items-center justify-center text-xl shrink-0">
                ⚙️
              </div>
              <div>
                <span class="text-[10px] font-black uppercase tracking-wider text-cyan-400 block">
                  Treadmill Running Machine Setting
                </span>
                <div class="text-sm font-black text-white mt-0.5">
                  Set speed to at least <span class="text-cyan-300 font-mono">{{ treadmillSpeed.mph.toFixed(1) }} mph</span> (<span class="text-cyan-300 font-mono">{{ treadmillSpeed.kph.toFixed(1) }} km/h</span>)
                </div>
                <span class="text-[11px] text-slate-400 block">
                  Complete {{ cardioDistanceMiles }} mile(s) within {{ formatMinutesSeconds(cardioChoice === 'run' ? cardioExercise.alt_run_time_seconds : cardioExercise.alt_walk_time_seconds) }} to pass this rung.
                </span>
              </div>
            </div>
            <div class="bg-cyan-950/90 border border-cyan-500/30 px-3 py-1.5 rounded-lg text-right self-stretch sm:self-center font-mono">
              <div class="text-[10px] text-slate-400 uppercase">Pace Target</div>
              <div class="text-xs font-bold text-cyan-300">
                {{ Math.floor((cardioChoice === 'run' ? cardioExercise.alt_run_time_seconds : cardioExercise.alt_walk_time_seconds) / 60) }}m {{ (cardioChoice === 'run' ? cardioExercise.alt_run_time_seconds : cardioExercise.alt_walk_time_seconds) % 60 }}s / mi
              </div>
            </div>
          </div>

          <!-- Stationary Run Info Banner -->
          <div
            v-else-if="cardioChoice === 'stationary'"
            class="mt-4 bg-slate-950/80 border border-slate-800 rounded-xl p-3 text-xs text-slate-300 flex items-center gap-3"
          >
            <span class="text-lg">👟</span>
            <div>
              <span class="text-white font-bold">Stationary Run Protocol:</span> Run on the spot lifting feet 4 inches off the floor (1 step counted per forward left foot contact). Perform 10 scissor jumps every 75 steps (6 minutes continuous).
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
