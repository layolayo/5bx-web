<script setup lang="ts">
import { BadgesResponse } from '../types';

defineProps<{
  badges: BadgesResponse;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();
</script>

<template>
  <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4">
    <div class="bg-slate-800 border border-slate-700 rounded-2xl w-full max-w-2xl max-h-[90vh] overflow-y-auto p-6 shadow-2xl">
      <div class="flex justify-between items-center pb-3 border-b border-slate-700 mb-6">
        <div>
          <h2 class="text-2xl font-bold text-white flex items-center gap-2">
            <span>🏆</span>
            <span>Milestones & Achievements</span>
          </h2>
          <p class="text-xs text-slate-400">RCAF Age Standards, Flying Crew Elite, and Superman Targets.</p>
        </div>
        <button @click="emit('close')" class="text-slate-400 hover:text-white p-2">✕</button>
      </div>

      <!-- Earned Badges Showcase -->
      <div class="mb-8">
        <h3 class="text-sm font-bold text-slate-300 uppercase tracking-wider mb-3">Earned Badges</h3>
        <div v-if="badges.earned_badges.length === 0" class="bg-slate-900/60 p-6 rounded-xl text-center text-slate-400 text-sm">
          No badges earned yet. Reach your age target in both Strength and Cardio to claim your first badge!
        </div>
        <div v-else class="grid grid-cols-1 sm:grid-cols-2 gap-3">
          <div
            v-for="b in badges.earned_badges"
            :key="b.id"
            class="bg-gradient-to-r from-amber-500/10 to-slate-900 border border-amber-500/30 p-3 rounded-xl flex items-center gap-3"
          >
            <div class="w-12 h-12 bg-white rounded-lg p-1 shrink-0 flex items-center justify-center">
              <img v-if="b.image_name" :src="'/images/badges/' + b.image_name" :alt="b.badge_title" class="max-h-full max-w-full object-contain" />
              <span v-else class="text-2xl">🏅</span>
            </div>
            <div>
              <h4 class="text-sm font-bold text-amber-300">{{ b.badge_title }}</h4>
              <p class="text-[10px] text-slate-400">Unlocked {{ new Date(b.earned_at).toLocaleDateString('en-GB') }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Roadmap / Targets -->
      <div>
        <h3 class="text-sm font-bold text-slate-300 uppercase tracking-wider mb-3">Milestone Targets</h3>
        <div class="space-y-2.5">
          <div
            v-for="t in badges.targets"
            :key="t.title"
            class="p-3.5 rounded-xl border flex items-center justify-between transition-colors"
            :class="t.is_achieved ? 'bg-emerald-950/20 border-emerald-500/30' : 'bg-slate-900/50 border-slate-700'"
          >
            <div class="flex items-center gap-3">
              <div class="w-10 h-10 bg-slate-800 rounded-lg p-1 flex items-center justify-center shrink-0 border border-slate-700">
                <img :src="'/images/badges/' + t.image_name" :alt="t.title" class="max-h-full max-w-full object-contain" />
              </div>
              <div>
                <div class="flex items-center gap-2">
                  <h4 class="text-sm font-bold" :class="t.is_achieved ? 'text-emerald-400' : 'text-white'">{{ t.title }}</h4>
                  <span class="text-[10px] px-1.5 py-0.5 rounded font-mono" :class="t.category === 'Superman' ? 'bg-purple-900 text-purple-200' : (t.category === 'Elite' ? 'bg-blue-900 text-blue-200' : 'bg-slate-700 text-slate-300')">
                    {{ t.category }}
                  </span>
                </div>
                <p class="text-xs text-slate-400">Required: Chart {{ t.chart }} Level {{ t.level_display }} (Both Strength & Cardio)</p>
              </div>
            </div>

            <div class="text-right shrink-0">
              <span v-if="t.is_achieved" class="text-xs font-bold text-emerald-400 bg-emerald-500/10 px-2 py-1 rounded border border-emerald-500/30">
                ✓ Unlocked
              </span>
              <span v-else class="text-xs text-slate-500 font-medium">
                In Progress
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
