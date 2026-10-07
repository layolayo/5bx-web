<script setup lang="ts">
import { ref, computed } from 'vue';
import { WorkoutSessionHistory } from '../types';
import { updateSessionNotes, deleteSession } from '../api';

const props = defineProps<{
  session: WorkoutSessionHistory;
  isLatest: boolean;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'updated'): void;
  (e: 'deleted'): void;
}>();

const editNotes = ref(props.session.notes || '');
const isSavingNotes = ref(false);
const notesSaveSuccess = ref(false);
const revertLevel = ref(true);
const isDeleting = ref(false);
const showDeleteConfirm = ref(false);
const errorMessage = ref('');

function formatDateTime(iso: string) {
  try {
    const d = new Date(iso);
    return d.toLocaleDateString('en-GB', {
      weekday: 'short',
      day: 'numeric',
      month: 'short',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  } catch {
    return iso;
  }
}

function formatDuration(seconds: number) {
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return `${m}m ${s.toString().padStart(2, '0')}s (${seconds}s)`;
}

async function handleSaveNotes() {
  isSavingNotes.value = true;
  errorMessage.value = '';
  notesSaveSuccess.value = false;
  try {
    await updateSessionNotes(props.session.id, editNotes.value);
    notesSaveSuccess.value = true;
    emit('updated');
    setTimeout(() => {
      notesSaveSuccess.value = false;
    }, 3000);
  } catch (err: any) {
    errorMessage.value = err.message || 'Failed to update debrief notes.';
  } finally {
    isSavingNotes.value = false;
  }
}

async function handleDelete() {
  isDeleting.value = true;
  errorMessage.value = '';
  try {
    await deleteSession(props.session.id, props.isLatest && revertLevel.value);
    emit('deleted');
    emit('close');
  } catch (err: any) {
    errorMessage.value = err.message || 'Failed to delete session.';
    isDeleting.value = false;
    showDeleteConfirm.value = false;
  }
}
</script>

<template>
  <div class="fixed inset-0 z-50 bg-slate-950/85 backdrop-blur-md flex items-center justify-center p-4">
    <div class="bg-slate-900 border border-slate-700/80 rounded-3xl w-full max-w-xl p-6 sm:p-8 shadow-2xl relative overflow-hidden max-h-[92vh] overflow-y-auto">
      <!-- Glow ambient accent -->
      <div class="absolute -top-24 right-0 w-80 h-80 bg-cyan-500/10 blur-3xl pointer-events-none"></div>

      <!-- Header -->
      <div class="flex items-start justify-between pb-4 border-b border-slate-800 mb-6">
        <div>
          <div class="inline-flex items-center gap-2 px-2.5 py-0.5 rounded-full bg-cyan-950/80 border border-cyan-500/30 text-cyan-300 text-[10px] font-bold uppercase tracking-wider mb-1">
            <span>📜</span>
            <span>Flight Debrief & Record Management</span>
          </div>
          <h2 class="text-xl font-black text-white uppercase tracking-tight">Mission Telemetry #{{ session.id }}</h2>
          <p class="text-xs text-slate-400 font-mono">{{ formatDateTime(session.timestamp) }}</p>
        </div>
        <button
          @click="emit('close')"
          class="w-8 h-8 rounded-full bg-slate-800 text-slate-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
        >
          ✕
        </button>
      </div>

      <!-- Error / Success Alert -->
      <div v-if="errorMessage" class="bg-red-500/20 border border-red-500/50 text-red-300 text-xs p-3 rounded-xl mb-4">
        {{ errorMessage }}
      </div>
      <div v-if="notesSaveSuccess" class="bg-emerald-500/20 border border-emerald-500/50 text-emerald-300 text-xs p-3 rounded-xl mb-4">
        ✓ Debrief notes updated successfully.
      </div>

      <!-- Verdict Banner -->
      <div class="bg-slate-950/90 border border-slate-800 rounded-2xl p-4 mb-6">
        <div class="flex items-center justify-between mb-3">
          <span class="text-[10px] font-bold uppercase tracking-wider text-slate-400">RCAF Official Grading</span>
          <span
            class="text-[11px] font-bold px-2.5 py-0.5 rounded-full uppercase tracking-wider font-mono"
            :class="session.overall_status === 'Promoted' ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/40' : (session.overall_status === 'Demoted' ? 'bg-red-500/20 text-red-300 border border-red-500/40' : 'bg-slate-800 text-slate-300 border border-slate-700')"
          >
            {{ session.overall_status }}
          </span>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-xs">
          <div class="bg-slate-900/80 p-3 rounded-xl border border-slate-800">
            <span class="text-[10px] text-slate-400 uppercase font-bold block mb-1">Strength Track (Ex 1–4)</span>
            <div class="text-sm font-black text-emerald-400 font-mono">{{ session.verdict_strength }}</div>
            <div class="text-[10px] text-slate-500 mt-1">Starting Position: Chart {{ session.strength_chart }}</div>
          </div>
          <div class="bg-slate-900/80 p-3 rounded-xl border border-slate-800">
            <span class="text-[10px] text-slate-400 uppercase font-bold block mb-1">Cardio Track (Ex 5)</span>
            <div class="text-sm font-black text-cyan-400 font-mono">{{ session.verdict_cardio }}</div>
            <div class="text-[10px] text-slate-500 mt-1">Starting Position: Chart {{ session.cardio_chart }}</div>
          </div>
        </div>
      </div>

      <!-- Reps Breakdown -->
      <div class="mb-6">
        <h4 class="text-xs font-bold text-slate-400 uppercase tracking-wider mb-2.5">Repetition Breakdown</h4>
        <div class="grid grid-cols-2 sm:grid-cols-4 gap-2.5 text-center">
          <div class="bg-slate-950/70 p-3 rounded-xl border border-slate-800">
            <div class="text-[10px] text-slate-400 font-bold uppercase">1. Bends</div>
            <div class="text-xl font-black text-white font-mono mt-0.5">{{ session.reps_1 }}</div>
          </div>
          <div class="bg-slate-950/70 p-3 rounded-xl border border-slate-800">
            <div class="text-[10px] text-slate-400 font-bold uppercase">2. Sit-Ups</div>
            <div class="text-xl font-black text-white font-mono mt-0.5">{{ session.reps_2 }}</div>
          </div>
          <div class="bg-slate-950/70 p-3 rounded-xl border border-slate-800">
            <div class="text-[10px] text-slate-400 font-bold uppercase">3. Arches</div>
            <div class="text-xl font-black text-white font-mono mt-0.5">{{ session.reps_3 }}</div>
          </div>
          <div class="bg-slate-950/70 p-3 rounded-xl border border-slate-800">
            <div class="text-[10px] text-slate-400 font-bold uppercase">4. Push-Ups</div>
            <div class="text-xl font-black text-white font-mono mt-0.5">{{ session.reps_4 }}</div>
          </div>
        </div>

        <!-- Exercise 5 Cardio Card -->
        <div class="mt-2.5 bg-slate-950/70 p-3.5 rounded-xl border border-slate-800 flex items-center justify-between">
          <div>
            <div class="text-[10px] text-slate-400 font-bold uppercase">5. Cardio Track Discipline</div>
            <div class="text-sm font-black text-cyan-400 mt-0.5 capitalize">
              {{ session.cardio_mode === 'run' ? '1-Mile Outdoor Run' : (session.cardio_mode === 'walk' ? '2-Mile Walk' : 'Stationary Run') }}
            </div>
          </div>
          <div class="text-right font-mono">
            <span class="text-base font-black text-white">
              {{ session.cardio_mode === 'stationary' ? session.reps_5 + ' steps' : formatDuration(session.cardio_duration_secs) }}
            </span>
          </div>
        </div>
      </div>

      <!-- Flight Debrief Notes Editor -->
      <div class="mb-6">
        <label class="text-xs font-bold text-slate-400 uppercase tracking-wider block mb-2">
          Debrief Notes
        </label>
        <textarea
          v-model="editNotes"
          rows="3"
          placeholder="Record notes on cadence, physical sensations, altitude, weather..."
          class="w-full bg-slate-950 text-white p-3 rounded-xl border border-slate-800 text-xs focus:border-cyan-500 focus:outline-none transition-colors"
        ></textarea>
        <div class="flex justify-end mt-2">
          <button
            @click="handleSaveNotes"
            :disabled="isSavingNotes"
            class="px-4 py-2 rounded-xl bg-cyan-600 hover:bg-cyan-500 disabled:opacity-50 text-slate-950 font-bold text-xs uppercase tracking-wider transition-colors cursor-pointer"
          >
            {{ isSavingNotes ? 'Saving...' : 'Save Debrief Notes' }}
          </button>
        </div>
      </div>

      <!-- Record Management Danger Zone -->
      <div class="border-t border-slate-800 pt-6">
        <div class="bg-red-950/30 border border-red-500/30 rounded-2xl p-4">
          <div class="flex items-center justify-between mb-2">
            <div class="flex items-center gap-2">
              <span class="text-red-400 font-bold text-sm">⚠️</span>
              <h5 class="text-xs font-black text-red-300 uppercase tracking-wider">Flight Log Record Removal</h5>
            </div>
            <span v-if="isLatest" class="text-[10px] font-mono font-bold bg-amber-500/20 text-amber-300 border border-amber-500/30 px-2 py-0.5 rounded">
              Latest Mission
            </span>
          </div>

          <p class="text-xs text-slate-400 mb-4">
            If this workout was logged erroneously, you can remove it from your flight log.
          </p>

          <!-- Level Reversion Checkbox for Latest Session -->
          <div v-if="isLatest" class="mb-4 bg-slate-950/80 p-3 rounded-xl border border-slate-800">
            <label class="flex items-start gap-2.5 cursor-pointer select-none">
              <input
                v-model="revertLevel"
                type="checkbox"
                class="mt-0.5 rounded border-slate-700 text-cyan-500 focus:ring-0 cursor-pointer"
              />
              <div class="text-xs">
                <span class="font-bold text-white block">Revert pilot rung level to pre-workout state</span>
                <span class="text-[11px] text-slate-400 leading-tight block mt-0.5">
                  Reverts your active chart and level back to Chart {{ session.strength_chart }} (Strength) and Chart {{ session.cardio_chart }} (Cardio), undoing any promotions or demotions earned in this mission.
                </span>
              </div>
            </label>
          </div>

          <!-- Confirm or Trigger Button -->
          <div v-if="!showDeleteConfirm">
            <button
              @click="showDeleteConfirm = true"
              class="w-full py-2.5 px-4 rounded-xl bg-red-950 hover:bg-red-900 border border-red-500/40 text-red-300 hover:text-white text-xs font-bold uppercase tracking-wider transition-colors cursor-pointer"
            >
              Delete This Flight Log Entry
            </button>
          </div>

          <div v-else class="space-y-2">
            <div class="text-xs font-bold text-red-300 text-center">
              Are you certain? This action cannot be reversed.
            </div>
            <div class="flex gap-2">
              <button
                @click="showDeleteConfirm = false"
                class="flex-1 py-2 rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-300 text-xs font-bold transition-colors cursor-pointer"
              >
                Cancel
              </button>
              <button
                @click="handleDelete"
                :disabled="isDeleting"
                class="flex-1 py-2 rounded-xl bg-red-600 hover:bg-red-500 text-white text-xs font-bold uppercase tracking-wider transition-colors cursor-pointer"
              >
                {{ isDeleting ? 'Removing...' : 'Confirm Deletion' }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
