<script setup lang="ts">
import { ref, computed } from 'vue';
import { WorkoutSessionHistory } from '../types';

const props = defineProps<{
  history: WorkoutSessionHistory[];
}>();

type GraphMode = 'level' | 'ex1' | 'ex2' | 'ex3' | 'ex4' | 'ex5';
const mode = ref<GraphMode>('level');
const hoveredPoint = ref<any | null>(null);

// Chronological sessions (oldest first for graphing)
const sortedSessions = computed(() => {
  return [...props.history].reverse();
});

// Graph geometry
const width = 800;
const height = 300;
const padding = { top: 30, right: 30, bottom: 45, left: 60 };

const chartWidth = width - padding.left - padding.right;
const chartHeight = height - padding.top - padding.bottom;

// Exercise metadata dictionary
const exerciseMeta: Record<string, { name: string; number: number; track: 'Strength' | 'Cardio'; unit: string; color: string }> = {
  ex1: { name: 'Forward Bends', number: 1, track: 'Strength', unit: 'reps', color: '#10b981' },
  ex2: { name: 'Sit-Ups', number: 2, track: 'Strength', unit: 'reps', color: '#10b981' },
  ex3: { name: 'Back Arches', number: 3, track: 'Strength', unit: 'reps', color: '#10b981' },
  ex4: { name: 'Push-Ups', number: 4, track: 'Strength', unit: 'reps', color: '#10b981' },
  ex5: { name: 'Cardio Track', number: 5, track: 'Cardio', unit: 'steps / duration', color: '#06b6d4' },
};

// 1. Level Progression Data (Ladder score: chart * 12 + level)
const levelPoints = computed(() => {
  const sessions = sortedSessions.value;
  if (sessions.length === 0) return { strength: [], cardio: [], minScore: 1, maxScore: 24 };

  const sScores = sessions.map((s) => s.strength_chart * 12 + s.strength_level);
  const cScores = sessions.map((s) => s.cardio_chart * 12 + s.cardio_level);

  const minScore = Math.max(1, Math.min(...sScores, ...cScores) - 1);
  const maxScore = Math.max(...sScores, ...cScores) + 1;
  const scoreRange = Math.max(1, maxScore - minScore);

  const stepX = sessions.length > 1 ? chartWidth / (sessions.length - 1) : chartWidth / 2;

  const strength = sessions.map((s, idx) => {
    const score = s.strength_chart * 12 + s.strength_level;
    const x = padding.left + (sessions.length > 1 ? idx * stepX : chartWidth / 2);
    const y = padding.top + chartHeight - ((score - minScore) / scoreRange) * chartHeight;
    return { x, y, session: s, score, type: 'Strength' };
  });

  const cardio = sessions.map((s, idx) => {
    const score = s.cardio_chart * 12 + s.cardio_level;
    const x = padding.left + (sessions.length > 1 ? idx * stepX : chartWidth / 2);
    const y = padding.top + chartHeight - ((score - minScore) / scoreRange) * chartHeight;
    return { x, y, session: s, score, type: 'Cardio' };
  });

  return { strength, cardio, minScore, maxScore };
});

const strengthPolyline = computed(() => {
  return levelPoints.value.strength.map((p) => `${p.x},${p.y}`).join(' ');
});

const cardioPolyline = computed(() => {
  return levelPoints.value.cardio.map((p) => `${p.x},${p.y}`).join(' ');
});

// 2. Individual Exercise Data Calculation
const singleExerciseData = computed(() => {
  const sessions = sortedSessions.value;
  if (sessions.length === 0 || mode.value === 'level') {
    return { points: [], polyline: '', areaPoints: '', minVal: 0, maxVal: 10, yTicks: [], best: 0, latest: 0, avg: 0 };
  }

  const exKey = mode.value;
  const isCardio = exKey === 'ex5';

  const rawValues = sessions.map((s) => {
    if (exKey === 'ex1') return s.reps_1;
    if (exKey === 'ex2') return s.reps_2;
    if (exKey === 'ex3') return s.reps_3;
    if (exKey === 'ex4') return s.reps_4;
    // For Cardio: stationary steps or duration
    return s.cardio_mode === 'stationary' ? s.reps_5 : s.cardio_duration_secs;
  });

  const minVal = Math.max(0, Math.min(...rawValues) - 2);
  const maxVal = Math.max(isCardio ? 50 : 10, Math.max(...rawValues) + 2);
  const valRange = Math.max(1, maxVal - minVal);

  const stepX = sessions.length > 1 ? chartWidth / (sessions.length - 1) : chartWidth / 2;

  const points = sessions.map((s, idx) => {
    const val = rawValues[idx];
    const x = padding.left + (sessions.length > 1 ? idx * stepX : chartWidth / 2);
    const y = padding.top + chartHeight - ((val - minVal) / valRange) * chartHeight;

    let displayVal = `${val} reps`;
    if (isCardio) {
      if (s.cardio_mode === 'stationary') {
        displayVal = `${val} steps`;
      } else {
        const m = Math.floor(val / 60);
        const sec = val % 60;
        displayVal = `${m}m ${sec}s (${s.cardio_mode})`;
      }
    }

    return { x, y, val, displayVal, session: s };
  });

  const polyline = points.map((p) => `${p.x},${p.y}`).join(' ');
  const areaPoints = points.length > 0
    ? `${points[0].x},${padding.top + chartHeight} ${polyline} ${points[points.length - 1].x},${padding.top + chartHeight}`
    : '';

  // Y-axis tick marks (5 ticks)
  const yTicks = [0, 1, 2, 3, 4].map((i) => {
    const val = Math.round(minVal + (valRange / 4) * i);
    const y = padding.top + chartHeight - (i * (chartHeight / 4));
    return { val, y };
  });

  // KPIs
  const best = Math.max(...rawValues);
  const latest = rawValues[rawValues.length - 1] || 0;
  const avg = Math.round(rawValues.reduce((a, b) => a + b, 0) / (rawValues.length || 1));

  return { points, polyline, areaPoints, minVal, maxVal, yTicks, best, latest, avg };
});

function scoreToRungDisplay(score: number): string {
  const chart = Math.floor(score / 12);
  const level = score % 12 || 12;
  const levelNames = ['D-', 'D', 'D+', 'C-', 'C', 'C+', 'B-', 'B', 'B+', 'A-', 'A', 'A+'];
  return `C${chart} ${levelNames[level - 1]}`;
}

function formatDate(iso: string) {
  try {
    const d = new Date(iso);
    return d.toLocaleDateString('en-GB', { day: 'numeric', month: 'short' });
  } catch {
    return iso;
  }
}
</script>

<template>
  <div class="glass-panel rounded-2xl p-5 border border-slate-800 space-y-5">
    <!-- Header with Track Filter Navigation -->
    <div class="flex flex-col lg:flex-row lg:items-center justify-between gap-4">
      <div>
        <div class="inline-flex items-center gap-2 px-2.5 py-0.5 rounded-full bg-cyan-950/80 border border-cyan-500/30 text-cyan-300 text-[10px] font-bold uppercase tracking-wider mb-1">
          <span>📈</span>
          <span>Mission Telemetry & Performance Curves</span>
        </div>
        <h3 class="text-lg font-black text-white uppercase tracking-tight">
          {{ mode === 'level' ? 'Overall Ladder Position Telemetry' : exerciseMeta[mode]?.name + ' Telemetry Curve' }}
        </h3>
        <p class="text-xs text-slate-400">
          Historical performance curves across {{ history.length }} recorded sessions
        </p>
      </div>

      <!-- Track Grouped Filter Pills -->
      <div class="flex flex-wrap items-center gap-1.5 bg-slate-950/90 p-1.5 rounded-2xl border border-slate-800">
        <!-- Overall Trajectory -->
        <button
          @click="mode = 'level'"
          class="px-3 py-1.5 text-xs font-bold rounded-xl transition-all cursor-pointer"
          :class="mode === 'level' ? 'bg-cyan-500 text-slate-950 shadow-md font-black' : 'text-slate-400 hover:text-white'"
        >
          Ladder Rungs
        </button>

        <span class="text-slate-700 hidden sm:inline">|</span>

        <!-- Strength Track Group (Green) -->
        <div class="flex items-center gap-1 bg-emerald-950/40 p-0.5 rounded-xl border border-emerald-500/30">
          <span class="text-[10px] font-black text-emerald-400 uppercase tracking-tight px-1.5 hidden md:inline">
            Strength:
          </span>
          <button
            @click="mode = 'ex1'"
            class="px-2 py-1 text-[11px] font-bold rounded-lg transition-all cursor-pointer"
            :class="mode === 'ex1' ? 'bg-emerald-500 text-slate-950 font-black' : 'text-slate-400 hover:text-emerald-300'"
            title="Exercise 1: Forward Bends"
          >
            1. Bends
          </button>
          <button
            @click="mode = 'ex2'"
            class="px-2 py-1 text-[11px] font-bold rounded-lg transition-all cursor-pointer"
            :class="mode === 'ex2' ? 'bg-emerald-500 text-slate-950 font-black' : 'text-slate-400 hover:text-emerald-300'"
            title="Exercise 2: Sit-Ups"
          >
            2. Sit-Ups
          </button>
          <button
            @click="mode = 'ex3'"
            class="px-2 py-1 text-[11px] font-bold rounded-lg transition-all cursor-pointer"
            :class="mode === 'ex3' ? 'bg-emerald-500 text-slate-950 font-black' : 'text-slate-400 hover:text-emerald-300'"
            title="Exercise 3: Back Arches"
          >
            3. Arches
          </button>
          <button
            @click="mode = 'ex4'"
            class="px-2 py-1 text-[11px] font-bold rounded-lg transition-all cursor-pointer"
            :class="mode === 'ex4' ? 'bg-emerald-500 text-slate-950 font-black' : 'text-slate-400 hover:text-emerald-300'"
            title="Exercise 4: Push-Ups"
          >
            4. Push-Ups
          </button>
        </div>

        <span class="text-slate-700 hidden sm:inline">|</span>

        <!-- Cardio Track Group (Cyan) -->
        <div class="flex items-center gap-1 bg-cyan-950/40 p-0.5 rounded-xl border border-cyan-500/30">
          <button
            @click="mode = 'ex5'"
            class="px-2.5 py-1 text-[11px] font-bold rounded-lg transition-all cursor-pointer flex items-center gap-1"
            :class="mode === 'ex5' ? 'bg-cyan-500 text-slate-950 font-black' : 'text-slate-400 hover:text-cyan-300'"
            title="Exercise 5: Cardio Conditioning"
          >
            <span>5. Cardio</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="history.length < 2" class="p-8 text-center text-xs text-slate-400 bg-slate-950/50 rounded-2xl border border-slate-800">
      Record at least two workout sessions to generate comparative flight telemetry curves.
    </div>

    <!-- Telemetry SVG Chart Container -->
    <div v-else class="relative overflow-hidden bg-slate-950/80 rounded-2xl border border-slate-800/80 p-3">
      <!-- Active Legend & Metrics Bar -->
      <div class="flex flex-wrap items-center justify-between gap-3 text-xs pb-2 mb-2 border-b border-slate-800/80">
        <!-- Left: Legend -->
        <div class="flex items-center gap-3 font-mono text-[11px]">
          <template v-if="mode === 'level'">
            <div class="flex items-center gap-1.5">
              <span class="w-2.5 h-2.5 rounded-full bg-emerald-400 shadow-sm shadow-emerald-500/50"></span>
              <span class="text-white font-bold">Strength Track (Ex 1–4)</span>
            </div>
            <div class="flex items-center gap-1.5">
              <span class="w-2.5 h-2.5 rounded-full bg-cyan-400 shadow-sm shadow-cyan-500/50"></span>
              <span class="text-white font-bold">Cardio Track (Ex 5)</span>
            </div>
          </template>
          <template v-else>
            <div class="flex items-center gap-1.5">
              <span
                class="w-2.5 h-2.5 rounded-full"
                :class="exerciseMeta[mode]?.track === 'Strength' ? 'bg-emerald-400 shadow-sm shadow-emerald-500/50' : 'bg-cyan-400 shadow-sm shadow-cyan-500/50'"
              ></span>
              <span class="text-white font-bold">{{ exerciseMeta[mode]?.name }}</span>
            </div>
          </template>
        </div>

        <!-- Right: Summary Statistics for Selected Exercise -->
        <div v-if="mode !== 'level'" class="flex items-center gap-3 font-mono text-[11px]">
          <span class="text-slate-400">
            Personal Best: <strong class="text-white">{{ singleExerciseData.best }}</strong>
          </span>
          <span class="text-slate-600">•</span>
          <span class="text-slate-400">
            Latest: <strong class="text-emerald-400">{{ singleExerciseData.latest }}</strong>
          </span>
          <span class="text-slate-600">•</span>
          <span class="text-slate-400">
            Cadence Avg: <strong class="text-cyan-400">{{ singleExerciseData.avg }}</strong>
          </span>
        </div>
      </div>

      <!-- SVG Drawing -->
      <svg :viewBox="`0 0 ${width} ${height}`" class="w-full h-auto select-none">
        <defs>
          <linearGradient id="strengthGradient" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stop-color="#10b981" stop-opacity="0.25" />
            <stop offset="100%" stop-color="#10b981" stop-opacity="0.0" />
          </linearGradient>
          <linearGradient id="cardioGradient" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stop-color="#06b6d4" stop-opacity="0.25" />
            <stop offset="100%" stop-color="#06b6d4" stop-opacity="0.0" />
          </linearGradient>
        </defs>

        <!-- Horizontal Grid Lines -->
        <line
          v-for="i in 5"
          :key="'grid-' + i"
          :x1="padding.left"
          :y1="padding.top + (chartHeight / 4) * (i - 1)"
          :x2="width - padding.right"
          :y2="padding.top + (chartHeight / 4) * (i - 1)"
          stroke="#1e293b"
          stroke-dasharray="3 3"
          stroke-width="1"
        />

        <!-- Y-Axis Tick Labels for Individual Exercise -->
        <template v-if="mode !== 'level'">
          <text
            v-for="t in singleExerciseData.yTicks"
            :key="'ytick-' + t.val"
            :x="padding.left - 10"
            :y="t.y + 4"
            text-anchor="end"
            fill="#64748b"
            font-size="10"
            font-family="monospace"
          >
            {{ t.val }}
          </text>
        </template>

        <!-- MODE 1: LADDER LEVEL PROGRESSION -->
        <g v-if="mode === 'level'">
          <!-- Strength Polyline & Points -->
          <polyline
            :points="strengthPolyline"
            fill="none"
            stroke="#10b981"
            stroke-width="3"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <circle
            v-for="(p, i) in levelPoints.strength"
            :key="'s-' + i"
            :cx="p.x"
            :cy="p.y"
            r="4.5"
            fill="#10b981"
            class="transition-all hover:r-6 cursor-pointer"
            @mouseenter="hoveredPoint = { ...p, title: `Strength: C${p.session.strength_chart} ${scoreToRungDisplay(p.score)}` }"
            @mouseleave="hoveredPoint = null"
          />

          <!-- Cardio Polyline & Points -->
          <polyline
            :points="cardioPolyline"
            fill="none"
            stroke="#06b6d4"
            stroke-width="3"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <circle
            v-for="(p, i) in levelPoints.cardio"
            :key="'c-' + i"
            :cx="p.x"
            :cy="p.y"
            r="4.5"
            fill="#06b6d4"
            class="transition-all hover:r-6 cursor-pointer"
            @mouseenter="hoveredPoint = { ...p, title: `Cardio: C${p.session.cardio_chart} ${scoreToRungDisplay(p.score)}` }"
            @mouseleave="hoveredPoint = null"
          />
        </g>

        <!-- MODE 2: INDIVIDUAL EXERCISE PERFORMANCE CURVE -->
        <g v-else>
          <!-- Gradient Area Fill -->
          <polygon
            :points="singleExerciseData.areaPoints"
            :fill="exerciseMeta[mode]?.track === 'Strength' ? 'url(#strengthGradient)' : 'url(#cardioGradient)'"
          />

          <!-- Main Polyline -->
          <polyline
            :points="singleExerciseData.polyline"
            fill="none"
            :stroke="exerciseMeta[mode]?.color || '#10b981'"
            stroke-width="3.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          />

          <!-- Interactive Data Points -->
          <circle
            v-for="(p, i) in singleExerciseData.points"
            :key="'pt-' + i"
            :cx="p.x"
            :cy="p.y"
            r="5"
            :fill="exerciseMeta[mode]?.color || '#10b981'"
            stroke="#020617"
            stroke-width="2"
            class="transition-all hover:r-7 cursor-pointer"
            @mouseenter="hoveredPoint = { ...p, title: `${exerciseMeta[mode]?.name}: ${p.displayVal}` }"
            @mouseleave="hoveredPoint = null"
          />
        </g>

        <!-- X-Axis Labels (Dates) -->
        <text
          v-for="(s, idx) in sortedSessions"
          :key="'lbl-' + idx"
          :x="padding.left + (sortedSessions.length > 1 ? idx * (chartWidth / (sortedSessions.length - 1)) : chartWidth / 2)"
          :y="height - 15"
          text-anchor="middle"
          fill="#64748b"
          font-size="10"
          font-family="monospace"
        >
          <tspan v-if="idx === 0 || idx === sortedSessions.length - 1 || idx % Math.ceil(sortedSessions.length / 6) === 0">
            {{ formatDate(s.timestamp) }}
          </tspan>
        </text>
      </svg>

      <!-- Hover Tooltip Overlay -->
      <div
        v-if="hoveredPoint"
        class="absolute pointer-events-none bg-slate-900/95 border border-slate-700 p-2.5 rounded-xl shadow-2xl text-xs z-20 backdrop-blur-md"
        :style="{ left: `${hoveredPoint.x}px`, top: `${Math.max(10, hoveredPoint.y - 70)}px` }"
      >
        <div class="font-bold text-white">{{ hoveredPoint.title }}</div>
        <div class="text-[10px] text-slate-400 font-mono">Date: {{ formatDate(hoveredPoint.session.timestamp) }}</div>
        <div class="text-[10px] text-emerald-400 font-mono mt-0.5">Status: {{ hoveredPoint.session.overall_status }}</div>
      </div>
    </div>
  </div>
</template>
