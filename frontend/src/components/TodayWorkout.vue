<script setup lang="ts">
import { TodayWorkout, UserProfile } from '../types';

defineProps<{
  workout: TodayWorkout;
  profile: UserProfile | null;
}>();

const emit = defineEmits<{
  (e: 'start-timer'): void;
  (e: 'open-sheet'): void;
  (e: 'log-manual'): void;
}>();
</script>

<template>
  <div class="max-w-4xl mx-auto p-4 sm:p-6 space-y-6">
    <!-- Hero / Status Banner -->
    <div class="bg-gradient-to-r from-slate-800 to-slate-850 border border-slate-700 rounded-2xl p-6 shadow-xl relative overflow-hidden">
      <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 relative z-10">
        <div>
          <div class="flex items-center gap-2 mb-1">
            <span class="bg-blue-600/30 text-blue-400 border border-blue-500/30 px-2.5 py-0.5 rounded-full text-xs font-semibold tracking-wide uppercase">
              11-Minute Mission
            </span>
            <span class="text-xs text-slate-400">{{ workout.date }}</span>
          </div>
          <h2 class="text-2xl sm:text-3xl font-black text-white">Daily Workout Schedule</h2>
          <p class="text-sm text-slate-300 mt-1">Five movements executed in sequence without resting equipment.</p>
        </div>

        <!-- Quick Level Pills -->
        <div class="flex sm:flex-col gap-2">
          <div class="bg-slate-900/80 border border-slate-700 px-3.5 py-1.5 rounded-xl">
            <span class="text-[10px] text-slate-400 font-semibold block uppercase">Strength (Ex 1–4)</span>
            <span class="text-base font-bold text-emerald-400">{{ workout.strength_display }}</span>
          </div>
          <div class="bg-slate-900/80 border border-slate-700 px-3.5 py-1.5 rounded-xl">
            <span class="text-[10px] text-slate-400 font-semibold block uppercase">Cardio (Ex 5)</span>
            <span class="text-base font-bold text-blue-400">{{ workout.cardio_display }}</span>
          </div>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="mt-6 pt-5 border-t border-slate-700/60 flex flex-wrap gap-3 relative z-10">
        <button
          @click="emit('start-timer')"
          class="flex-1 min-w-[200px] bg-blue-600 hover:bg-blue-500 text-white font-bold py-3.5 px-6 rounded-xl shadow-lg transition-transform active:scale-95 flex items-center justify-center gap-2 text-base"
        >
          <span>⏱️</span>
          <span>Start 11-Minute Timer</span>
        </button>

        <button
          @click="emit('open-sheet')"
          class="bg-slate-800 hover:bg-slate-700 text-slate-200 font-semibold py-3.5 px-5 rounded-xl border border-slate-600 flex items-center gap-2 text-sm transition-colors"
        >
          <span>📄</span>
          <span>Single-Sheet Form (Print / PDF)</span>
        </button>

        <button
          @click="emit('start-timer')"
          class="bg-slate-800/80 hover:bg-slate-700 text-slate-300 font-medium py-3.5 px-4 rounded-xl border border-slate-700 text-sm transition-colors"
        >
          <span>✍️</span>
          <span>Log Reps Directly</span>
        </button>
      </div>
    </div>

    <!-- 5 Exercise Cards List -->
    <div class="space-y-4">
      <h3 class="text-lg font-bold text-white flex items-center gap-2">
        <span>Today's Exercises</span>
        <span class="text-xs font-normal text-slate-400">(Total 11 Minutes)</span>
      </h3>

      <div
        v-for="ex in workout.exercises"
        :key="ex.exercise_number"
        class="bg-slate-800/90 border border-slate-700 rounded-xl p-4 sm:p-5 flex flex-col sm:flex-row items-center gap-4 transition-all hover:border-slate-600"
      >
        <!-- Movement Number Badge -->
        <div class="w-10 h-10 rounded-full bg-slate-900 border border-slate-700 flex items-center justify-center text-slate-200 font-black text-sm shrink-0">
          {{ ex.exercise_number }}
        </div>

        <!-- Exercise Image Thumbnail -->
        <div class="w-36 h-24 bg-white rounded-lg p-1.5 flex items-center justify-center shrink-0 shadow-inner">
          <img :src="'/images/' + ex.image_path" :alt="ex.name" class="max-h-full max-w-full object-contain" />
        </div>

        <!-- Exercise Details -->
        <div class="flex-1 text-center sm:text-left">
          <div class="flex flex-wrap items-center justify-center sm:justify-start gap-2 mb-1">
            <h4 class="text-base font-bold text-white">{{ ex.name }}</h4>
            <span class="text-xs bg-slate-700 text-slate-300 px-2 py-0.5 rounded font-mono">
              {{ ex.time_limit_seconds >= 60 ? (ex.time_limit_seconds / 60) + ' min' : ex.time_limit_seconds + ' sec' }}
            </span>
          </div>
          <p class="text-xs text-slate-300 leading-relaxed">{{ ex.instructions }}</p>
        </div>

        <!-- Target Target Counter -->
        <div class="bg-slate-900/90 border border-slate-700/80 px-4 py-2.5 rounded-xl text-center shrink-0 w-full sm:w-auto">
          <span class="text-[10px] text-slate-400 font-semibold uppercase block">Prescribed Target</span>
          <div v-if="!ex.is_cardio" class="text-xl font-black text-emerald-400">
            {{ ex.target_reps }} <span class="text-xs font-normal text-slate-400">reps</span>
          </div>
          <div v-else class="text-center">
            <span class="text-lg font-black text-blue-400">{{ ex.target_reps }} <span class="text-xs font-normal text-slate-400">steps</span></span>
            <div v-if="ex.alt_run_time_seconds > 0" class="text-[10px] text-slate-400 mt-0.5">
              Run: {{ Math.floor(ex.alt_run_time_seconds / 60) }}:{{ (ex.alt_run_time_seconds % 60).toString().padStart(2, '0') }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
