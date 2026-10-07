<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { UserProfile, ExerciseChartRow, ExerciseInstructionRow } from '../types';
import { fetchSystemCharts } from '../api';

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

function getLevelLabel(level: number): string {
  return levelNames[level - 1] || 'D-';
}

function formatDuration(seconds: number): string {
  if (!seconds || seconds <= 0) return '--';
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return `${m}:${s.toString().padStart(2, '0')}`;
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
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
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

      <!-- Chart Selector Tabs -->
      <div class="flex flex-wrap gap-1.5 bg-slate-900/90 p-1.5 rounded-2xl border border-slate-800">
        <button
          v-for="c in [1, 2, 3, 4, 5, 6]"
          :key="c"
          @click="selectedChart = c"
          class="px-4 py-2 text-xs font-black rounded-xl transition-all cursor-pointer"
          :class="selectedChart === c ? 'bg-cyan-500 text-slate-950 shadow-md shadow-cyan-500/20' : 'text-slate-400 hover:text-white'"
        >
          Chart {{ c }}
        </button>
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

    <!-- Movement Header Thumbnails Bar (Clickable for details) -->
    <div class="grid grid-cols-2 sm:grid-cols-5 gap-3">
      <div
        v-for="instr in currentChartInstructions"
        :key="instr.exercise"
        @click="selectedExerciseDetail = instr"
        class="glass-panel glass-panel-hover rounded-xl p-3 cursor-pointer flex flex-col items-center text-center group"
      >
        <div class="w-full h-16 bg-white rounded-lg p-1.5 flex items-center justify-center mb-2 shadow-inner">
          <img :src="'/images/' + instr.image_path" :alt="instr.name" class="max-h-full max-w-full object-contain" />
        </div>
        <div class="text-[10px] font-bold text-cyan-400 uppercase tracking-wider">Ex {{ instr.exercise }} (Click info)</div>
        <div class="text-xs font-bold text-white group-hover:text-cyan-300 truncate w-full">{{ instr.name }}</div>
      </div>
    </div>

    <!-- The 12-Rung Matrix Table -->
    <div class="glass-panel rounded-2xl overflow-hidden shadow-2xl border border-slate-800">
      <div class="overflow-x-auto">
        <table class="w-full text-left text-xs tabular-nums border-collapse">
          <thead>
            <tr class="bg-slate-950/90 border-b border-slate-800 text-slate-400 uppercase tracking-wider text-[10px]">
              <th class="py-3.5 px-4 font-bold">Rung</th>
              <th class="py-3.5 px-3 font-bold text-emerald-400 cursor-pointer" @click="showExercise(1)">Ex 1: Stretch</th>
              <th class="py-3.5 px-3 font-bold text-emerald-400 cursor-pointer" @click="showExercise(2)">Ex 2: Sit-Up</th>
              <th class="py-3.5 px-3 font-bold text-emerald-400 cursor-pointer" @click="showExercise(3)">Ex 3: Back Arch</th>
              <th class="py-3.5 px-3 font-bold text-emerald-400 cursor-pointer" @click="showExercise(4)">Ex 4: Push-Up</th>
              <th class="py-3.5 px-4 font-bold text-cyan-400 cursor-pointer" @click="showExercise(5)">Ex 5: Stationary Run</th>
              <th class="py-3.5 px-3 font-bold text-cyan-300">Alt 1-Mi Run</th>
              <th class="py-3.5 px-3 font-bold text-cyan-300">Alt 2-Mi Walk</th>
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
              <td class="py-3 px-4 font-black text-sm text-white flex items-center gap-2">
                <span>{{ getLevelLabel(row.level) }}</span>
                <span v-if="isCurrentStrength(row.level)" class="text-[9px] bg-emerald-500 text-slate-950 px-1.5 py-0.5 rounded font-black font-sans">
                  S
                </span>
                <span v-if="isCurrentCardio(row.level)" class="text-[9px] bg-cyan-400 text-slate-950 px-1.5 py-0.5 rounded font-black font-sans">
                  C
                </span>
              </td>

              <!-- Exercise Targets -->
              <td class="py-3 px-3 text-slate-200 font-semibold">{{ row.ex1 }}</td>
              <td class="py-3 px-3 text-slate-200 font-semibold">{{ row.ex2 }}</td>
              <td class="py-3 px-3 text-slate-200 font-semibold">{{ row.ex3 }}</td>
              <td class="py-3 px-3 text-slate-200 font-semibold">{{ row.ex4 }}</td>
              <td class="py-3 px-4 font-bold text-cyan-300">
                {{ row.ex5 }}
                <span v-if="selectedChart >= 5" class="text-[10px] text-slate-400 font-sans">+ jumps</span>
              </td>
              <td class="py-3 px-3 text-slate-400">{{ formatDuration(row.ex5_run) }}</td>
              <td class="py-3 px-3 text-slate-400">{{ formatDuration(row.ex5_walk) }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Exercise Posture / Technique Modal Drawer -->
    <div v-if="selectedExerciseDetail" class="fixed inset-0 z-50 bg-slate-950/85 backdrop-blur-md flex items-center justify-center p-4">
      <div class="bg-slate-900 border border-slate-700/80 rounded-3xl w-full max-w-lg p-6 sm:p-8 shadow-2xl relative">
        <div class="flex justify-between items-center pb-3 border-b border-slate-800 mb-4">
          <div>
            <span class="text-[10px] font-bold text-cyan-400 uppercase tracking-wider block">
              Chart {{ selectedExerciseDetail.chart }} • Movement {{ selectedExerciseDetail.exercise }}
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
          class="w-full btn-control-primary bg-cyan-500 hover:bg-cyan-400 text-slate-950"
        >
          Close Movement Details
        </button>
      </div>
    </div>
  </div>
</template>
