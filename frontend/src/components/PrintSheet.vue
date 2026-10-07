<script setup lang="ts">
import { TodayWorkout, UserProfile } from '../types';

defineProps<{
  workout: TodayWorkout;
  profile: UserProfile | null;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

function triggerPrint() {
  window.print();
}
</script>

<template>
  <div class="print-container">
    <!-- Non-print toolbar -->
    <div class="no-print bg-slate-800 border-b border-slate-700 p-4 flex items-center justify-between sticky top-0 z-50">
      <div>
        <h2 class="text-xl font-bold text-white">Daily Workout Form (Single Sheet)</h2>
        <p class="text-sm text-slate-400">Formatted strictly for a single A4 or Letter page for offline gym training.</p>
      </div>
      <div class="flex gap-3">
        <button
          @click="triggerPrint"
          class="bg-blue-600 hover:bg-blue-500 text-white font-medium px-4 py-2 rounded-lg flex items-center gap-2 shadow"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z" />
          </svg>
          Print to Paper / PDF
        </button>
        <button
          @click="emit('close')"
          class="bg-slate-700 hover:bg-slate-600 text-white font-medium px-4 py-2 rounded-lg"
        >
          Close
        </button>
      </div>
    </div>

    <!-- The Single Printable Sheet -->
    <div class="printable-sheet max-w-4xl mx-auto p-6 bg-white text-black shadow-lg my-4 rounded print:m-0 print:p-0 print:shadow-none">
      <!-- Header -->
      <div class="border-b-2 border-black pb-2 mb-3 flex justify-between items-end">
        <div>
          <h1 class="text-2xl font-black tracking-tight uppercase">Royal Canadian Air Force 5BX Plan</h1>
          <p class="text-xs font-semibold text-gray-700">11 Minutes A Day - Daily Workout Scorecard</p>
        </div>
        <div class="text-right text-xs">
          <p class="font-bold">Pilot: {{ workout.username }} (Age: {{ profile?.age || '--' }})</p>
          <p>Date: {{ workout.date }}</p>
        </div>
      </div>

      <!-- Current Status Banner -->
      <div class="grid grid-cols-2 gap-3 mb-3 bg-gray-100 p-2 border border-gray-400 rounded text-xs">
        <div>
          <span class="font-bold">Strength Track (Ex 1–4):</span>
          <span class="ml-2 font-mono font-bold bg-white px-2 py-0.5 border border-gray-300 rounded">{{ workout.strength_display }}</span>
        </div>
        <div>
          <span class="font-bold">Cardio Track (Ex 5):</span>
          <span class="ml-2 font-mono font-bold bg-white px-2 py-0.5 border border-gray-300 rounded">{{ workout.cardio_display }}</span>
        </div>
      </div>

      <!-- Workout Table -->
      <table class="print-table w-full border-collapse border border-gray-800 text-xs">
        <thead>
          <tr class="bg-gray-200">
            <th class="p-1 border border-gray-600 text-center w-8">#</th>
            <th class="p-1 border border-gray-600 w-24 text-center">Illustration</th>
            <th class="p-1 border border-gray-600">Exercise & Technique Cue</th>
            <th class="p-1 border border-gray-600 text-center w-24">Target Time</th>
            <th class="p-1 border border-gray-600 text-center w-28">Target Standard</th>
            <th class="p-1 border border-gray-600 text-center w-24">Actual Reps</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="ex in workout.exercises" :key="ex.exercise_number" class="border-b border-gray-400">
            <td class="p-2 border border-gray-400 text-center font-bold text-sm">{{ ex.exercise_number }}</td>
            <td class="p-1 border border-gray-400 text-center">
              <img :src="'/images/' + ex.image_path" :alt="ex.name" class="print-img mx-auto max-h-12 max-w-20 object-contain" />
            </td>
            <td class="p-2 border border-gray-400">
              <div class="font-bold text-sm">{{ ex.name }}</div>
              <div class="text-[10px] text-gray-700 leading-tight mt-0.5">{{ ex.instructions }}</div>
            </td>
            <td class="p-2 border border-gray-400 text-center font-medium">
              {{ ex.time_limit_seconds >= 60 ? (ex.time_limit_seconds / 60) + ' min' : ex.time_limit_seconds + ' sec' }}
            </td>
            <td class="p-2 border border-gray-400 text-center">
              <div v-if="!ex.is_cardio" class="font-bold text-sm">{{ ex.target_reps }} reps</div>
              <div v-else class="text-[10px] leading-tight">
                <span class="font-bold text-sm">{{ ex.target_reps }} steps</span><br/>
                <span v-if="ex.alt_run_time_seconds > 0" class="text-gray-600">Run: {{ Math.floor(ex.alt_run_time_seconds / 60) }}:{{ (ex.alt_run_time_seconds % 60).toString().padStart(2, '0') }}</span>
              </div>
            </td>
            <td class="p-1 border border-gray-400 text-center">
              <div class="print-write-box mx-auto border-2 border-dashed border-gray-400 rounded h-8 w-16"></div>
            </td>
          </tr>
        </tbody>
      </table>

      <!-- Notes / Offline Log Section -->
      <div class="mt-3 grid grid-cols-3 gap-3 text-[10px]">
        <div class="col-span-2 border border-gray-400 p-2 rounded">
          <span class="font-bold uppercase">Workout Notes & Energy Level:</span>
          <div class="h-10 mt-1 border-b border-gray-300"></div>
        </div>
        <div class="border border-gray-400 p-2 rounded">
          <span class="font-bold uppercase">Verdict Checklist:</span>
          <div class="mt-1 space-y-1">
            <label class="flex items-center gap-1"><input type="checkbox" /> All Strength Targets Met (Level Up)</label>
            <label class="flex items-center gap-1"><input type="checkbox" /> Exceeded Targets (Leapfrog)</label>
            <label class="flex items-center gap-1"><input type="checkbox" /> Cardio Target Met</label>
          </div>
        </div>
      </div>

      <!-- Footer Rules -->
      <div class="mt-2 text-[9px] text-gray-500 text-center border-t border-gray-300 pt-1">
        Royal Canadian Air Force 5BX Fitness Standard • 11 Minutes Daily • Do not strain; progress strictly by completing standard reps within time limits.
      </div>
    </div>
  </div>
</template>

<style scoped>
@media screen {
  .print-container {
    min-height: 100vh;
    background-color: #0f172a;
    padding-bottom: 2rem;
  }
}
</style>
