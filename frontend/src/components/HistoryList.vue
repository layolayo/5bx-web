<script setup lang="ts">
import { WorkoutSessionHistory } from '../types';

defineProps<{
  history: WorkoutSessionHistory[];
}>();

function formatDateTime(iso: string) {
  try {
    const d = new Date(iso);
    return d.toLocaleDateString('en-GB', { day: 'numeric', month: 'short', year: 'numeric', hour: '2-digit', minute: '2-digit' });
  } catch {
    return iso;
  }
}
</script>

<template>
  <div class="max-w-4xl mx-auto p-4 sm:p-6 space-y-4">
    <div class="flex justify-between items-center mb-2">
      <div>
        <h2 class="text-2xl font-bold text-white">Workout History & Progression</h2>
        <p class="text-xs text-slate-400">Log of recorded training sessions and ladder movements.</p>
      </div>
    </div>

    <div v-if="history.length === 0" class="bg-slate-800 border border-slate-700 rounded-xl p-8 text-center text-slate-400">
      No workout sessions recorded yet. Start your first 11-minute mission today!
    </div>

    <div v-else class="space-y-3">
      <div
        v-for="s in history"
        :key="s.id"
        class="bg-slate-800 border border-slate-700 rounded-xl p-4 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 transition hover:border-slate-600"
      >
        <div>
          <div class="flex items-center gap-2 mb-1">
            <span class="text-xs font-semibold text-slate-400">{{ formatDateTime(s.timestamp) }}</span>
            <span
              class="text-[10px] font-bold px-2 py-0.5 rounded uppercase"
              :class="s.overall_status === 'Promoted' ? 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30' : (s.overall_status === 'Demoted' ? 'bg-red-500/20 text-red-400 border border-red-500/30' : 'bg-slate-700 text-slate-300')"
            >
              {{ s.overall_status }}
            </span>
          </div>

          <div class="text-sm font-bold text-white">
            <span>Strength: {{ s.verdict_strength }}</span>
          </div>
          <div class="text-xs text-slate-300 mt-0.5">
            <span>Cardio: {{ s.verdict_cardio }}</span>
          </div>

          <div class="text-[11px] text-slate-400 mt-2 flex gap-3 font-mono">
            <span>Ex1: {{ s.reps_1 }}</span>
            <span>Ex2: {{ s.reps_2 }}</span>
            <span>Ex3: {{ s.reps_3 }}</span>
            <span>Ex4: {{ s.reps_4 }}</span>
            <span>Ex5: {{ s.cardio_mode === 'stationary' ? s.reps_5 + ' steps' : s.cardio_duration_secs + 's' }}</span>
          </div>

          <div v-if="s.notes" class="text-xs italic text-slate-400 mt-1 bg-slate-900/40 px-2 py-1 rounded">
            "{{ s.notes }}"
          </div>
        </div>

        <div class="text-right shrink-0 bg-slate-900/60 p-2.5 rounded-lg border border-slate-700/60 text-xs">
          <div class="font-bold text-slate-200">
            C{{ s.strength_chart }}/C{{ s.cardio_chart }}
          </div>
          <div class="text-[10px] text-slate-400">
            Chart Position
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
