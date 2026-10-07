<script setup lang="ts">
import { ref } from 'vue';
import { UserProfile } from '../types';

const props = defineProps<{
  profile: UserProfile;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'save', payload: { s_chart: number; s_level: number; c_chart: number; c_level: number; reason: string }): void;
}>();

const sChart = ref(props.profile.strength_chart);
const sLevel = ref(props.profile.strength_level);
const cChart = ref(props.profile.cardio_chart);
const cLevel = ref(props.profile.cardio_level);
const reason = ref('');

const LEVEL_NAMES = ['D-', 'D', 'D+', 'C-', 'C', 'C+', 'B-', 'B', 'B+', 'A-', 'A', 'A+'];

function save() {
  emit('save', {
    s_chart: sChart.value,
    s_level: sLevel.value,
    c_chart: cChart.value,
    c_level: cLevel.value,
    reason: reason.value || 'Manual adjustment',
  });
}
</script>

<template>
  <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4">
    <div class="bg-slate-800 border border-slate-700 rounded-2xl w-full max-w-md p-6 shadow-2xl">
      <div class="flex justify-between items-center pb-3 border-b border-slate-700 mb-4">
        <h2 class="text-xl font-bold text-white">Manual Level Adjustment</h2>
        <button @click="emit('close')" class="text-slate-400 hover:text-white p-2">✕</button>
      </div>

      <p class="text-xs text-slate-400 mb-4">
        Manually adjust your active ladder level if returning from illness, rehabilitation, or setting a custom baseline.
      </p>

      <div class="space-y-4">
        <!-- Strength -->
        <div class="bg-slate-900/60 p-3 rounded-xl border border-slate-700">
          <label class="text-xs font-bold text-emerald-400 block mb-2 uppercase">Strength (Ex 1–4)</label>
          <div class="grid grid-cols-2 gap-2">
            <div>
              <span class="text-[10px] text-slate-400 block mb-1">Chart (1–6)</span>
              <select v-model.number="sChart" class="w-full bg-slate-800 text-white p-2 rounded border border-slate-600 font-bold">
                <option v-for="c in 6" :key="c" :value="c">Chart {{ c }}</option>
              </select>
            </div>
            <div>
              <span class="text-[10px] text-slate-400 block mb-1">Level (1–12)</span>
              <select v-model.number="sLevel" class="w-full bg-slate-800 text-white p-2 rounded border border-slate-600 font-bold">
                <option v-for="l in 12" :key="l" :value="l">Level {{ l }} ({{ LEVEL_NAMES[l-1] }})</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Cardio -->
        <div class="bg-slate-900/60 p-3 rounded-xl border border-slate-700">
          <label class="text-xs font-bold text-blue-400 block mb-2 uppercase">Cardio (Ex 5)</label>
          <div class="grid grid-cols-2 gap-2">
            <div>
              <span class="text-[10px] text-slate-400 block mb-1">Chart (1–6)</span>
              <select v-model.number="cChart" class="w-full bg-slate-800 text-white p-2 rounded border border-slate-600 font-bold">
                <option v-for="c in 6" :key="c" :value="c">Chart {{ c }}</option>
              </select>
            </div>
            <div>
              <span class="text-[10px] text-slate-400 block mb-1">Level (1–12)</span>
              <select v-model.number="cLevel" class="w-full bg-slate-800 text-white p-2 rounded border border-slate-600 font-bold">
                <option v-for="l in 12" :key="l" :value="l">Level {{ l }} ({{ LEVEL_NAMES[l-1] }})</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Reason -->
        <div>
          <label class="text-xs text-slate-400 font-semibold block mb-1">Reason for adjustment</label>
          <input v-model="reason" type="text" placeholder="e.g. Back after holiday, injury recovery" class="w-full bg-slate-900 text-white p-2 rounded-lg border border-slate-700 text-sm" />
        </div>

        <div class="flex gap-3 pt-2">
          <button @click="emit('close')" class="flex-1 py-2.5 rounded-xl bg-slate-700 text-slate-300 font-semibold hover:bg-slate-600">
            Cancel
          </button>
          <button @click="save" class="flex-1 py-2.5 rounded-xl bg-blue-600 hover:bg-blue-500 text-white font-bold shadow">
            Save Adjustment
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
