<script setup lang="ts">
import { WorkoutSessionHistory } from '../types';
import ProgressionGraph from './ProgressionGraph.vue';

defineProps<{
  history: WorkoutSessionHistory[];
}>();

const emit = defineEmits<{
  (e: 'inspect', session: WorkoutSessionHistory): void;
}>();

function formatDateTime(iso: string) {
  try {
    const d = new Date(iso);
    return d.toLocaleDateString('en-GB', { day: 'numeric', month: 'short', year: 'numeric', hour: '2-digit', minute: '2-digit' });
  } catch {
    return iso;
  }
}

function formatDuration(seconds: number): string {
  if (!seconds || seconds <= 0) return '0:00';
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return `${m}:${s.toString().padStart(2, '0')}`;
}

function cleanStrengthVerdict(v: string | null | undefined): string {
  if (!v) return '';
  const compound = v.match(/Strength\s*\((.+?)\)\s*(?:\||\/)\s*Cardio\s*\((.*)\)\s*$/i);
  if (compound) {
    return compound[1].trim();
  }
  const manual = v.match(/MANUAL SET:\s*S\((.*?)\)\s*\|\s*C\((.*?)\)/i);
  if (manual) {
    return `Manual Set: ${manual[1].trim()}`;
  }
  return v.trim();
}

function cleanCardioVerdict(v: string | null | undefined): string {
  if (!v) return '';
  const compound = v.match(/Strength\s*\((.+?)\)\s*(?:\||\/)\s*Cardio\s*\((.*)\)\s*$/i);
  if (compound) {
    return compound[2].trim();
  }
  const manual = v.match(/MANUAL SET:\s*S\((.*?)\)\s*\|\s*C\((.*?)\)/i);
  if (manual) {
    return `Manual Set: ${manual[2].trim()}`;
  }
  return v.trim();
}
</script>

<template>
  <div class="max-w-5xl mx-auto px-4 py-8 space-y-6">
    <!-- Header Section -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
      <div>
        <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-950/80 border border-cyan-500/30 text-cyan-300 text-xs font-bold uppercase tracking-wider mb-2">
          <span>📜</span>
          <span>Pilot Flight Log & Telemetry</span>
        </div>
        <h2 class="text-3xl font-black text-white uppercase tracking-tight">Recorded Missions</h2>
        <p class="text-xs text-slate-400">Complete performance ledger and progression telemetry curves.</p>
      </div>
    </div>

    <!-- Telemetry Graph Component -->
    <ProgressionGraph :history="history" />

    <!-- History Session Log Cards -->
    <div v-if="history.length === 0" class="glass-panel rounded-2xl p-12 text-center text-slate-400 text-sm">
      No workout sessions recorded yet. Start your first 11-minute mission today!
    </div>

    <div v-else class="space-y-3">
      <div class="flex items-center justify-between text-xs font-bold text-slate-400 uppercase tracking-wider px-2">
        <span>Historical Mission Debriefs ({{ history.length }})</span>
      </div>

      <div
        v-for="s in history"
        :key="s.id"
        class="glass-panel glass-panel-hover rounded-2xl p-5 flex flex-col md:flex-row justify-between items-start md:items-center gap-4 transition-all cursor-pointer group"
        @click="emit('inspect', s)"
      >
        <div class="flex-1">
          <div class="flex items-center gap-2 mb-2">
            <span class="text-xs font-mono text-slate-400">{{ formatDateTime(s.timestamp) }}</span>
            <span
              class="text-[10px] font-bold px-2 py-0.5 rounded-full uppercase tracking-wider font-mono"
              :class="s.overall_status === 'Promoted' ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/40' : (s.overall_status === 'Demoted' ? 'bg-red-500/20 text-red-300 border border-red-500/40' : 'bg-slate-800 text-slate-300 border border-slate-700')"
            >
              {{ s.overall_status }}
            </span>
          </div>

          <div class="flex flex-wrap items-center gap-3 text-sm font-bold text-white mb-1">
            <span class="text-emerald-400">Strength: {{ cleanStrengthVerdict(s.verdict_strength) }}</span>
            <span class="text-slate-600">•</span>
            <span class="text-cyan-400">Cardio: {{ cleanCardioVerdict(s.verdict_cardio) }}</span>
          </div>

          <div class="text-xs text-slate-300 flex flex-wrap gap-x-4 gap-y-1 font-mono mt-2 bg-slate-950/60 p-2.5 rounded-xl border border-slate-800/80">
            <span>Bends: <strong class="text-white">{{ s.reps_1 }}</strong></span>
            <span>Sit-Ups: <strong class="text-white">{{ s.reps_2 }}</strong></span>
            <span>Arches: <strong class="text-white">{{ s.reps_3 }}</strong></span>
            <span>Push-Ups: <strong class="text-white">{{ s.reps_4 }}</strong></span>
            <span>Cardio: <strong class="text-cyan-300">{{ s.cardio_mode === 'stationary' ? s.reps_5 + ' steps' : formatDuration(s.cardio_duration_secs) + ' (' + s.cardio_mode + ')' }}</strong></span>
          </div>

          <div v-if="s.notes" class="text-xs italic text-slate-400 mt-2 bg-slate-950/40 px-3 py-1.5 rounded-lg border border-slate-800">
            "{{ s.notes }}"
          </div>
        </div>

        <div class="flex items-center gap-3 self-stretch md:self-center justify-between md:justify-end shrink-0">
          <div class="text-right bg-slate-950 p-3.5 rounded-xl border border-slate-800 text-xs min-w-[110px]">
            <div class="font-black text-sm text-cyan-400 font-mono">
              C{{ s.strength_chart }} / C{{ s.cardio_chart }}
            </div>
            <div class="text-[10px] text-slate-500 uppercase tracking-wider mt-0.5">
              Chart Position
            </div>
          </div>

          <button
            type="button"
            class="px-3.5 py-3 rounded-xl bg-slate-900 group-hover:bg-cyan-500/20 border border-slate-800 group-hover:border-cyan-500/40 text-slate-400 group-hover:text-cyan-300 text-xs font-bold transition-all flex items-center gap-1.5 cursor-pointer shrink-0"
            title="Inspect Flight Debrief"
          >
            <span>🔍</span>
            <span class="hidden sm:inline">Inspect & Manage</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
