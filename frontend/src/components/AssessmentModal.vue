<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { LayoffStatus, DiagnosticPlacementResult } from '../types';
import { evaluateDiagnostic, applyAssessment } from '../api';

const props = defineProps<{
  show: boolean;
  layoffStatus: LayoffStatus | null;
  currentStrengthChart: number;
  currentStrengthLevel: number;
  currentStrengthDisplay: string;
  currentCardioChart: number;
  currentCardioLevel: number;
  currentCardioDisplay: string;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'applied'): void;
}>();

type ActiveTab = 'layoff' | 'benchmark' | 'novice';
const activeTab = ref<ActiveTab>('layoff');

// Diagnostic benchmark form state
const ex1Reps = ref<number>(10);
const ex2Reps = ref<number>(8);
const ex3Reps = ref<number>(12);
const ex4Reps = ref<number>(6);
const cardioMode = ref<'stationary' | 'run' | 'walk'>('stationary');
const cardioReps = ref<number>(250);
const cardioMin = ref<number>(8);
const cardioSec = ref<number>(30);

const isEvaluating = ref(false);
const isApplying = ref(false);
const placementResult = ref<DiagnosticPlacementResult | null>(null);
const errorMessage = ref<string>('');

// Synchronise tab on modal show
watch(
  () => props.show,
  (newVal) => {
    if (newVal) {
      if (props.layoffStatus && props.layoffStatus.is_layoff) {
        activeTab.value = 'layoff';
      } else {
        activeTab.value = 'benchmark';
      }
      errorMessage.value = '';
      calculateLivePlacement();
    }
  },
  { immediate: true }
);

// Automatic recalculation when inputs change
watch([ex1Reps, ex2Reps, ex3Reps, ex4Reps, cardioMode, cardioReps, cardioMin, cardioSec], () => {
  calculateLivePlacement();
});

async function calculateLivePlacement() {
  try {
    isEvaluating.value = true;
    errorMessage.value = '';

    const durationSecs = cardioMode.value === 'stationary' ? 0 : cardioMin.value * 60 + cardioSec.value;

    const res = await evaluateDiagnostic({
      reps_1: Number(ex1Reps.value) || 0,
      reps_2: Number(ex2Reps.value) || 0,
      reps_3: Number(ex3Reps.value) || 0,
      reps_4: Number(ex4Reps.value) || 0,
      cardio_mode: cardioMode.value,
      reps_5: cardioMode.value === 'stationary' ? Number(cardioReps.value) || 0 : undefined,
      cardio_duration_secs: cardioMode.value !== 'stationary' ? durationSecs : undefined,
    });

    placementResult.value = res;
  } catch (e: any) {
    console.error('Placement evaluation error:', e);
  } finally {
    isEvaluating.value = false;
  }
}

async function handleAcceptSafeReentry() {
  if (!props.layoffStatus) return;
  try {
    isApplying.value = true;
    errorMessage.value = '';
    await applyAssessment({
      strength_chart: props.layoffStatus.recommended_strength_chart,
      strength_level: props.layoffStatus.recommended_strength_level,
      cardio_chart: props.layoffStatus.recommended_cardio_chart,
      cardio_level: props.layoffStatus.recommended_cardio_level,
      assessment_type: 'layoff_safe_reentry',
      notes: `Accepted safe re-entry recommendation after ${props.layoffStatus.days_inactive} days of absence.`,
    });
    emit('applied');
    emit('close');
  } catch (e: any) {
    errorMessage.value = e.message || 'Failed to apply safe re-entry.';
  } finally {
    isApplying.value = false;
  }
}

async function handleApplyBenchmark() {
  if (!placementResult.value) return;
  try {
    isApplying.value = true;
    errorMessage.value = '';
    await applyAssessment({
      strength_chart: placementResult.value.strength_chart,
      strength_level: placementResult.value.strength_level,
      cardio_chart: placementResult.value.cardio_chart,
      cardio_level: placementResult.value.cardio_level,
      assessment_type: 'diagnostic_placement',
      notes: `Calibrated via Diagnostic Benchmark: Strength ${placementResult.value.strength_display}, Cardio ${placementResult.value.cardio_display}.`,
    });
    emit('applied');
    emit('close');
  } catch (e: any) {
    errorMessage.value = e.message || 'Failed to apply benchmark placement.';
  } finally {
    isApplying.value = false;
  }
}

async function handleApplyNovice() {
  try {
    isApplying.value = true;
    errorMessage.value = '';
    await applyAssessment({
      strength_chart: 1,
      strength_level: 1,
      cardio_chart: 1,
      cardio_level: 1,
      assessment_type: 'novice_start',
      notes: 'Initialised flight readiness at RCAF baseline: Chart 1, Level 1 (D-).',
    });
    emit('applied');
    emit('close');
  } catch (e: any) {
    errorMessage.value = e.message || 'Failed to initialise novice baseline.';
  } finally {
    isApplying.value = false;
  }
}

const severityColour = computed(() => {
  if (!props.layoffStatus) return 'text-slate-400 border-slate-700 bg-slate-800/40';
  switch (props.layoffStatus.severity) {
    case 'mild':
      return 'text-amber-400 border-amber-500/30 bg-amber-500/10';
    case 'moderate':
      return 'text-orange-400 border-orange-500/30 bg-orange-500/10';
    case 'prolonged':
    case 'extended':
      return 'text-rose-400 border-rose-500/30 bg-rose-500/10';
    default:
      return 'text-slate-400 border-slate-700 bg-slate-800/40';
  }
});
</script>

<template>
  <div v-if="show" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/80 backdrop-blur-sm animate-in fade-in duration-200">
    <div class="relative w-full max-w-2xl bg-slate-900 border border-slate-800 rounded-3xl shadow-2xl overflow-hidden flex flex-col max-h-[90vh]">
      
      <!-- Modal Header -->
      <div class="px-6 py-5 border-b border-slate-800 bg-slate-900/60 flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-emerald-500/20 to-cyan-500/20 border border-emerald-500/30 flex items-center justify-center text-emerald-400 font-black">
            🧭
          </div>
          <div>
            <h2 class="text-xl font-bold text-white tracking-wide">Flight Assessment & Re-calibration</h2>
            <p class="text-xs text-slate-400">Establish your operational rung or adjust safely after an absence</p>
          </div>
        </div>
        <button
          @click="emit('close')"
          class="p-2 text-slate-400 hover:text-white rounded-xl hover:bg-slate-800 transition"
          aria-label="Close modal"
        >
          ✕
        </button>
      </div>

      <!-- Navigation Tabs -->
      <div class="flex border-b border-slate-800 bg-slate-950/40 px-6 pt-3 gap-2">
        <button
          v-if="layoffStatus && layoffStatus.is_layoff"
          @click="activeTab = 'layoff'"
          :class="[
            'pb-3 px-4 text-xs font-bold tracking-wider uppercase transition border-b-2 flex items-center gap-1.5',
            activeTab === 'layoff'
              ? 'border-amber-400 text-amber-300'
              : 'border-transparent text-slate-400 hover:text-slate-200'
          ]"
        >
          <span>⚠️</span> Layoff Re-entry
        </button>

        <button
          @click="activeTab = 'benchmark'"
          :class="[
            'pb-3 px-4 text-xs font-bold tracking-wider uppercase transition border-b-2 flex items-center gap-1.5',
            activeTab === 'benchmark'
              ? 'border-cyan-400 text-cyan-300'
              : 'border-transparent text-slate-400 hover:text-slate-200'
          ]"
        >
          <span>⚡</span> Diagnostic Benchmark
        </button>

        <button
          @click="activeTab = 'novice'"
          :class="[
            'pb-3 px-4 text-xs font-bold tracking-wider uppercase transition border-b-2 flex items-center gap-1.5',
            activeTab === 'novice'
              ? 'border-emerald-400 text-emerald-300'
              : 'border-transparent text-slate-400 hover:text-slate-200'
          ]"
        >
          <span>🌱</span> Novice Induction
        </button>
      </div>

      <!-- Modal Body -->
      <div class="p-6 overflow-y-auto space-y-6 flex-1">
        
        <!-- Error alert -->
        <div v-if="errorMessage" class="p-4 rounded-2xl bg-rose-500/10 border border-rose-500/30 text-rose-300 text-xs">
          {{ errorMessage }}
        </div>

        <!-- TAB 1: LAYOFF RE-ENTRY ADVISORY -->
        <div v-if="activeTab === 'layoff' && layoffStatus" class="space-y-6">
          
          <!-- Layoff status card -->
          <div :class="['p-5 rounded-2xl border flex flex-col sm:flex-row sm:items-center justify-between gap-4', severityColour]">
            <div>
              <div class="text-xs font-bold uppercase tracking-wider opacity-80">Absence Detected</div>
              <div class="text-2xl font-black mt-0.5">{{ layoffStatus.days_inactive }} Days Inactive</div>
              <div v-if="layoffStatus.last_workout_date" class="text-xs opacity-75 mt-1">
                Previous sortie: {{ layoffStatus.last_workout_date }}
              </div>
            </div>
            <div class="text-xs max-w-sm leading-relaxed">
              {{ layoffStatus.rationale }}
            </div>
          </div>

          <!-- Comparison Grid -->
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            
            <!-- Current Active Levels -->
            <div class="p-4 rounded-2xl bg-slate-950/60 border border-slate-800 space-y-3">
              <div class="text-xs font-bold uppercase tracking-wider text-slate-400 flex items-center gap-2">
                <span>📍</span> Current Recorded Rungs
              </div>
              <div class="space-y-2">
                <div class="p-2.5 rounded-xl bg-slate-900 border border-slate-800">
                  <div class="text-[10px] uppercase font-bold text-emerald-400">Strength Track</div>
                  <div class="text-sm font-semibold text-slate-200">{{ layoffStatus.current_strength_display }}</div>
                </div>
                <div class="p-2.5 rounded-xl bg-slate-900 border border-slate-800">
                  <div class="text-[10px] uppercase font-bold text-cyan-400">Cardio Track</div>
                  <div class="text-sm font-semibold text-slate-200">{{ layoffStatus.current_cardio_display }}</div>
                </div>
              </div>
            </div>

            <!-- Recommended Safe Re-entry Levels -->
            <div class="p-4 rounded-2xl bg-emerald-950/20 border border-emerald-500/30 space-y-3">
              <div class="text-xs font-bold uppercase tracking-wider text-emerald-300 flex items-center gap-2">
                <span>🛡️</span> Recommended Safe Re-entry
              </div>
              <div class="space-y-2">
                <div class="p-2.5 rounded-xl bg-slate-900/90 border border-emerald-500/30">
                  <div class="text-[10px] uppercase font-bold text-emerald-400">Calibrated Strength</div>
                  <div class="text-sm font-bold text-emerald-200">{{ layoffStatus.recommended_strength_display }}</div>
                </div>
                <div class="p-2.5 rounded-xl bg-slate-900/90 border border-emerald-500/30">
                  <div class="text-[10px] uppercase font-bold text-cyan-400">Calibrated Cardio</div>
                  <div class="text-sm font-bold text-cyan-200">{{ layoffStatus.recommended_cardio_display }}</div>
                </div>
              </div>
            </div>

          </div>

          <!-- Actions -->
          <div class="pt-2 flex flex-col sm:flex-row gap-3">
            <button
              @click="handleAcceptSafeReentry"
              :disabled="isApplying"
              class="flex-1 py-3 px-5 rounded-2xl bg-emerald-500 hover:bg-emerald-400 disabled:opacity-50 text-slate-950 font-bold text-sm tracking-wide shadow-lg shadow-emerald-500/20 transition flex items-center justify-center gap-2"
            >
              <span>✓</span>
              <span>{{ isApplying ? 'Re-calibrating...' : 'Accept Safe Re-entry Rung' }}</span>
            </button>

            <button
              @click="activeTab = 'benchmark'"
              class="py-3 px-5 rounded-2xl bg-slate-800 hover:bg-slate-700 text-cyan-300 font-semibold text-sm transition flex items-center justify-center gap-2"
            >
              <span>⚡</span>
              <span>Test Current Fitness Instead</span>
            </button>

            <button
              @click="emit('close')"
              class="py-3 px-4 rounded-2xl bg-slate-900 hover:bg-slate-800 text-slate-400 hover:text-slate-200 font-medium text-xs transition"
            >
              Maintain Current Rung
            </button>
          </div>

        </div>

        <!-- TAB 2: DIAGNOSTIC BENCHMARK -->
        <div v-else-if="activeTab === 'benchmark'" class="space-y-6">
          
          <div class="p-4 rounded-2xl bg-cyan-950/20 border border-cyan-500/30 text-xs text-cyan-200 leading-relaxed flex items-start gap-3">
            <span class="text-lg">💡</span>
            <div>
              <span class="font-bold text-cyan-300">RCAF Diagnostic Protocol:</span>
              Enter the maximum comfortable repetitions you can perform within the standard 5BX time envelopes (or beat the cardio clock). Our engine will determine your optimum training rungs.
            </div>
          </div>

          <!-- Strength inputs (Emerald track) -->
          <div class="p-4 rounded-2xl bg-slate-950/50 border border-emerald-500/30 space-y-4">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold uppercase tracking-wider text-emerald-400 flex items-center gap-2">
                <span>💪</span> Strength Track (Exercises 1 to 4)
              </span>
              <span class="text-[10px] text-slate-400">Strict time limits</span>
            </div>

            <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
              <div class="space-y-1">
                <label class="text-[11px] font-medium text-slate-300 block">Ex 1: Forward Bends</label>
                <div class="text-[10px] text-slate-500">2 mins</div>
                <input
                  v-model.number="ex1Reps"
                  type="number"
                  min="0"
                  max="100"
                  class="w-full bg-slate-900 border border-slate-700 focus:border-emerald-500 rounded-xl px-3 py-2 text-white font-bold text-sm text-center"
                />
              </div>

              <div class="space-y-1">
                <label class="text-[11px] font-medium text-slate-300 block">Ex 2: Sit-Ups</label>
                <div class="text-[10px] text-slate-500">1 min</div>
                <input
                  v-model.number="ex2Reps"
                  type="number"
                  min="0"
                  max="100"
                  class="w-full bg-slate-900 border border-slate-700 focus:border-emerald-500 rounded-xl px-3 py-2 text-white font-bold text-sm text-center"
                />
              </div>

              <div class="space-y-1">
                <label class="text-[11px] font-medium text-slate-300 block">Ex 3: Back Arches</label>
                <div class="text-[10px] text-slate-500">1 min</div>
                <input
                  v-model.number="ex3Reps"
                  type="number"
                  min="0"
                  max="100"
                  class="w-full bg-slate-900 border border-slate-700 focus:border-emerald-500 rounded-xl px-3 py-2 text-white font-bold text-sm text-center"
                />
              </div>

              <div class="space-y-1">
                <label class="text-[11px] font-medium text-slate-300 block">Ex 4: Push-Ups</label>
                <div class="text-[10px] text-slate-500">1 min</div>
                <input
                  v-model.number="ex4Reps"
                  type="number"
                  min="0"
                  max="100"
                  class="w-full bg-slate-900 border border-slate-700 focus:border-emerald-500 rounded-xl px-3 py-2 text-white font-bold text-sm text-center"
                />
              </div>
            </div>
          </div>

          <!-- Cardio inputs (Cyan track) -->
          <div class="p-4 rounded-2xl bg-slate-950/50 border border-cyan-500/30 space-y-4">
            <div class="flex items-center justify-between">
              <span class="text-xs font-bold uppercase tracking-wider text-cyan-400 flex items-center gap-2">
                <span>🏃</span> Cardio Track (Exercise 5)
              </span>
              <div class="flex gap-1 bg-slate-900 p-0.5 rounded-lg border border-slate-800">
                <button
                  v-for="mode in (['stationary', 'run', 'walk'] as const)"
                  :key="mode"
                  type="button"
                  @click="cardioMode = mode"
                  :class="[
                    'px-2.5 py-1 text-[10px] font-bold rounded-md uppercase transition',
                    cardioMode === mode
                      ? 'bg-cyan-500 text-slate-950'
                      : 'text-slate-400 hover:text-slate-200'
                  ]"
                >
                  {{ mode === 'stationary' ? 'Stationary' : mode === 'run' ? 'Run' : 'Walk' }}
                </button>
              </div>
            </div>

            <!-- Stationary steps -->
            <div v-if="cardioMode === 'stationary'" class="space-y-1">
              <label class="text-[11px] font-medium text-slate-300 block">Stationary Run Steps (in 6 minutes)</label>
              <input
                v-model.number="cardioReps"
                type="number"
                min="0"
                max="1000"
                step="10"
                class="w-full sm:w-1/2 bg-slate-900 border border-slate-700 focus:border-cyan-500 rounded-xl px-3 py-2 text-white font-bold text-sm"
              />
            </div>

            <!-- Outdoor time mm:ss -->
            <div v-else class="space-y-2">
              <label class="text-[11px] font-medium text-slate-300 block">
                {{ cardioMode === 'run' ? '1 Mile (1.6 km) Run Time' : '2 Mile (3.2 km) Walk Time' }} (mm:ss)
              </label>
              <div class="flex items-center gap-2 max-w-xs">
                <div class="flex-1 flex items-center bg-slate-900 border border-slate-700 rounded-xl px-3 py-2 focus-within:border-cyan-500">
                  <input
                    v-model.number="cardioMin"
                    type="number"
                    min="0"
                    max="59"
                    class="w-full bg-transparent text-white font-mono font-bold text-center focus:outline-none"
                    placeholder="mm"
                  />
                  <span class="text-slate-500 text-xs font-bold px-1">m</span>
                </div>
                <span class="text-slate-500 font-bold">:</span>
                <div class="flex-1 flex items-center bg-slate-900 border border-slate-700 rounded-xl px-3 py-2 focus-within:border-cyan-500">
                  <input
                    v-model.number="cardioSec"
                    type="number"
                    min="0"
                    max="59"
                    class="w-full bg-transparent text-white font-mono font-bold text-center focus:outline-none"
                    placeholder="ss"
                  />
                  <span class="text-slate-500 text-xs font-bold px-1">s</span>
                </div>
              </div>
            </div>

          </div>

          <!-- Placement Outcome Preview -->
          <div v-if="placementResult" class="p-4 rounded-2xl bg-gradient-to-br from-slate-950 to-slate-900 border border-slate-800 space-y-3">
            <div class="text-xs font-bold uppercase tracking-wider text-slate-400 flex items-center justify-between">
              <span>🎯 Calibrated Placement Preview</span>
              <span v-if="isEvaluating" class="text-cyan-400 animate-pulse text-[10px]">Calculating...</span>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <div class="p-3 rounded-xl bg-slate-900/80 border border-emerald-500/30">
                <div class="text-[10px] font-bold uppercase text-emerald-400">Assessed Strength Rung</div>
                <div class="text-base font-black text-emerald-200 mt-0.5">{{ placementResult.strength_display }}</div>
              </div>
              <div class="p-3 rounded-xl bg-slate-900/80 border border-cyan-500/30">
                <div class="text-[10px] font-bold uppercase text-cyan-400">Assessed Cardio Rung</div>
                <div class="text-base font-black text-cyan-200 mt-0.5">{{ placementResult.cardio_display }}</div>
              </div>
            </div>

            <p class="text-xs text-slate-400 leading-relaxed">
              {{ placementResult.summary }}
            </p>
          </div>

          <!-- Actions -->
          <div class="pt-2 flex flex-col sm:flex-row gap-3">
            <button
              @click="handleApplyBenchmark"
              :disabled="isApplying || !placementResult"
              class="flex-1 py-3 px-5 rounded-2xl bg-cyan-500 hover:bg-cyan-400 disabled:opacity-50 text-slate-950 font-bold text-sm tracking-wide shadow-lg shadow-cyan-500/20 transition flex items-center justify-center gap-2"
            >
              <span>⚡</span>
              <span>{{ isApplying ? 'Setting Rung...' : 'Apply Calibrated Starting Level' }}</span>
            </button>

            <button
              @click="emit('close')"
              class="py-3 px-5 rounded-2xl bg-slate-800 hover:bg-slate-700 text-slate-300 font-medium text-xs transition"
            >
              Cancel
            </button>
          </div>

        </div>

        <!-- TAB 3: NOVICE INDUCTION -->
        <div v-else-if="activeTab === 'novice'" class="space-y-6">
          
          <div class="p-5 rounded-2xl bg-emerald-950/20 border border-emerald-500/30 space-y-3">
            <div class="text-sm font-bold text-emerald-300 flex items-center gap-2">
              <span>🌱</span> The RCAF Progressive Induction Philosophy
            </div>
            <p class="text-xs text-slate-300 leading-relaxed">
              Dr Bill Orban designed the 5BX plan on a foundational principle: physical progression must be governed by connective tissue and cardiovascular adaptation, not merely muscular willingness.
            </p>
            <p class="text-xs text-slate-300 leading-relaxed">
              Starting at <strong>Chart 1 • Level 1 (D-)</strong> requires just 2 toe touches, 3 sit-ups, 4 back arches, 2 push-ups, and 100 stationary running steps. Even if this feels light on Day 1, starting here ensures your ligaments and joints strengthen safely before the intensity escalates.
            </p>
          </div>

          <div class="p-4 rounded-2xl bg-slate-950 border border-slate-800 text-xs text-slate-400 space-y-1">
            <div class="font-bold text-slate-200">Induction Assignment:</div>
            <div>• Strength: <strong>Chart 1 • Level 1 (D-)</strong></div>
            <div>• Cardio: <strong>Chart 1 • Level 1 (D-)</strong></div>
          </div>

          <!-- Actions -->
          <div class="pt-2 flex flex-col sm:flex-row gap-3">
            <button
              @click="handleApplyNovice"
              :disabled="isApplying"
              class="flex-1 py-3 px-5 rounded-2xl bg-emerald-500 hover:bg-emerald-400 disabled:opacity-50 text-slate-950 font-bold text-sm tracking-wide shadow-lg shadow-emerald-500/20 transition flex items-center justify-center gap-2"
            >
              <span>🚀</span>
              <span>{{ isApplying ? 'Initialising...' : 'Begin at Chart 1 • Level 1 (D-)' }}</span>
            </button>

            <button
              @click="emit('close')"
              class="py-3 px-5 rounded-2xl bg-slate-800 hover:bg-slate-700 text-slate-300 font-medium text-xs transition"
            >
              Cancel
            </button>
          </div>

        </div>

      </div>

    </div>
  </div>
</template>
