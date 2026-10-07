<script setup lang="ts">
import { ref, computed } from 'vue';
import { WorkoutSessionHistory } from '../types';

const props = defineProps<{
  history: WorkoutSessionHistory[];
}>();

const mode = ref<'level' | 'reps'>('level');
const hoveredPoint = ref<any | null>(null);

// Chronological sessions (oldest first for graphing)
const sortedSessions = computed(() => {
  return [...props.history].reverse();
});

// Graph geometry
const width = 800;
const height = 300;
const padding = { top: 30, right: 30, bottom: 40, left: 60 };

const chartWidth = width - padding.left - padding.right;
const chartHeight = height - padding.top - padding.bottom;

// Compute Level Points
const levelPoints = computed(() => {
  const sessions = sortedSessions.value;
  if (sessions.length === 0) return { strength: [], cardio: [], xLabels: [] };

  // Calculate scores: (chart * 12) + level
  const sScores = sessions.map((s) => s.strength_chart * 12 + s.strength_level);
  const cScores = sessions.map((s) => s.cardio_chart * 12 + s.cardio_level);

  const minScore = Math.max(1, Math.min(...sScores, ...cScores) - 2);
  const maxScore = Math.max(...sScores, ...cScores) + 2;
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

// Compute SVG polyline points
const strengthPolyline = computed(() => {
  return levelPoints.value.strength.map((p) => `${p.x},${p.y}`).join(' ');
});

const cardioPolyline = computed(() => {
  return levelPoints.value.cardio.map((p) => `${p.x},${p.y}`).join(' ');
});

// Rep Volume Points
const repsData = computed(() => {
  const sessions = sortedSessions.value;
  if (sessions.length === 0) return { ex1: [], ex2: [], ex3: [], ex4: [], maxReps: 50 };

  const allReps = sessions.flatMap((s) => [s.reps_1, s.reps_2, s.reps_3, s.reps_4]);
  const maxReps = Math.max(20, Math.max(...allReps) + 5);

  const stepX = sessions.length > 1 ? chartWidth / (sessions.length - 1) : chartWidth / 2;

  const createPoints = (getter: (s: WorkoutSessionHistory) => number, label: string) => {
    return sessions.map((s, idx) => {
      const val = getter(s);
      const x = padding.left + (sessions.length > 1 ? idx * stepX : chartWidth / 2);
      const y = padding.top + chartHeight - (val / maxReps) * chartHeight;
      return { x, y, val, label, session: s };
    });
  };

  return {
    ex1: createPoints((s) => s.reps_1, 'Ex 1: Bends'),
    ex2: createPoints((s) => s.reps_2, 'Ex 2: Sit-Ups'),
    ex3: createPoints((s) => s.reps_3, 'Ex 3: Arches'),
    ex4: createPoints((s) => s.reps_4, 'Ex 4: Push-Ups'),
    maxReps,
  };
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
  <div class="glass-panel rounded-2xl p-5 border border-slate-800 space-y-4">
    <!-- Header with Telemetry Switcher -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
      <div>
        <h3 class="text-base font-black text-white uppercase tracking-tight flex items-center gap-2">
          <span>📈</span>
          <span>Flight Telemetry & Progression Graph</span>
        </h3>
        <p class="text-xs text-slate-400">Historical performance curves across {{ history.length }} recorded sessions</p>
      </div>

      <!-- Mode Selector -->
      <div class="flex bg-slate-950/80 p-1 rounded-xl border border-slate-800">
        <button
          @click="mode = 'level'"
          class="px-3 py-1.5 text-xs font-bold rounded-lg transition-all cursor-pointer"
          :class="mode === 'level' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          Ladder Rungs
        </button>
        <button
          @click="mode = 'reps'"
          class="px-3 py-1.5 text-xs font-bold rounded-lg transition-all cursor-pointer"
          :class="mode === 'reps' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          Rep Volume
        </button>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="history.length < 2" class="p-8 text-center text-xs text-slate-400">
      Record at least two workout sessions to generate comparative flight telemetry curves.
    </div>

    <!-- SVG Graph Container -->
    <div v-else class="relative overflow-hidden bg-slate-950/80 rounded-xl border border-slate-800/80 p-2">
      <!-- Legend -->
      <div class="flex items-center justify-end gap-4 text-[11px] font-mono pr-4 pt-2">
        <template v-if="mode === 'level'">
          <div class="flex items-center gap-1.5">
            <span class="w-2.5 h-2.5 rounded-full bg-emerald-400"></span>
            <span class="text-slate-300">Strength Track</span>
          </div>
          <div class="flex items-center gap-1.5">
            <span class="w-2.5 h-2.5 rounded-full bg-cyan-400"></span>
            <span class="text-slate-300">Cardio Track</span>
          </div>
        </template>
        <template v-else>
          <div class="flex items-center gap-1.5">
            <span class="w-2 h-2 rounded-full bg-emerald-400"></span>
            <span class="text-slate-400">Ex 1</span>
          </div>
          <div class="flex items-center gap-1.5">
            <span class="w-2 h-2 rounded-full bg-cyan-400"></span>
            <span class="text-slate-400">Ex 2</span>
          </div>
          <div class="flex items-center gap-1.5">
            <span class="w-2 h-2 rounded-full bg-amber-400"></span>
            <span class="text-slate-400">Ex 3</span>
          </div>
          <div class="flex items-center gap-1.5">
            <span class="w-2 h-2 rounded-full bg-rose-400"></span>
            <span class="text-slate-400">Ex 4</span>
          </div>
        </template>
      </div>

      <svg :viewBox="`0 0 ${width} ${height}`" class="w-full h-auto select-none">
        <!-- Horizontal Grid Lines -->
        <line
          v-for="i in 5"
          :key="i"
          :x1="padding.left"
          :y1="padding.top + (chartHeight / 4) * (i - 1)"
          :x2="width - padding.right"
          :y2="padding.top + (chartHeight / 4) * (i - 1)"
          stroke="#1e293b"
          stroke-dasharray="4"
          stroke-width="1"
        />

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

        <!-- MODE 2: REPS PROGRESSION -->
        <g v-else>
          <!-- Ex 1 Line -->
          <polyline
            :points="repsData.ex1.map((p) => `${p.x},${p.y}`).join(' ')"
            fill="none"
            stroke="#10b981"
            stroke-width="2"
            stroke-dasharray="2"
          />
          <!-- Ex 2 Line -->
          <polyline
            :points="repsData.ex2.map((p) => `${p.x},${p.y}`).join(' ')"
            fill="none"
            stroke="#06b6d4"
            stroke-width="2"
          />
          <!-- Ex 3 Line -->
          <polyline
            :points="repsData.ex3.map((p) => `${p.x},${p.y}`).join(' ')"
            fill="none"
            stroke="#f59e0b"
            stroke-width="2"
          />
          <!-- Ex 4 Line -->
          <polyline
            :points="repsData.ex4.map((p) => `${p.x},${p.y}`).join(' ')"
            fill="none"
            stroke="#f43f5e"
            stroke-width="2"
          />

          <!-- Dots for Ex 2 (Sit-ups) & Ex 4 (Push-ups) -->
          <circle
            v-for="(p, i) in repsData.ex4"
            :key="'ex4-' + i"
            :cx="p.x"
            :cy="p.y"
            r="4"
            fill="#f43f5e"
            class="cursor-pointer"
            @mouseenter="hoveredPoint = { ...p, title: `Push-ups: ${p.val} reps` }"
            @mouseleave="hoveredPoint = null"
          />
        </g>

        <!-- X-Axis Labels (Dates) -->
        <text
          v-for="(s, idx) in sortedSessions"
          :key="'lbl-' + idx"
          :x="padding.left + (sortedSessions.length > 1 ? idx * (chartWidth / (sortedSessions.length - 1)) : chartWidth / 2)"
          :y="height - 10"
          text-anchor="middle"
          fill="#64748b"
          font-size="10"
          font-family="monospace"
        >
          <tspan v-if="idx === 0 || idx === sortedSessions.length - 1 || idx % Math.ceil(sortedSessions.length / 5) === 0">
            {{ formatDate(s.timestamp) }}
          </tspan>
        </text>
      </svg>

      <!-- Hover Tooltip Overlay -->
      <div
        v-if="hoveredPoint"
        class="absolute pointer-events-none bg-slate-900 border border-slate-700 p-2.5 rounded-xl shadow-xl text-xs z-20"
        :style="{ left: `${hoveredPoint.x}px`, top: `${Math.max(10, hoveredPoint.y - 60)}px` }"
      >
        <div class="font-bold text-white">{{ hoveredPoint.title }}</div>
        <div class="text-[10px] text-slate-400 font-mono">Date: {{ formatDate(hoveredPoint.session.timestamp) }}</div>
        <div class="text-[10px] text-emerald-400 font-mono">{{ hoveredPoint.session.overall_status }}</div>
      </div>
    </div>
  </div>
</template>
