<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { UserProfile, ExerciseChartRow, ExerciseInstructionRow } from '../types';
import { fetchSystemCharts } from '../api';
import { calculateTreadmillSpeed } from '../telemetry';

const props = defineProps<{
  profile: UserProfile | null;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

const selectedChart = ref<number>(props.profile?.strength_chart || 1);
const allCharts = ref<ExerciseChartRow[]>([]);
const allInstructions = ref<ExerciseInstructionRow[]>([]);
const loading = ref(true);
const selectedExerciseDetail = ref<ExerciseInstructionRow | null>(null);

const levelNames = ['D-', 'D', 'D+', 'C-', 'C', 'C+', 'B-', 'B', 'B+', 'A-', 'A', 'A+'];

onMounted(async () => {
  try {
    const data = await fetchSystemCharts();
    allCharts.value = data.charts;
    allInstructions.value = data.instructions;
    if (props.profile?.strength_chart) {
      selectedChart.value = props.profile.strength_chart;
    }
  } catch (e) {
    console.error('Failed to load system charts:', e);
  } finally {
    loading.value = false;
  }
});

const currentChartRows = computed(() => {
  return allCharts.value.filter((r) => r.chart === selectedChart.value);
});

const currentChartInstructions = computed(() => {
  return allInstructions.value.filter((i) => i.chart === selectedChart.value);
});

const strengthInstructions = computed(() => {
  return currentChartInstructions.value.filter((i) => i.exercise <= 4);
});

const cardioInstructions = computed(() => {
  return currentChartInstructions.value.filter((i) => i.exercise >= 5);
});


function getLevelLabel(level: number): string {
  return levelNames[level - 1] || 'D-';
}

function formatDuration(seconds: number): string {
  if (!seconds || seconds <= 0) return '--';
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return `${m}:${s.toString().padStart(2, '0')}`;
}

function getRunSpeed(seconds: number): string {
  if (!seconds || seconds <= 0) return '';
  const dist = selectedChart.value === 1 ? 0.5 : 1.0;
  const speed = calculateTreadmillSpeed(dist, seconds);
  return `≥ ${speed.kph.toFixed(1)} km/h (${speed.mph.toFixed(1)} mph)`;
}

function getWalkSpeed(seconds: number): string {
  if (!seconds || seconds <= 0) return '';
  const dist = selectedChart.value === 1 ? 1.0 : 2.0;
  const speed = calculateTreadmillSpeed(dist, seconds);
  return `≥ ${speed.kph.toFixed(1)} km/h (${speed.mph.toFixed(1)} mph)`;
}

function isCurrentStrength(level: number): boolean {
  if (!props.profile) return false;
  return props.profile.strength_chart === selectedChart.value && props.profile.strength_level === level;
}

function isCurrentCardio(level: number): boolean {
  if (!props.profile) return false;
  return props.profile.cardio_chart === selectedChart.value && props.profile.cardio_level === level;
}

function showExercise(exNum: number) {
  const instr = currentChartInstructions.value.find((i) => i.exercise === exNum);
  if (instr) {
    selectedExerciseDetail.value = instr;
  }
}
</script>

<template>
  <div class="max-w-6xl mx-auto px-4 py-8 space-y-6">
    <!-- Header Section -->
    <div class="space-y-4">
      <div>
        <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-950/80 border border-cyan-500/30 text-cyan-300 text-xs font-bold uppercase tracking-wider mb-2">
          <span>✈️</span>
          <span>Royal Canadian Air Force 5BX System Reference</span>
        </div>
        <h2 class="text-3xl font-black text-white uppercase tracking-tight">Full System Ladder Matrix</h2>
        <p class="text-sm text-slate-400">
          Six progressive charts containing 12 calibrated rungs each (72 rungs total). Complete all targets in your chart to advance.
        </p>
      </div>

      <!-- Dedicated Responsive Chart Selector Segmented Control -->
      <div class="bg-slate-900/90 p-1.5 rounded-2xl border border-slate-800 shadow-lg">
        <div class="grid grid-cols-6 gap-1 sm:gap-1.5">
          <button
            v-for="c in [1, 2, 3, 4, 5, 6]"
            :key="c"
            @click="selectedChart = c"
            class="py-2.5 px-1.5 sm:px-3 text-xs font-black rounded-xl transition-all cursor-pointer text-center flex items-center justify-center gap-1 select-none"
            :class="selectedChart === c
              ? 'bg-cyan-500 text-slate-950 shadow-md shadow-cyan-500/25 font-black ring-1 ring-cyan-400'
              : 'text-slate-400 hover:text-white hover:bg-slate-800/80'"
            :aria-label="'Chart ' + c"
          >
            <span class="hidden sm:inline text-[11px] uppercase tracking-wider opacity-80 font-bold">Chart</span>
            <span class="text-sm sm:text-xs font-black">{{ c }}</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Active Chart Legend & Pilot Position Indicator -->
    <div v-if="profile" class="flex flex-wrap items-center gap-4 bg-slate-950/80 p-4 rounded-2xl border border-slate-800 text-xs text-slate-300">
      <span class="font-bold text-slate-400 uppercase tracking-wider">Pilot Position Legend:</span>
      <div class="flex items-center gap-2">
        <span class="w-3 h-3 rounded-full bg-emerald-400 ring-2 ring-emerald-500/40"></span>
        <span>Current Strength Level: <strong class="text-emerald-400 font-mono">Chart {{ profile.strength_chart }} {{ profile.strength_level_display }}</strong></span>
      </div>
      <div class="flex items-center gap-2">
        <span class="w-3 h-3 rounded-full bg-cyan-400 ring-2 ring-cyan-500/40"></span>
        <span>Current Cardio Level: <strong class="text-cyan-400 font-mono">Chart {{ profile.cardio_chart }} {{ profile.cardio_level_display }}</strong></span>
      </div>
    </div>

    <!-- Movement Overview: Split into Dedicated Strength (Emerald) and Cardio (Cyan) Rows -->
    <div class="space-y-6">
      <!-- Row 1: Strength Disciplines (Movements 1 to 4 - Emerald Green) -->
      <div class="space-y-2.5">
        <div class="flex items-center justify-between px-1">
          <div class="flex items-center gap-2">
            <span class="w-2.5 h-2.5 rounded-full bg-emerald-400 ring-2 ring-emerald-500/40"></span>
            <h3 class="text-xs font-black uppercase tracking-wider text-emerald-300">Strength Track (Movements 1 to 4)</h3>
            <span class="hidden sm:inline text-[11px] text-slate-400">• Calisthenics &amp; core mobility</span>
          </div>
          <span class="text-[10px] font-mono text-emerald-400/90 bg-emerald-950/60 border border-emerald-500/30 px-2.5 py-0.5 rounded-md">
            5 Min Allocation (Ex 1–4)
          </span>
        </div>

        <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
          <div
            v-for="instr in strengthInstructions"
            :key="instr.exercise"
            @click="selectedExerciseDetail = instr"
            class="bg-slate-900/80 hover:bg-emerald-950/30 border border-emerald-500/30 hover:border-emerald-400/70 rounded-xl p-3 cursor-pointer flex flex-col items-center text-center group transition-all shadow-lg hover:shadow-emerald-950/30"
          >
            <div class="w-full h-16 bg-white rounded-lg p-1.5 flex items-center justify-center mb-2 shadow-inner group-hover:scale-[1.02] transition-transform">
              <img :src="'/images/' + instr.image_path" :alt="instr.name" class="max-h-full max-w-full object-contain" />
            </div>
            <div class="text-[10px] font-bold text-emerald-400 uppercase tracking-wider">Ex {{ instr.exercise }} • Info</div>
            <div class="text-xs font-bold text-white group-hover:text-emerald-300 truncate w-full" :title="instr.name">{{ instr.name }}</div>
          </div>
        </div>
      </div>

      <!-- Row 2: Cardio Disciplines (Movements 5 to 7 - Cyan) -->
      <div class="space-y-2.5">
        <div class="flex items-center justify-between px-1">
          <div class="flex items-center gap-2">
            <span class="w-2.5 h-2.5 rounded-full bg-cyan-400 ring-2 ring-cyan-500/40"></span>
            <h3 class="text-xs font-black uppercase tracking-wider text-cyan-300">Cardio Track (Aerobic Disciplines)</h3>
            <span class="hidden sm:inline text-[11px] text-slate-400">• Stationary run or outdoor alternatives</span>
          </div>
          <span class="text-[10px] font-mono text-cyan-400/90 bg-cyan-950/60 border border-cyan-500/30 px-2.5 py-0.5 rounded-md">
            6 Min Allocation (Ex 5) or Timed Road Work
          </span>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
          <div
            v-for="instr in cardioInstructions"
            :key="instr.exercise"
            @click="selectedExerciseDetail = instr"
            class="bg-slate-900/80 hover:bg-cyan-950/30 border border-cyan-500/30 hover:border-cyan-400/70 rounded-xl p-3 cursor-pointer flex flex-col items-center text-center group transition-all shadow-lg hover:shadow-cyan-950/30"
          >
            <div class="w-full h-16 bg-white rounded-lg p-1.5 flex items-center justify-center mb-2 shadow-inner group-hover:scale-[1.02] transition-transform">
              <img :src="'/images/' + instr.image_path" :alt="instr.name" class="max-h-full max-w-full object-contain" />
            </div>
            <div class="text-[10px] font-bold text-cyan-400 uppercase tracking-wider">
              Ex {{ instr.exercise }} • {{ instr.exercise === 5 ? 'Standard Aerobic' : 'Outdoor Alternative' }}
            </div>
            <div class="text-xs font-bold text-white group-hover:text-cyan-300 truncate w-full" :title="instr.name">{{ instr.name }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- The 12-Rung Matrix Table with Grouped Strength & Cardio Headers -->
    <div class="glass-panel rounded-2xl overflow-hidden shadow-2xl border border-slate-800">
      <div class="overflow-x-auto pb-3">
        <table class="w-full text-left text-xs tabular-nums border-collapse">
          <thead>
            <!-- Group Tier Header -->
            <tr class="bg-slate-950/95 border-b border-slate-800 text-[10px] tracking-wider uppercase">
              <th rowspan="2" class="py-3 px-4 font-black text-slate-300 border-r border-slate-800 align-middle text-center w-16">
                Rung
              </th>
              <th colspan="4" class="py-2.5 px-3 text-center font-black bg-emerald-950/50 text-emerald-300 border-b border-r border-emerald-500/30">
                Strength Disciplines (Movements 1 to 4 • Emerald)
              </th>
              <th colspan="3" class="py-2.5 px-3 text-center font-black bg-cyan-950/50 text-cyan-300 border-b border-cyan-500/30">
                Cardio Disciplines (Aerobic Capacity • Cyan)
              </th>
            </tr>
            <!-- Individual Column Headers -->
            <tr class="bg-slate-950/90 border-b border-slate-800 text-slate-400 uppercase tracking-wider text-[10px]">
              <th class="py-2.5 px-3 font-bold text-emerald-400 cursor-pointer hover:text-emerald-300 transition-colors" @click="showExercise(1)">
                Ex 1: Stretch
              </th>
              <th class="py-2.5 px-3 font-bold text-emerald-400 cursor-pointer hover:text-emerald-300 transition-colors" @click="showExercise(2)">
                Ex 2: Sit-Up
              </th>
              <th class="py-2.5 px-3 font-bold text-emerald-400 cursor-pointer hover:text-emerald-300 transition-colors" @click="showExercise(3)">
                Ex 3: Back Arch
              </th>
              <th class="py-2.5 px-3 font-bold text-emerald-400 cursor-pointer hover:text-emerald-300 transition-colors border-r border-slate-800" @click="showExercise(4)">
                Ex 4: Push-Up
              </th>
              <th class="py-2.5 px-4 font-bold text-cyan-400 cursor-pointer hover:text-cyan-300 transition-colors" @click="showExercise(5)">
                Ex 5: Stationary Run
              </th>
              <th class="py-2.5 px-3 font-bold text-cyan-300 cursor-pointer hover:text-cyan-200 transition-colors" @click="showExercise(6)">
                <div>{{ selectedChart === 1 ? 'Run 0.5 mi' : 'Run 1.0 mi' }}</div>
                <div class="text-[9px] font-normal text-slate-400">({{ selectedChart === 1 ? '0.8 km' : '1.6 km' }})</div>
              </th>
              <th class="py-2.5 px-3 font-bold text-cyan-300 cursor-pointer hover:text-cyan-200 transition-colors" @click="showExercise(7)">
                <div>{{ selectedChart === 1 ? 'Walk 1.0 mi' : (selectedChart >= 3 ? 'Jog 2.0 mi' : 'Walk 2.0 mi') }}</div>
                <div class="text-[9px] font-normal text-slate-400">({{ selectedChart === 1 ? '1.6 km' : '3.2 km' }})</div>
              </th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-800/60 font-mono">
            <tr
              v-for="row in currentChartRows"
              :key="row.id"
              class="transition-colors hover:bg-slate-800/40"
              :class="{
                'bg-emerald-950/30 border-l-4 border-l-emerald-400': isCurrentStrength(row.level) && !isCurrentCardio(row.level),
                'bg-cyan-950/30 border-l-4 border-l-cyan-400': isCurrentCardio(row.level) && !isCurrentStrength(row.level),
                'bg-gradient-to-r from-emerald-950/40 to-cyan-950/40 border-l-4 border-l-teal-400': isCurrentStrength(row.level) && isCurrentCardio(row.level),
              }"
            >
              <!-- Level Label -->
              <td class="py-3 px-4 font-black text-sm text-white flex items-center justify-between border-r border-slate-800/80">
                <span>{{ getLevelLabel(row.level) }}</span>
                <div class="flex items-center gap-1">
                  <span v-if="isCurrentStrength(row.level)" class="text-[9px] bg-emerald-500 text-slate-950 px-1.5 py-0.5 rounded font-black font-sans shadow-sm" title="Your Current Strength Rung">
                    S
                  </span>
                  <span v-if="isCurrentCardio(row.level)" class="text-[9px] bg-cyan-400 text-slate-950 px-1.5 py-0.5 rounded font-black font-sans shadow-sm" title="Your Current Cardio Rung">
                    C
                  </span>
                </div>
              </td>

              <!-- Strength Targets (Movements 1 to 4) -->
              <td class="py-3 px-3 text-slate-200 font-semibold">{{ row.ex1 }}</td>
              <td class="py-3 px-3 text-slate-200 font-semibold">{{ row.ex2 }}</td>
              <td class="py-3 px-3 text-slate-200 font-semibold">{{ row.ex3 }}</td>
              <td class="py-3 px-3 text-slate-200 font-semibold border-r border-slate-800/80">{{ row.ex4 }}</td>

              <!-- Cardio Targets (Movements 5 to 7) -->
              <td class="py-3 px-4 font-bold text-cyan-300">
                {{ row.ex5 }}
                <span v-if="selectedChart >= 5" class="text-[10px] text-slate-400 font-sans">+ jumps</span>
              </td>
              <td class="py-3 px-3 text-cyan-400/90 font-mono" :title="row.ex5_run > 0 ? 'Treadmill Speed: ' + getRunSpeed(row.ex5_run) : ''">
                <div>{{ formatDuration(row.ex5_run) }}</div>
                <div v-if="row.ex5_run > 0" class="text-[9px] font-sans text-slate-500 hidden sm:block">
                  {{ getRunSpeed(row.ex5_run) }}
                </div>
              </td>
              <td class="py-3 px-3 text-cyan-400/90 font-mono" :title="row.ex5_walk > 0 ? 'Treadmill Speed: ' + getWalkSpeed(row.ex5_walk) : ''">
                <div>{{ formatDuration(row.ex5_walk) }}</div>
                <div v-if="row.ex5_walk > 0" class="text-[9px] font-sans text-slate-500 hidden sm:block">
                  {{ getWalkSpeed(row.ex5_walk) }}
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Historical RCAF Roadwork Calibration Note -->
    <div class="bg-slate-900/60 border border-slate-800 rounded-2xl p-4 sm:p-5 text-xs text-slate-300 space-y-2">
      <div class="flex items-center gap-2 font-bold text-cyan-300">
        <span>⏱️</span>
        <span class="uppercase tracking-wider text-[11px]">RCAF Doctrine: Aerobic Substitution Calibration</span>
      </div>
      <p class="text-slate-400 leading-relaxed">
        <template v-if="selectedChart <= 4">
          In <strong>Charts 1 to 4</strong>, the official RCAF standard calibrates continuous walking roadwork in <strong>whole-minute targets</strong> (17 to 35 minutes) and running roadwork into <strong>quarter- and half-minute brackets</strong> across letter tiers (D, C, B, A).
        </template>
        <template v-else>
          In <strong>Charts 5 and 6</strong> (Flying Crew & elite standards), the RCAF discontinued walking roadwork and calibrated 1-mile running ceilings into <strong>discrete second-by-second benchmarks</strong> for every individual rung down to 5:00.
        </template>
      </p>
    </div>

    <!-- Exercise Posture / Technique Modal Drawer with Contextual Theming -->
    <div v-if="selectedExerciseDetail" class="fixed inset-0 z-50 bg-slate-950/85 backdrop-blur-md flex items-center justify-center p-4">
      <div
        class="bg-slate-900 border rounded-3xl w-full max-w-lg p-6 sm:p-8 shadow-2xl relative"
        :class="selectedExerciseDetail.exercise <= 4 ? 'border-emerald-500/40 shadow-emerald-950/30' : 'border-cyan-500/40 shadow-cyan-950/30'"
      >
        <div class="flex justify-between items-center pb-3 border-b border-slate-800 mb-4">
          <div>
            <span
              class="text-[10px] font-bold uppercase tracking-wider block"
              :class="selectedExerciseDetail.exercise <= 4 ? 'text-emerald-400' : 'text-cyan-400'"
            >
              Chart {{ selectedExerciseDetail.chart }} • Movement {{ selectedExerciseDetail.exercise }} ({{ selectedExerciseDetail.exercise <= 4 ? 'Strength Discipline' : 'Cardio Discipline' }})
            </span>
            <h3 class="text-xl font-black text-white">{{ selectedExerciseDetail.name }}</h3>
          </div>
          <button
            @click="selectedExerciseDetail = null"
            class="w-8 h-8 rounded-full bg-slate-800 text-slate-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
          >
            ✕
          </button>
        </div>

        <div class="w-full h-44 bg-white rounded-2xl p-3 flex items-center justify-center shadow-inner mb-4 border border-slate-300">
          <img :src="'/images/' + selectedExerciseDetail.image_path" :alt="selectedExerciseDetail.name" class="max-h-full max-w-full object-contain" />
        </div>

        <div class="bg-slate-950/90 p-4 rounded-xl border border-slate-800 text-xs text-slate-300 leading-relaxed mb-5 whitespace-pre-line">
          {{ selectedExerciseDetail.instructions }}
        </div>

        <button
          @click="selectedExerciseDetail = null"
          class="w-full btn-control-primary"
          :class="selectedExerciseDetail.exercise <= 4 ? 'bg-emerald-500 hover:bg-emerald-400 text-slate-950' : 'bg-cyan-500 hover:bg-cyan-400 text-slate-950'"
        >
          Close Movement Details
        </button>
      </div>
    </div>
  </div>
</template>
