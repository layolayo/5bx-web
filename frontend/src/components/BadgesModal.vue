<script setup lang="ts">
import { BadgesResponse } from '../types';

defineProps<{
  badges: BadgesResponse;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

function formatBadgeType(type: string): string {
  if (!type) return '';
  if (type === 'EliteSuperman') return 'Elite Superman';
  return type.replace(/([a-z])([A-Z])/g, '$1 $2');
}
</script>

<template>
  <div class="fixed inset-0 z-50 bg-slate-950/85 backdrop-blur-md flex items-center justify-center p-4">
    <div class="bg-slate-900 border border-slate-700/80 rounded-3xl w-full max-w-3xl max-h-[90vh] overflow-y-auto p-6 sm:p-8 shadow-2xl relative">
      <!-- Glow ambient accent -->
      <div class="absolute -top-20 -right-20 w-48 h-48 bg-amber-500/10 blur-2xl pointer-events-none"></div>

      <!-- Header -->
      <div class="flex justify-between items-center pb-4 border-b border-slate-800 mb-6">
        <div>
          <div class="inline-flex items-center gap-1.5 px-3 py-0.5 rounded-full bg-amber-950/60 border border-amber-500/30 text-amber-300 text-[10px] font-bold uppercase tracking-wider mb-1">
            <span>🏆</span>
            <span>RCAF Trophy Room</span>
          </div>
          <h2 class="text-2xl font-black text-white uppercase tracking-tight">Milestones & Achievements</h2>
          <p class="text-xs text-slate-400">Official Royal Canadian Air Force age standards, Flying Crew Elite wings, and Superman honours.</p>
        </div>
        <button
          @click="emit('close')"
          class="w-8 h-8 rounded-full bg-slate-800 text-slate-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
        >
          ✕
        </button>
      </div>

      <!-- Earned Badges Showcase -->
      <div class="mb-8">
        <h3 class="text-xs font-black text-slate-300 uppercase tracking-wider mb-3 flex items-center justify-between">
          <span>Earned Flight Honours ({{ badges.earned_badges.length }})</span>
          <span class="text-[10px] text-amber-400 font-mono">Evaluated Against Current Rung</span>
        </h3>

        <div v-if="badges.earned_badges.length === 0" class="bg-slate-950/80 p-8 rounded-2xl border border-slate-800 text-center text-slate-400 text-xs">
          No honours earned yet. Advance your Strength and Cardio ladder rungs to unlock official RCAF wings!
        </div>

        <div v-else class="grid grid-cols-1 sm:grid-cols-2 gap-3">
          <div
            v-for="b in badges.earned_badges"
            :key="b.key"
            class="p-4 rounded-2xl border flex items-center gap-4 transition-all"
            :class="b.is_highest ? 'bg-gradient-to-r from-amber-500/15 via-amber-900/10 to-slate-950 border-amber-500/50 shadow-lg shadow-amber-500/10' : 'bg-slate-950/80 border-slate-800'"
          >
            <!-- Badge Icon Container -->
            <div class="w-14 h-14 bg-white rounded-xl p-1.5 shrink-0 flex items-center justify-center shadow-inner border border-slate-300">
              <img :src="'/images/badges/' + b.image_name" :alt="b.title" class="max-h-full max-w-full object-contain" />
            </div>

            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-1.5 mb-0.5">
                <span
                  class="text-[9px] font-black uppercase tracking-wider px-2 py-0.5 rounded-full"
                  :class="b.is_highest ? 'bg-amber-400 text-slate-950' : 'bg-slate-800 text-slate-400'"
                >
                  {{ b.is_highest ? 'Highest Honour' : formatBadgeType(b.badge_type) }}
                </span>
                <span class="text-[10px] font-mono text-emerald-400 font-bold truncate">{{ b.status_text }}</span>
              </div>
              <h4 class="text-sm font-black text-white truncate">{{ b.title }}</h4>
              <p class="text-[11px] font-mono text-slate-400 mt-0.5">{{ b.details }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Target Roadmap -->
      <div>
        <h3 class="text-xs font-black text-slate-300 uppercase tracking-wider mb-3">
          Calibration Targets Roadmap
        </h3>

        <div class="space-y-2.5">
          <div
            v-for="t in badges.targets"
            :key="t.title"
            class="p-4 rounded-2xl border flex items-center justify-between transition-colors"
            :class="t.is_achieved ? 'bg-emerald-950/20 border-emerald-500/40' : 'bg-slate-950/80 border-slate-800'"
          >
            <div class="flex items-center gap-3.5">
              <div class="w-10 h-10 rounded-xl bg-white p-1 shrink-0 flex items-center justify-center shadow-inner">
                <img :src="'/images/badges/' + t.image_name" :alt="t.title" class="max-h-full max-w-full object-contain" />
              </div>
              <div>
                <h4 class="text-sm font-bold text-white">{{ t.title }}</h4>
                <p class="text-xs font-mono text-slate-400">Target Standard: Chart {{ t.chart }} • Level {{ t.level_display }}</p>
              </div>
            </div>

            <div class="shrink-0 text-right">
              <span
                class="text-[11px] font-bold px-3 py-1 rounded-full uppercase tracking-wider font-mono"
                :class="t.is_achieved ? 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30' : 'bg-slate-800 text-slate-400'"
              >
                {{ t.is_achieved ? '✓ Qualified' : 'In Progress' }}
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Close Button -->
      <div class="mt-8 pt-4 border-t border-slate-800">
        <button
          @click="emit('close')"
          class="w-full btn-control-primary bg-cyan-500 hover:bg-cyan-400 text-slate-950"
        >
          Return to Mission
        </button>
      </div>
    </div>
  </div>
</template>
