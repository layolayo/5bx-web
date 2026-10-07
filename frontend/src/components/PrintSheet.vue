<script setup lang="ts">
import { computed } from 'vue';
import { TodayWorkout, UserProfile } from '../types';

const props = defineProps<{
  workout: TodayWorkout;
  profile: UserProfile | null;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

function triggerPrint() {
  window.print();
}

const runDistanceMiles = computed(() => {
  return props.workout.cardio_chart === 1 ? 0.5 : 1.0;
});

const walkDistanceMiles = computed(() => {
  return props.workout.cardio_chart === 1 ? 1.0 : 2.0;
});

function formatMmSs(seconds: number): string {
  if (!seconds || seconds <= 0) return '00:00';
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return `${m}:${s.toString().padStart(2, '0')}`;
}

function calcTreadmillSpeed(miles: number, seconds: number): { mph: string; kph: string } {
  if (!seconds || seconds <= 0 || !miles || miles <= 0) {
    return { mph: '0.0', kph: '0.0' };
  }
  const hours = seconds / 3600.0;
  const mph = miles / hours;
  const kph = mph * 1.60934;
  return {
    mph: mph.toFixed(1),
    kph: kph.toFixed(1),
  };
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
          class="bg-blue-600 hover:bg-blue-500 text-white font-medium px-4 py-2 rounded-lg flex items-center gap-2 shadow cursor-pointer"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H5a2 2 0 00-2 2v4a2 2 0 002 2h2m2 4h6a2 2 0 002-2v-4a2 2 0 00-2-2H9a2 2 0 00-2 2v4a2 2 0 002 2zm8-12V5a2 2 0 00-2-2H9a2 2 0 00-2 2v4h10z" />
          </svg>
          Print to Paper / PDF
        </button>
        <button
          @click="emit('close')"
          class="bg-slate-700 hover:bg-slate-600 text-white font-medium px-4 py-2 rounded-lg cursor-pointer"
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
          <p class="text-xs font-semibold text-gray-700">11 Minutes A Day • Daily Workout Scorecard</p>
        </div>
        <div class="text-right text-xs">
          <p class="font-bold">Pilot: {{ workout.username }} (Age: {{ profile?.age || '--' }})</p>
          <p>Date: {{ workout.date }}</p>
        </div>
      </div>

      <!-- Current Status Banner -->
      <div class="grid grid-cols-2 gap-3 mb-3 bg-gray-100 p-2 border border-gray-400 rounded text-xs">
        <div>
          <span class="font-bold">Strength Track (Movements 1–4):</span>
          <span class="ml-2 font-mono font-bold bg-white px-2 py-0.5 border border-gray-300 rounded">{{ workout.strength_display }}</span>
        </div>
        <div>
          <span class="font-bold">Cardio Track (Movement 5):</span>
          <span class="ml-2 font-mono font-bold bg-white px-2 py-0.5 border border-gray-300 rounded">{{ workout.cardio_display }}</span>
        </div>
      </div>

      <!-- Workout Table -->
      <table class="print-table w-full border-collapse border border-gray-800 text-xs">
        <thead>
          <tr class="bg-gray-200">
            <th class="p-1 border border-gray-600 text-center w-8">#</th>
            <th class="p-1 border border-gray-600 w-20 text-center">Illustration</th>
            <th class="p-1 border border-gray-600">Exercise & Technique Cue</th>
            <th class="p-1 border border-gray-600 text-center w-24">Target Time</th>
            <th class="p-1 border border-gray-600 text-left w-64">Target Standard</th>
            <th class="p-1 border border-gray-600 text-left w-44">Actual Performance</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="ex in workout.exercises" :key="ex.exercise_number" class="border-b border-gray-400">
            <td class="p-2 border border-gray-400 text-center font-bold text-sm">{{ ex.exercise_number }}</td>
            <td class="p-1 border border-gray-400 text-center">
              <img :src="'/images/' + ex.image_path" :alt="ex.name" class="print-img mx-auto max-h-12 max-w-16 object-contain" />
            </td>
            <td class="p-2 border border-gray-400">
              <div class="font-bold text-sm">{{ ex.name }}</div>
              <div class="text-[10px] text-gray-700 leading-tight mt-0.5">{{ ex.instructions }}</div>
              <div v-if="ex.is_cardio" class="text-[9.5px] text-gray-600 italic mt-1 font-medium bg-gray-50 border border-gray-200 p-1 rounded">
                Perform any ONE of the three calibrated cardio disciplines below.
              </div>
            </td>

            <!-- Target Time Column -->
            <td class="p-2 border border-gray-400 text-center font-medium">
              <template v-if="!ex.is_cardio">
                {{ ex.time_limit_seconds >= 60 ? (ex.time_limit_seconds / 60) + ' min' : ex.time_limit_seconds + ' sec' }}
              </template>
              <template v-else>
                <div class="text-[10px] leading-tight space-y-1 text-center font-mono">
                  <div><span class="font-bold">Stationary:</span> 6 min</div>
                  <div v-if="ex.alt_run_time_seconds > 0"><span class="font-bold">Run:</span> ≤ {{ formatMmSs(ex.alt_run_time_seconds) }}</div>
                  <div v-if="ex.alt_walk_time_seconds > 0"><span class="font-bold">Walk:</span> ≤ {{ formatMmSs(ex.alt_walk_time_seconds) }}</div>
                </div>
              </template>
            </td>

            <!-- Target Standard Column -->
            <td class="p-2 border border-gray-400">
              <!-- Movements 1 to 4: Strength -->
              <div v-if="!ex.is_cardio" class="font-bold text-sm text-center">
                {{ ex.target_reps }} reps
              </div>

              <!-- Movement 5: Cardio Disciplines & Running Machine Speeds -->
              <div v-else class="text-[10px] leading-snug space-y-1.5">
                <!-- Option 1: Stationary Run -->
                <div class="bg-gray-50 border border-gray-200 rounded p-1">
                  <div class="font-bold text-gray-900 flex items-center justify-between">
                    <span>1. Stationary Run</span>
                    <span class="font-mono text-cyan-800">{{ ex.target_reps }} steps</span>
                  </div>
                  <div class="text-[9px] text-gray-600 mt-0.5">
                    10 scissor jumps every 75 steps
                  </div>
                </div>

                <!-- Option 2: Timed Run / Jog -->
                <div v-if="ex.alt_run_time_seconds > 0" class="bg-gray-50 border border-gray-200 rounded p-1">
                  <div class="font-bold text-gray-900 flex items-center justify-between">
                    <span>2. Run / Jog ({{ runDistanceMiles }} mi / {{ (runDistanceMiles * 1.60934).toFixed(1) }} km)</span>
                    <span class="font-mono text-amber-800">≤ {{ formatMmSs(ex.alt_run_time_seconds) }}</span>
                  </div>
                  <div class="text-[9px] text-gray-700 font-mono mt-0.5">
                    Treadmill: ≥ <strong>{{ calcTreadmillSpeed(runDistanceMiles, ex.alt_run_time_seconds).kph }} km/h</strong> ({{ calcTreadmillSpeed(runDistanceMiles, ex.alt_run_time_seconds).mph }} mph)
                  </div>
                </div>

                <!-- Option 3: Timed Walk -->
                <div v-if="ex.alt_walk_time_seconds > 0" class="bg-gray-50 border border-gray-200 rounded p-1">
                  <div class="font-bold text-gray-900 flex items-center justify-between">
                    <span>3. Continuous Walk ({{ walkDistanceMiles }} mi / {{ (walkDistanceMiles * 1.60934).toFixed(1) }} km)</span>
                    <span class="font-mono text-teal-800">≤ {{ formatMmSs(ex.alt_walk_time_seconds) }}</span>
                  </div>
                  <div class="text-[9px] text-gray-700 font-mono mt-0.5">
                    Treadmill: ≥ <strong>{{ calcTreadmillSpeed(walkDistanceMiles, ex.alt_walk_time_seconds).kph }} km/h</strong> ({{ calcTreadmillSpeed(walkDistanceMiles, ex.alt_walk_time_seconds).mph }} mph)
                  </div>
                </div>
              </div>
            </td>

            <!-- Actual Performance Column -->
            <td class="p-2 border border-gray-400">
              <!-- Movements 1 to 4: Write-in Box -->
              <div v-if="!ex.is_cardio" class="flex items-center justify-center gap-1.5">
                <div class="print-write-box border-2 border-dashed border-gray-400 rounded h-8 w-16"></div>
                <span class="text-[11px] text-gray-600 font-medium">reps</span>
              </div>

              <!-- Movement 5: Discipline Selection Write-in Box -->
              <div v-else class="text-[10px] space-y-2">
                <div class="flex items-center gap-1.5">
                  <input type="checkbox" class="w-3.5 h-3.5 border-gray-500 rounded" />
                  <span class="font-medium text-gray-800 w-20">Stationary:</span>
                  <div class="border-b-2 border-dashed border-gray-500 w-14 h-5 text-center text-xs"></div>
                  <span class="text-[9px] text-gray-500">steps</span>
                </div>
                <div class="flex items-center gap-1.5">
                  <input type="checkbox" class="w-3.5 h-3.5 border-gray-500 rounded" />
                  <span class="font-medium text-gray-800 w-20 text-[9px] leading-tight">Run ({{ runDistanceMiles }}m/{{ (runDistanceMiles * 1.60934).toFixed(1) }}k):</span>
                  <div class="border-b-2 border-dashed border-gray-500 w-14 h-5 text-center text-xs"></div>
                  <span class="text-[9px] text-gray-500">mm:ss</span>
                </div>
                <div class="flex items-center gap-1.5">
                  <input type="checkbox" class="w-3.5 h-3.5 border-gray-500 rounded" />
                  <span class="font-medium text-gray-800 w-20 text-[9px] leading-tight">Walk ({{ walkDistanceMiles }}m/{{ (walkDistanceMiles * 1.60934).toFixed(1) }}k):</span>
                  <div class="border-b-2 border-dashed border-gray-500 w-14 h-5 text-center text-xs"></div>
                  <span class="text-[9px] text-gray-500">mm:ss</span>
                </div>
              </div>
            </td>
          </tr>
        </tbody>
      </table>

      <!-- Notes / Offline Log Section -->
      <div class="mt-3 grid grid-cols-3 gap-3 text-[10px]">
        <div class="col-span-2 border border-gray-400 p-2 rounded flex flex-col justify-between">
          <div>
            <span class="font-bold uppercase">Workout Notes & Energy Level:</span>
            <div class="h-8 mt-1 border-b border-gray-300"></div>
          </div>
          <p class="text-[9px] text-gray-600 mt-1 italic">
            Running Machine / Treadmill calibration: Maintain the calibrated speed shown above (or higher) to ensure completion within the official RCAF time ceiling.
          </p>
        </div>
        <div class="border border-gray-400 p-2 rounded">
          <span class="font-bold uppercase">Verdict Checklist:</span>
          <div class="mt-1 space-y-1">
            <label class="flex items-center gap-1"><input type="checkbox" /> All 4 Strength Targets Met (Level Up)</label>
            <label class="flex items-center gap-1"><input type="checkbox" /> Exceeded Targets ≥2 Reps (Leapfrog)</label>
            <label class="flex items-center gap-1"><input type="checkbox" /> Cardio Discipline Satisfied</label>
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

