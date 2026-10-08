<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import { LayoffStatus, DiagnosticPlacementResult, ExerciseInstructionRow, ExerciseChartRow, WorkoutSessionHistory } from '../types';
import { evaluateDiagnostic, applyAssessment, fetchSystemCharts } from '../api';
import { playCountdownBeep, playTransitionChime, playCelebrationChime } from '../audio';
import { usePrecisionTimer } from '../composables/usePrecisionTimer';
import { getLastPerformance, calculateTreadmillSpeed, formatDurationMmSs } from '../telemetry';

const props = defineProps<{
  show: boolean;
  history?: WorkoutSessionHistory[];
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

// Candidate Target Chart State (1 to 6)
const candidateChart = ref<number>(props.currentStrengthChart || 1);

// Chart Descriptions based on RCAF 5BX doctrine
const chartDescriptions: Record<number, string> = {
  1: 'Chart 1: Novice baseline & physical recovery. Uses knee push-ups and gentle forward bends.',
  2: 'Chart 2: Active foundation. Standard toe push-ups and palm floor touches.',
  3: 'Chart 3: Regular active conditioning. V-push-ups, 3-point floor touches, and half knee bends.',
  4: 'Chart 4: Advanced conditioning. Semi-tuck sit-ups and wide posterior extensions.',
  5: 'Chart 5: Aircrew readiness. High-intensity military candidate standards.',
  6: 'Chart 6: Elite flight standard. Maximum RCAF aerobatic & extreme physical standard.',
};

// System exercise instructions & charts cache
const systemInstructions = ref<ExerciseInstructionRow[]>([]);
const systemCharts = ref<ExerciseChartRow[]>([]);

async function loadInstructions() {
  try {
    const data = await fetchSystemCharts();
    if (data) {
      if (data.instructions) systemInstructions.value = data.instructions;
      if (data.charts) systemCharts.value = data.charts;
    }
  } catch (e) {
    console.error('Failed to load system instructions:', e);
  }
}

// Diagnostic benchmark form state
const ex1Reps = ref<number>(14);
const ex2Reps = ref<number>(10);
const ex3Reps = ref<number>(12);
const ex4Reps = ref<number>(8);
const cardioMode = ref<'stationary' | 'run' | 'walk'>('stationary');
const cardioReps = ref<number>(250);
const cardioMin = ref<number>(8);
const cardioSec = ref<number>(30);

// Candidate cardio distance & pace calculations
const candidateCardioDistanceMiles = computed(() => {
  if (cardioMode.value === 'run') {
    return candidateChart.value === 1 ? 0.5 : 1.0;
  }
  if (cardioMode.value === 'walk') {
    return candidateChart.value === 1 ? 1.0 : 2.0;
  }
  return 0;
});

const candidateCardioDistanceKm = computed(() => {
  return (candidateCardioDistanceMiles.value * 1.60934).toFixed(1);
});

const candidateCardioEntryRow = computed(() => {
  return systemCharts.value.find((c) => c.chart === candidateChart.value && c.level === 1);
});

const candidateCardioEliteRow = computed(() => {
  return systemCharts.value.find((c) => c.chart === candidateChart.value && c.level === 12);
});

const candidateCardioTargetSeconds = computed(() => {
  const row = candidateCardioEntryRow.value;
  if (!row) return cardioMode.value === 'stationary' ? 360 : (cardioMode.value === 'run' ? 525 : 1740);
  if (cardioMode.value === 'run') return row.ex5_run;
  if (cardioMode.value === 'walk') return row.ex5_walk;
  return 360;
});

const candidateTreadmillSpeed = computed(() => {
  return calculateTreadmillSpeed(candidateCardioDistanceMiles.value, candidateCardioTargetSeconds.value);
});

function getCandidateStandard(exNum: number, tier: 'entry' | 'elite'): string {
  const row = tier === 'entry' ? candidateCardioEntryRow.value : candidateCardioEliteRow.value;
  if (!row) return '—';
  if (exNum === 1) return `${row.ex1} reps`;
  if (exNum === 2) return `${row.ex2} reps`;
  if (exNum === 3) return `${row.ex3} reps`;
  if (exNum === 4) return `${row.ex4} reps`;
  if (exNum === 5) {
    if (cardioMode.value === 'stationary') return `${row.ex5} steps`;
    if (cardioMode.value === 'run') return `≤ ${formatDurationMmSs(row.ex5_run)}`;
    if (cardioMode.value === 'walk') return `≤ ${formatDurationMmSs(row.ex5_walk)}`;
  }
  return '—';
}

// Candidate Chart exercises (1 to 5)
const candidateExercises = computed(() => {
  return [1, 2, 3, 4, 5].map((exNum) => {
    if (exNum === 5) {
      if (cardioMode.value === 'run') {
        const dist = candidateCardioDistanceMiles.value;
        const km = candidateCardioDistanceKm.value;
        const targetSec = candidateCardioTargetSeconds.value;
        return {
          exercise_number: 5,
          name: `${dist}-Mile Continuous Run (${km} km)`,
          instructions: `Run continuously at a steady aerobic cadence over ${dist} mi (${km} km). On treadmill, maintain at least ${candidateTreadmillSpeed.value.display} without holding handrails.`,
          image_path: 'run.png',
          time_limit_seconds: targetSec,
          distance_miles: dist,
          target_sec: targetSec,
        };
      }
      if (cardioMode.value === 'walk') {
        const dist = candidateCardioDistanceMiles.value;
        const km = candidateCardioDistanceKm.value;
        const targetSec = candidateCardioTargetSeconds.value;
        return {
          exercise_number: 5,
          name: `${dist}-Mile Continuous Walk (${km} km)`,
          instructions: `Vigorous, continuous outdoor or treadmill walking over ${dist} mi (${km} km). Maintain brisk cadence with arms swinging freely. Note: Charts 5 & 6 require running.`,
          image_path: 'walk.png',
          time_limit_seconds: targetSec,
          distance_miles: dist,
          target_sec: targetSec,
        };
      }
      const inst5 = systemInstructions.value.find(
        (i) => i.chart === candidateChart.value && i.exercise === 5
      );
      return {
        exercise_number: 5,
        name: inst5?.name || 'Stationary Run',
        instructions: inst5?.instructions || 'Stationary running with scissor jumps every 75 steps (count each time left foot touches the ground).',
        image_path: inst5?.image_path || `c${candidateChart.value}_ex5.png`,
        time_limit_seconds: 360,
        distance_miles: 0,
        target_sec: 360,
      };
    }

    const inst = systemInstructions.value.find(
      (i) => i.chart === candidateChart.value && i.exercise === exNum
    );
    const defaultNames = ['', 'Toe Touch', 'Sit-Ups', 'Back Arches', 'Push-Ups', 'Stationary Run'];
    const defaultImages = [
      '',
      `c${candidateChart.value}_ex1.png`,
      `c${candidateChart.value}_ex2.png`,
      `c${candidateChart.value}_ex3.png`,
      `c${candidateChart.value}_ex4.png`,
      `c${candidateChart.value}_ex5.png`,
    ];
    return {
      exercise_number: exNum,
      name: inst?.name || defaultNames[exNum],
      instructions: inst?.instructions || 'Execute repetitions strictly with controlled cadence within the designated time envelope.',
      image_path: inst?.image_path || defaultImages[exNum],
      time_limit_seconds: exNum === 1 ? 120 : 60,
      distance_miles: 0,
      target_sec: exNum === 1 ? 120 : 60,
    };
  });
});

const isEvaluating = ref(false);
const isApplying = ref(false);
const placementResult = ref<DiagnosticPlacementResult | null>(null);
const errorMessage = ref<string>('');

// Interactive Drill Timer State (Precision Wall-Clock with Screen Wake Lock)
const precisionTimer = usePrecisionTimer();
const activeDrillExercise = ref<any | null>(null);
type DrillStage = 'ready' | 'countdown' | 'running' | 'record';
const drillStage = ref<DrillStage>('ready');
const drillCountdown = ref<number | string>(3);
const drillRecordedReps = ref<number>(0);
const drillRecordedMin = ref<number>(8);
const drillRecordedSec = ref<number>(30);
let drillCountdownTimeout: any = null;

// Synchronise tab and data on modal show
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
      if (props.currentStrengthChart && props.currentStrengthChart > 0) {
        candidateChart.value = props.currentStrengthChart;
      }
      loadInstructions();
      calculateLivePlacement();
    } else {
      cancelDrill();
    }
  },
  { immediate: true }
);

// Automatic recalculation when inputs or target chart changes
watch(
  [candidateChart, ex1Reps, ex2Reps, ex3Reps, ex4Reps, cardioMode, cardioReps, cardioMin, cardioSec],
  () => {
    calculateLivePlacement();
  }
);

async function calculateLivePlacement() {
  try {
    isEvaluating.value = true;
    errorMessage.value = '';

    const durationSecs = cardioMode.value === 'stationary' ? 0 : (Number(cardioMin.value) || 0) * 60 + (Number(cardioSec.value) || 0);

    const res = await evaluateDiagnostic({
      candidate_chart: candidateChart.value,
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

// Drill Timer Methods
function startDrill(ex: any) {
  activeDrillExercise.value = ex;
  drillStage.value = 'ready';
  precisionTimer.stopTimer();

  if (ex.exercise_number === 1) drillRecordedReps.value = ex1Reps.value;
  else if (ex.exercise_number === 2) drillRecordedReps.value = ex2Reps.value;
  else if (ex.exercise_number === 3) drillRecordedReps.value = ex3Reps.value;
  else if (ex.exercise_number === 4) drillRecordedReps.value = ex4Reps.value;
  else if (ex.exercise_number === 5) {
    if (cardioMode.value === 'stationary') {
      drillRecordedReps.value = cardioReps.value;
    } else {
      drillRecordedMin.value = cardioMin.value;
      drillRecordedSec.value = cardioSec.value;
    }
  }
}

function beginDrillCountdown() {
  drillStage.value = 'countdown';
  drillCountdown.value = 3;
  playCountdownBeep(false);

  drillCountdownTimeout = setTimeout(() => {
    drillCountdown.value = 2;
    playCountdownBeep(false);
    drillCountdownTimeout = setTimeout(() => {
      drillCountdown.value = 1;
      playCountdownBeep(false);
      drillCountdownTimeout = setTimeout(() => {
        drillCountdown.value = 'GO!';
        playCountdownBeep(true);
        drillCountdownTimeout = setTimeout(() => {
          startDrillClock();
        }, 500);
      }, 1000);
    }, 1000);
  }, 1000);
}

function startDrillClock() {
  if (!activeDrillExercise.value) return;
  drillStage.value = 'running';
  precisionTimer.startTimer(activeDrillExercise.value.time_limit_seconds, onDrillComplete);
}

function onDrillComplete() {
  if (activeDrillExercise.value?.exercise_number === 5 && cardioMode.value !== 'stationary') {
    const elapsed = activeDrillExercise.value.time_limit_seconds;
    drillRecordedMin.value = Math.floor(elapsed / 60);
    drillRecordedSec.value = elapsed % 60;
  }
  drillStage.value = 'record';
}

function toggleDrillPause() {
  precisionTimer.togglePause();
}

function finishDrillEarly() {
  const elapsed = precisionTimer.elapsedSeconds.value;
  precisionTimer.finishTimer();
  if (activeDrillExercise.value?.exercise_number === 5 && cardioMode.value !== 'stationary') {
    drillRecordedMin.value = Math.floor(elapsed / 60);
    drillRecordedSec.value = elapsed % 60;
  }
  drillStage.value = 'record';
}

function saveDrillPerformance() {
  if (!activeDrillExercise.value) return;
  const exNum = activeDrillExercise.value.exercise_number;

  if (exNum === 1) ex1Reps.value = drillRecordedReps.value;
  else if (exNum === 2) ex2Reps.value = drillRecordedReps.value;
  else if (exNum === 3) ex3Reps.value = drillRecordedReps.value;
  else if (exNum === 4) ex4Reps.value = drillRecordedReps.value;
  else if (exNum === 5) {
    if (cardioMode.value === 'stationary') {
      cardioReps.value = drillRecordedReps.value;
    } else {
      cardioMin.value = drillRecordedMin.value;
      cardioSec.value = drillRecordedSec.value;
    }
  }

  cancelDrill();
  calculateLivePlacement();
}

function cancelDrill() {
  precisionTimer.stopTimer();
  clearTimeout(drillCountdownTimeout);
  activeDrillExercise.value = null;
  drillStage.value = 'ready';
}

function formatMmSs(totalSeconds: number): string {
  const m = Math.floor(totalSeconds / 60);
  const s = totalSeconds % 60;
  return `${m}:${s.toString().padStart(2, '0')}`;
}

// Handlers for applying assessment rungs
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
      notes: `Calibrated via Chart ${candidateChart.value} Diagnostic Benchmark: Strength ${placementResult.value.strength_display}, Cardio ${placementResult.value.cardio_display}.`,
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
      notes: 'Initialised 5BX program at Novice Baseline: Chart 1 • Level 1 (D-).',
    });
    emit('applied');
    emit('close');
  } catch (e: any) {
    errorMessage.value = e.message || 'Failed to apply novice baseline.';
  } finally {
    isApplying.value = false;
  }
}

onMounted(() => {
  loadInstructions();
});

onUnmounted(() => {
  cancelDrill();
});

const severityColour = computed(() => {
  if (!props.layoffStatus) return '';
  switch (props.layoffStatus.severity) {
    case 'mild':
      return 'border-amber-500/30 bg-amber-500/10 text-amber-300';
    case 'moderate':
      return 'border-orange-500/30 bg-orange-500/10 text-orange-300';
    case 'prolonged':
    case 'severe':
      return 'border-rose-500/30 bg-rose-500/10 text-rose-300';
    default:
      return 'border-cyan-500/30 bg-cyan-500/10 text-cyan-300';
  }
});
</script>

<template>
  <div v-if="show" class="fixed inset-0 z-50 flex items-center justify-center p-3 sm:p-4 bg-slate-950/85 backdrop-blur-md">
    <div class="relative w-full max-w-3xl max-h-[92vh] flex flex-col bg-slate-900 border border-slate-700/80 rounded-3xl shadow-2xl overflow-hidden">
      <!-- Glow ambient accent -->
      <div class="absolute -top-24 right-0 w-80 h-80 bg-cyan-500/10 blur-3xl pointer-events-none"></div>

      <!-- Modal Header -->
      <div class="p-5 sm:p-6 pb-4 border-b border-slate-800 flex items-center justify-between shrink-0 relative z-10">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-xl bg-cyan-500/20 border border-cyan-500/40 text-cyan-400 flex items-center justify-center text-xl font-bold shadow-sm">
            🧭
          </div>
          <div>
            <h2 class="text-xl font-black text-white uppercase tracking-tight">
              Flight Assessment & Re-calibration
            </h2>
            <p class="text-xs text-slate-400">
              RCAF protocol for candidate placement, baseline testing, and layoff re-entry
            </p>
          </div>
        </div>

        <button
          @click="emit('close')"
          class="w-8 h-8 rounded-full bg-slate-800 text-slate-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
        >
          ✕
        </button>
      </div>

      <!-- Top Tab Switcher -->
      <div class="flex border-b border-slate-800 px-6 pt-2 shrink-0 gap-2 overflow-x-auto select-none">
        <button
          v-if="layoffStatus && layoffStatus.is_layoff"
          @click="activeTab = 'layoff'"
          :class="[
            'pb-3 px-4 text-xs font-bold tracking-wider uppercase transition border-b-2 flex items-center gap-1.5 cursor-pointer',
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
            'pb-3 px-4 text-xs font-bold tracking-wider uppercase transition border-b-2 flex items-center gap-1.5 cursor-pointer',
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
            'pb-3 px-4 text-xs font-bold tracking-wider uppercase transition border-b-2 flex items-center gap-1.5 cursor-pointer',
            activeTab === 'novice'
              ? 'border-emerald-400 text-emerald-300'
              : 'border-transparent text-slate-400 hover:text-slate-200'
          ]"
        >
          <span>🌱</span> Novice Induction
        </button>
      </div>

      <!-- Modal Body -->
      <div class="p-5 sm:p-6 pr-4 sm:pr-6 overflow-y-auto space-y-6 flex-1">
        <!-- Error alert -->
        <div v-if="errorMessage" class="p-4 rounded-2xl bg-rose-500/10 border border-rose-500/30 text-rose-300 text-xs">
          {{ errorMessage }}
        </div>

        <!-- TAB 1: LAYOFF RE-ENTRY ADVISORY -->
        <div v-if="activeTab === 'layoff' && layoffStatus" class="space-y-6">
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

          <div class="pt-2 flex flex-col sm:flex-row gap-3">
            <button
              @click="handleAcceptSafeReentry"
              :disabled="isApplying"
              class="flex-1 py-3 px-5 rounded-2xl bg-emerald-500 hover:bg-emerald-400 disabled:opacity-50 text-slate-950 font-bold text-sm tracking-wide shadow-lg shadow-emerald-500/20 transition flex items-center justify-center gap-2 cursor-pointer"
            >
              <span>✓</span>
              <span>{{ isApplying ? 'Re-calibrating...' : 'Accept Safe Re-entry Rung' }}</span>
            </button>

            <button
              @click="activeTab = 'benchmark'"
              class="py-3 px-5 rounded-2xl bg-slate-800 hover:bg-slate-700 text-cyan-300 font-semibold text-sm transition flex items-center justify-center gap-2 cursor-pointer"
            >
              <span>⚡</span>
              <span>Test Current Fitness Instead</span>
            </button>

            <button
              @click="emit('close')"
              class="py-3 px-4 rounded-2xl bg-slate-900 hover:bg-slate-800 text-slate-400 hover:text-slate-200 font-medium text-xs transition cursor-pointer"
            >
              Maintain Current Rung
            </button>
          </div>
        </div>

        <!-- TAB 2: DIAGNOSTIC BENCHMARK WITH BENCHMARK CHART SELECTION & DRILL TIMERS -->
        <div v-else-if="activeTab === 'benchmark'" class="space-y-6">
          <!-- 1. TARGET BENCHMARK CHART SELECTOR -->
          <div class="p-4 rounded-2xl bg-slate-950/70 border border-slate-800 space-y-3">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-1">
              <div>
                <span class="text-xs font-black text-white uppercase tracking-wider block">
                  1. Select Target Benchmark Chart (1 to 6)
                </span>
                <span class="text-[11px] text-slate-400">
                  Calisthenic exercises advance in difficulty across charts. Select the chart standard you wish to test:
                </span>
              </div>
              <span class="text-[10px] font-mono text-cyan-300 bg-cyan-950/80 border border-cyan-500/30 px-2 py-0.5 rounded self-start sm:self-auto">
                Current: Chart {{ currentStrengthChart }}
              </span>
            </div>

            <!-- Chart Selection Tabs -->
            <div class="grid grid-cols-3 sm:grid-cols-6 gap-2 pt-1 select-none">
              <button
                v-for="ch in 6"
                :key="ch"
                type="button"
                @click="candidateChart = ch"
                class="py-3 px-2.5 rounded-xl border text-center transition-all cursor-pointer flex flex-col items-center justify-center"
                :class="candidateChart === ch
                  ? 'bg-cyan-500 text-slate-950 border-cyan-400 font-black shadow-md shadow-cyan-500/20'
                  : 'bg-slate-900 text-slate-300 border-slate-800 hover:border-slate-700'"
              >
                <span class="text-xs font-black">Chart {{ ch }}</span>
              </button>
            </div>

            <!-- Chart Description Callout -->
            <div class="text-[11px] text-cyan-300/90 bg-cyan-950/40 border border-cyan-500/20 p-2.5 rounded-xl leading-relaxed">
              {{ chartDescriptions[candidateChart] }}
            </div>
          </div>

          <!-- 2. STRENGTH EXERCISES 1 TO 4 (WITH GUIDED DRILL TIMERS) -->
          <div class="p-4 rounded-2xl bg-slate-950/60 border border-emerald-500/30 space-y-4">
            <div class="flex items-center justify-between">
              <div>
                <span class="text-xs font-black uppercase tracking-wider text-emerald-400 flex items-center gap-2">
                  <span>💪</span> Strength Track (Exercises 1 to 4)
                </span>
                <span class="text-[10px] text-slate-400">Perform maximum comfortable repetitions within designated time envelopes.</span>
              </div>
              <span class="text-[10px] font-mono text-emerald-400 bg-emerald-950/60 border border-emerald-500/30 px-2 py-0.5 rounded">
                Chart {{ candidateChart }} Standards
              </span>
            </div>

            <!-- 4 Exercise Cards -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-3.5">
              <!-- Exercise 1 -->
              <div class="p-3.5 rounded-xl bg-slate-900 border border-slate-800 flex flex-col justify-between gap-3">
                <div class="flex items-start gap-3">
                  <div class="w-16 h-14 bg-white rounded-lg p-1 flex items-center justify-center shrink-0 border border-slate-300">
                    <img :src="'/images/' + candidateExercises[0].image_path" :alt="candidateExercises[0].name" class="max-h-full max-w-full object-contain" />
                  </div>
                  <div class="flex-1 min-w-0">
                    <div class="flex items-center justify-between">
                      <span class="text-xs font-black text-white truncate">1. {{ candidateExercises[0].name }}</span>
                      <span class="text-[10px] font-mono text-emerald-400 font-bold">2 mins</span>
                    </div>
                    <p class="text-[10px] text-slate-400 line-clamp-2 mt-0.5">{{ candidateExercises[0].instructions }}</p>
                    <div class="flex flex-wrap items-center gap-1.5 text-[9px] font-mono mt-1.5">
                      <span v-if="getLastPerformance(props.history || [], 1)" class="text-amber-400 bg-amber-950/60 border border-amber-500/30 px-1.5 py-0.5 rounded">
                        Last: {{ getLastPerformance(props.history || [], 1)?.display }}
                      </span>
                      <span class="text-emerald-400 bg-emerald-950/60 border border-emerald-500/30 px-1.5 py-0.5 rounded">
                        Entry (D-): {{ getCandidateStandard(1, 'entry') }}
                      </span>
                      <span class="text-cyan-400 bg-cyan-950/60 border border-cyan-500/30 px-1.5 py-0.5 rounded">
                        Elite (A+): {{ getCandidateStandard(1, 'elite') }}
                      </span>
                    </div>
                  </div>
                </div>

                <div class="flex items-center justify-between pt-2 border-t border-slate-800/80 gap-2">
                  <button
                    type="button"
                    @click="startDrill(candidateExercises[0])"
                    class="py-1.5 px-3 rounded-lg bg-emerald-950/80 border border-emerald-500/40 hover:bg-emerald-900 text-emerald-300 text-[11px] font-bold flex items-center gap-1.5 transition-colors cursor-pointer"
                  >
                    <span>⏱️</span>
                    <span>Test Drill</span>
                  </button>

                  <div class="flex items-center gap-1.5">
                    <button
                      type="button"
                      @click="ex1Reps = Math.max(0, ex1Reps - 1)"
                      class="w-7 h-7 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 font-bold flex items-center justify-center cursor-pointer"
                    >-</button>
                    <input
                      v-model.number="ex1Reps"
                      type="number"
                      min="0"
                      max="100"
                      class="w-12 bg-slate-950 border border-slate-700 rounded-lg py-1 text-center font-black text-sm text-emerald-400 focus:outline-none focus:border-emerald-500"
                    />
                    <button
                      type="button"
                      @click="ex1Reps = ex1Reps + 1"
                      class="w-7 h-7 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 font-bold flex items-center justify-center cursor-pointer"
                    >+</button>
                    <span class="text-[10px] text-slate-500">reps</span>
                  </div>
                </div>
              </div>

              <!-- Exercise 2 -->
              <div class="p-3.5 rounded-xl bg-slate-900 border border-slate-800 flex flex-col justify-between gap-3">
                <div class="flex items-start gap-3">
                  <div class="w-16 h-14 bg-white rounded-lg p-1 flex items-center justify-center shrink-0 border border-slate-300">
                    <img :src="'/images/' + candidateExercises[1].image_path" :alt="candidateExercises[1].name" class="max-h-full max-w-full object-contain" />
                  </div>
                  <div class="flex-1 min-w-0">
                    <div class="flex items-center justify-between">
                      <span class="text-xs font-black text-white truncate">2. {{ candidateExercises[1].name }}</span>
                      <span class="text-[10px] font-mono text-emerald-400 font-bold">1 min</span>
                    </div>
                    <p class="text-[10px] text-slate-400 line-clamp-2 mt-0.5">{{ candidateExercises[1].instructions }}</p>
                    <div class="flex flex-wrap items-center gap-1.5 text-[9px] font-mono mt-1.5">
                      <span v-if="getLastPerformance(props.history || [], 2)" class="text-amber-400 bg-amber-950/60 border border-amber-500/30 px-1.5 py-0.5 rounded">
                        Last: {{ getLastPerformance(props.history || [], 2)?.display }}
                      </span>
                      <span class="text-emerald-400 bg-emerald-950/60 border border-emerald-500/30 px-1.5 py-0.5 rounded">
                        Entry (D-): {{ getCandidateStandard(2, 'entry') }}
                      </span>
                      <span class="text-cyan-400 bg-cyan-950/60 border border-cyan-500/30 px-1.5 py-0.5 rounded">
                        Elite (A+): {{ getCandidateStandard(2, 'elite') }}
                      </span>
                    </div>
                  </div>
                </div>

                <div class="flex items-center justify-between pt-2 border-t border-slate-800/80 gap-2">
                  <button
                    type="button"
                    @click="startDrill(candidateExercises[1])"
                    class="py-1.5 px-3 rounded-lg bg-emerald-950/80 border border-emerald-500/40 hover:bg-emerald-900 text-emerald-300 text-[11px] font-bold flex items-center gap-1.5 transition-colors cursor-pointer"
                  >
                    <span>⏱️</span>
                    <span>Test Drill</span>
                  </button>

                  <div class="flex items-center gap-1.5">
                    <button
                      type="button"
                      @click="ex2Reps = Math.max(0, ex2Reps - 1)"
                      class="w-7 h-7 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 font-bold flex items-center justify-center cursor-pointer"
                    >-</button>
                    <input
                      v-model.number="ex2Reps"
                      type="number"
                      min="0"
                      max="100"
                      class="w-12 bg-slate-950 border border-slate-700 rounded-lg py-1 text-center font-black text-sm text-emerald-400 focus:outline-none focus:border-emerald-500"
                    />
                    <button
                      type="button"
                      @click="ex2Reps = ex2Reps + 1"
                      class="w-7 h-7 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 font-bold flex items-center justify-center cursor-pointer"
                    >+</button>
                    <span class="text-[10px] text-slate-500">reps</span>
                  </div>
                </div>
              </div>

              <!-- Exercise 3 -->
              <div class="p-3.5 rounded-xl bg-slate-900 border border-slate-800 flex flex-col justify-between gap-3">
                <div class="flex items-start gap-3">
                  <div class="w-16 h-14 bg-white rounded-lg p-1 flex items-center justify-center shrink-0 border border-slate-300">
                    <img :src="'/images/' + candidateExercises[2].image_path" :alt="candidateExercises[2].name" class="max-h-full max-w-full object-contain" />
                  </div>
                  <div class="flex-1 min-w-0">
                    <div class="flex items-center justify-between">
                      <span class="text-xs font-black text-white truncate">3. {{ candidateExercises[2].name }}</span>
                      <span class="text-[10px] font-mono text-emerald-400 font-bold">1 min</span>
                    </div>
                    <p class="text-[10px] text-slate-400 line-clamp-2 mt-0.5">{{ candidateExercises[2].instructions }}</p>
                    <div class="flex flex-wrap items-center gap-1.5 text-[9px] font-mono mt-1.5">
                      <span v-if="getLastPerformance(props.history || [], 3)" class="text-amber-400 bg-amber-950/60 border border-amber-500/30 px-1.5 py-0.5 rounded">
                        Last: {{ getLastPerformance(props.history || [], 3)?.display }}
                      </span>
                      <span class="text-emerald-400 bg-emerald-950/60 border border-emerald-500/30 px-1.5 py-0.5 rounded">
                        Entry (D-): {{ getCandidateStandard(3, 'entry') }}
                      </span>
                      <span class="text-cyan-400 bg-cyan-950/60 border border-cyan-500/30 px-1.5 py-0.5 rounded">
                        Elite (A+): {{ getCandidateStandard(3, 'elite') }}
                      </span>
                    </div>
                  </div>
                </div>

                <div class="flex items-center justify-between pt-2 border-t border-slate-800/80 gap-2">
                  <button
                    type="button"
                    @click="startDrill(candidateExercises[2])"
                    class="py-1.5 px-3 rounded-lg bg-emerald-950/80 border border-emerald-500/40 hover:bg-emerald-900 text-emerald-300 text-[11px] font-bold flex items-center gap-1.5 transition-colors cursor-pointer"
                  >
                    <span>⏱️</span>
                    <span>Test Drill</span>
                  </button>

                  <div class="flex items-center gap-1.5">
                    <button
                      type="button"
                      @click="ex3Reps = Math.max(0, ex3Reps - 1)"
                      class="w-7 h-7 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 font-bold flex items-center justify-center cursor-pointer"
                    >-</button>
                    <input
                      v-model.number="ex3Reps"
                      type="number"
                      min="0"
                      max="100"
                      class="w-12 bg-slate-950 border border-slate-700 rounded-lg py-1 text-center font-black text-sm text-emerald-400 focus:outline-none focus:border-emerald-500"
                    />
                    <button
                      type="button"
                      @click="ex3Reps = ex3Reps + 1"
                      class="w-7 h-7 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 font-bold flex items-center justify-center cursor-pointer"
                    >+</button>
                    <span class="text-[10px] text-slate-500">reps</span>
                  </div>
                </div>
              </div>

              <!-- Exercise 4 -->
              <div class="p-3.5 rounded-xl bg-slate-900 border border-slate-800 flex flex-col justify-between gap-3">
                <div class="flex items-start gap-3">
                  <div class="w-16 h-14 bg-white rounded-lg p-1 flex items-center justify-center shrink-0 border border-slate-300">
                    <img :src="'/images/' + candidateExercises[3].image_path" :alt="candidateExercises[3].name" class="max-h-full max-w-full object-contain" />
                  </div>
                  <div class="flex-1 min-w-0">
                    <div class="flex items-center justify-between">
                      <span class="text-xs font-black text-white truncate">4. {{ candidateExercises[3].name }}</span>
                      <span class="text-[10px] font-mono text-emerald-400 font-bold">1 min</span>
                    </div>
                    <p class="text-[10px] text-slate-400 line-clamp-2 mt-0.5">{{ candidateExercises[3].instructions }}</p>
                    <div class="flex flex-wrap items-center gap-1.5 text-[9px] font-mono mt-1.5">
                      <span v-if="getLastPerformance(props.history || [], 4)" class="text-amber-400 bg-amber-950/60 border border-amber-500/30 px-1.5 py-0.5 rounded">
                        Last: {{ getLastPerformance(props.history || [], 4)?.display }}
                      </span>
                      <span class="text-emerald-400 bg-emerald-950/60 border border-emerald-500/30 px-1.5 py-0.5 rounded">
                        Entry (D-): {{ getCandidateStandard(4, 'entry') }}
                      </span>
                      <span class="text-cyan-400 bg-cyan-950/60 border border-cyan-500/30 px-1.5 py-0.5 rounded">
                        Elite (A+): {{ getCandidateStandard(4, 'elite') }}
                      </span>
                    </div>
                  </div>
                </div>

                <div class="flex items-center justify-between pt-2 border-t border-slate-800/80 gap-2">
                  <button
                    type="button"
                    @click="startDrill(candidateExercises[3])"
                    class="py-1.5 px-3 rounded-lg bg-emerald-950/80 border border-emerald-500/40 hover:bg-emerald-900 text-emerald-300 text-[11px] font-bold flex items-center gap-1.5 transition-colors cursor-pointer"
                  >
                    <span>⏱️</span>
                    <span>Test Drill</span>
                  </button>

                  <div class="flex items-center gap-1.5">
                    <button
                      type="button"
                      @click="ex4Reps = Math.max(0, ex4Reps - 1)"
                      class="w-7 h-7 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 font-bold flex items-center justify-center cursor-pointer"
                    >-</button>
                    <input
                      v-model.number="ex4Reps"
                      type="number"
                      min="0"
                      max="100"
                      class="w-12 bg-slate-950 border border-slate-700 rounded-lg py-1 text-center font-black text-sm text-emerald-400 focus:outline-none focus:border-emerald-500"
                    />
                    <button
                      type="button"
                      @click="ex4Reps = ex4Reps + 1"
                      class="w-7 h-7 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 font-bold flex items-center justify-center cursor-pointer"
                    >+</button>
                    <span class="text-[10px] text-slate-500">reps</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 3. CARDIO EXERCISE 5 (WITH DRILL TIMER) -->
          <div class="p-4 rounded-2xl bg-slate-950/60 border border-cyan-500/30 space-y-4">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
              <div>
                <span class="text-xs font-black uppercase tracking-wider text-cyan-400 flex items-center gap-2">
                  <span>🏃</span> Cardio Track (Exercise 5)
                </span>
                <span class="text-[10px] text-slate-400">Select stationary running or timed track/treadmill discipline.</span>
              </div>

              <!-- Discipline Selector -->
              <div class="flex gap-1 bg-slate-900 p-1 rounded-xl border border-slate-800 select-none">
                <button
                  v-for="mode in (['stationary', 'run', 'walk'] as const)"
                  :key="mode"
                  type="button"
                  @click="cardioMode = mode"
                  :class="[
                    'px-3 py-1 text-[11px] font-bold rounded-lg uppercase transition cursor-pointer',
                    cardioMode === mode
                      ? 'bg-cyan-500 text-slate-950 font-black shadow-sm'
                      : 'text-slate-400 hover:text-slate-200'
                  ]"
                >
                  {{ mode === 'stationary' ? 'Stationary' : mode === 'run' ? 'Run' : 'Walk' }}
                </button>
              </div>
            </div>

            <!-- Cardio Details Card -->
            <div class="p-3.5 rounded-xl bg-slate-900 border border-slate-800 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
              <div class="flex items-center gap-3">
                <div class="w-16 h-14 bg-white rounded-lg p-1 flex items-center justify-center shrink-0 border border-slate-300">
                  <img :src="'/images/' + candidateExercises[4].image_path" :alt="candidateExercises[4].name" class="max-h-full max-w-full object-contain" />
                </div>
                <div>
                  <div class="text-xs font-black text-white">5. {{ candidateExercises[4].name }}</div>
                  <div class="text-[10px] text-slate-400 mt-0.5">
                    {{ cardioMode === 'stationary' ? '6 Minutes Stationary Run (10 scissor jumps / 75 steps)' : (cardioMode === 'run' ? `Continuous ${candidateCardioDistanceMiles} mi (${candidateCardioDistanceKm} km) Run` : `Continuous ${candidateCardioDistanceMiles} mi (${candidateCardioDistanceKm} km) Walk`) }}
                  </div>
                  <!-- Treadmill speed recommendation if run/walk -->
                  <div v-if="cardioMode !== 'stationary' && candidateTreadmillSpeed.mph > 0" class="text-[10px] font-mono text-cyan-300 mt-0.5">
                    Treadmill target: ≥ <strong>{{ candidateTreadmillSpeed.display }}</strong>
                  </div>
                </div>
              </div>

              <div class="flex items-center gap-3 self-end sm:self-auto">
                <button
                  type="button"
                  @click="startDrill(candidateExercises[4])"
                  class="py-2 px-3.5 rounded-xl bg-cyan-950/80 border border-cyan-500/40 hover:bg-cyan-900 text-cyan-300 text-xs font-bold flex items-center gap-1.5 transition-colors cursor-pointer"
                >
                  <span>⏱️</span>
                  <span>{{ cardioMode === 'stationary' ? '6-Min Drill' : 'Time Drill' }}</span>
                </button>

                <!-- Value Input -->
                <div v-if="cardioMode === 'stationary'" class="flex items-center gap-1.5">
                  <input
                    v-model.number="cardioReps"
                    type="number"
                    min="0"
                    max="1200"
                    step="10"
                    class="w-20 bg-slate-950 border border-slate-700 rounded-xl px-2.5 py-1.5 text-center font-black text-sm text-cyan-400 focus:outline-none focus:border-cyan-500"
                  />
                  <span class="text-[11px] text-slate-400">steps</span>
                </div>

                <div v-else class="flex items-center gap-1.5">
                  <div class="flex items-center bg-slate-950 border border-slate-700 rounded-xl px-2 py-1">
                    <input
                      v-model.number="cardioMin"
                      type="number"
                      min="0"
                      max="59"
                      class="w-8 bg-transparent text-white font-mono font-bold text-center focus:outline-none"
                    />
                    <span class="text-slate-500 text-xs font-bold px-0.5">m</span>
                    <span class="text-slate-500 font-bold">:</span>
                    <input
                      v-model.number="cardioSec"
                      type="number"
                      min="0"
                      max="59"
                      class="w-8 bg-transparent text-white font-mono font-bold text-center focus:outline-none"
                    />
                    <span class="text-slate-500 text-xs font-bold px-0.5">s</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Historical & candidate benchmarks -->
            <div class="flex flex-wrap items-center gap-2 text-[10px] font-mono pt-1">
              <span v-if="getLastPerformance(props.history || [], 5, cardioMode)" class="text-amber-400 bg-amber-950/60 border border-amber-500/30 px-2 py-0.5 rounded-lg">
                Last Logged: {{ getLastPerformance(props.history || [], 5, cardioMode)?.display }}
              </span>
              <span class="text-emerald-400 bg-emerald-950/60 border border-emerald-500/30 px-2 py-0.5 rounded-lg">
                Chart {{ candidateChart }} Entry (D-): {{ getCandidateStandard(5, 'entry') }}
              </span>
              <span class="text-cyan-400 bg-cyan-950/60 border border-cyan-500/30 px-2 py-0.5 rounded-lg">
                Elite Standard (A+): {{ getCandidateStandard(5, 'elite') }}
              </span>
            </div>
          </div>

          <!-- 4. PLACEMENT OUTCOME PREVIEW -->
          <div v-if="placementResult" class="p-4 rounded-2xl bg-gradient-to-br from-slate-950 to-slate-900 border border-slate-800 space-y-3 shadow-lg">
            <div class="text-xs font-bold uppercase tracking-wider text-slate-400 flex items-center justify-between">
              <span>🎯 Calibrated Placement Preview</span>
              <span v-if="isEvaluating" class="text-cyan-400 animate-pulse text-[10px]">Evaluating Candidate Standards...</span>
            </div>

            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <div class="p-3.5 rounded-xl bg-slate-900/90 border border-emerald-500/30">
                <div class="text-[10px] font-bold uppercase text-emerald-400">Assessed Strength Rung</div>
                <div class="text-base font-black text-emerald-200 mt-0.5">{{ placementResult.strength_display }}</div>
              </div>
              <div class="p-3.5 rounded-xl bg-slate-900/90 border border-cyan-500/30">
                <div class="text-[10px] font-bold uppercase text-cyan-400">Assessed Cardio Rung</div>
                <div class="text-base font-black text-cyan-200 mt-0.5">{{ placementResult.cardio_display }}</div>
              </div>
            </div>

            <p class="text-xs text-slate-300 leading-relaxed bg-slate-950 p-2.5 rounded-xl border border-slate-850">
              {{ placementResult.summary }}
            </p>
          </div>

          <!-- Actions -->
          <div class="pt-2 flex flex-col sm:flex-row gap-3">
            <button
              @click="handleApplyBenchmark"
              :disabled="isApplying || !placementResult"
              class="flex-1 py-3 px-5 rounded-2xl bg-cyan-500 hover:bg-cyan-400 disabled:opacity-50 text-slate-950 font-black text-sm uppercase tracking-wider shadow-lg shadow-cyan-500/20 transition flex items-center justify-center gap-2 cursor-pointer"
            >
              <span>⚡</span>
              <span>{{ isApplying ? 'Calibrating Ladder...' : 'Apply Calibrated Starting Level' }}</span>
            </button>

            <button
              @click="emit('close')"
              class="py-3 px-5 rounded-2xl bg-slate-800 hover:bg-slate-700 text-slate-300 font-medium text-xs transition cursor-pointer"
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
              Starting at <strong>Chart 1 • Level 1 (D-)</strong> requires just 2 toe touches, 3 sit-ups, 4 back arches, 2 knee push-ups, and 100 stationary running steps. Even if this feels light on Day 1, starting here ensures your ligaments and joints strengthen safely before the intensity escalates.
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
              class="flex-1 py-3 px-5 rounded-2xl bg-emerald-500 hover:bg-emerald-400 disabled:opacity-50 text-slate-950 font-black text-sm uppercase tracking-wider shadow-lg shadow-emerald-500/20 transition flex items-center justify-center gap-2 cursor-pointer"
            >
              <span>🚀</span>
              <span>{{ isApplying ? 'Initialising...' : 'Begin at Chart 1 • Level 1 (D-)' }}</span>
            </button>

            <button
              @click="emit('close')"
              class="py-3 px-5 rounded-2xl bg-slate-800 hover:bg-slate-700 text-slate-300 font-medium text-xs transition cursor-pointer"
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- ========================================================================= -->
    <!-- INTERACTIVE EXERCISE DRILL TIMER MODAL OVERLAY                            -->
    <!-- ========================================================================= -->
    <div
      v-if="activeDrillExercise"
      class="fixed inset-0 z-60 bg-slate-950/95 backdrop-blur-xl flex items-center justify-center p-4 animate-in fade-in duration-200"
    >
      <div class="bg-slate-900 border border-slate-700/80 rounded-3xl w-full max-w-lg p-6 shadow-2xl relative flex flex-col space-y-4 text-center">
        <!-- Close / Cancel button -->
        <button
          @click="cancelDrill"
          class="absolute top-4 right-4 w-8 h-8 rounded-full bg-slate-800 text-slate-400 hover:text-white flex items-center justify-center transition cursor-pointer"
        >
          ✕
        </button>

        <!-- Exercise Title & Mission Envelope -->
        <div>
          <span class="text-[10px] font-mono font-bold uppercase tracking-wider text-cyan-400 bg-cyan-950/80 border border-cyan-500/30 px-2.5 py-0.5 rounded-full">
            Exercise {{ activeDrillExercise.exercise_number }} • Candidate Chart {{ candidateChart }}
          </span>
          <h3 class="text-xl font-black text-white uppercase tracking-tight mt-1.5">
            {{ activeDrillExercise.name }}
          </h3>
          <p class="text-xs text-slate-400 font-mono mt-0.5">
            <template v-if="activeDrillExercise.exercise_number === 5 && cardioMode !== 'stationary'">
              Target Ceiling: ≤ {{ formatDurationMmSs(activeDrillExercise.time_limit_seconds) }} ({{ candidateCardioDistanceMiles }} mi / {{ candidateCardioDistanceKm }} km)
            </template>
            <template v-else>
              Time Envelope: {{ activeDrillExercise.time_limit_seconds >= 60 ? (activeDrillExercise.time_limit_seconds / 60) + ' Minutes' : activeDrillExercise.time_limit_seconds + ' Seconds' }}
            </template>
          </p>

          <!-- Treadmill speed target if Run/Walk -->
          <div v-if="activeDrillExercise.exercise_number === 5 && cardioMode !== 'stationary' && candidateTreadmillSpeed.mph > 0" class="text-xs font-mono text-cyan-300 font-bold mt-1">
            Treadmill Target: ≥ {{ candidateTreadmillSpeed.display }}
          </div>
        </div>

        <!-- Screen Wake Lock indicator -->
        <div v-if="precisionTimer.isWakeLockActive.value" class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-emerald-950/70 border border-emerald-500/30 text-[11px] font-mono font-bold text-emerald-400 mx-auto">
          <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
          <span>📱 Screen Stay-Awake Active</span>
        </div>

        <!-- STAGE 1: GET SET BRIEFING -->
        <div v-if="drillStage === 'ready'" class="space-y-3.5">
          <div class="w-full h-36 bg-white rounded-2xl p-2.5 flex items-center justify-center border border-slate-300 shadow-inner">
            <img :src="'/images/' + activeDrillExercise.image_path" :alt="activeDrillExercise.name" class="max-h-full max-w-full object-contain" />
          </div>

          <!-- Tactical Telemetry: Previous vs Entry vs Elite -->
          <div class="grid grid-cols-2 gap-2 text-left bg-slate-950/90 p-3 rounded-xl border border-slate-800 text-xs font-mono">
            <div>
              <span class="text-slate-500 block text-[10px] uppercase font-bold">Previous Sortie</span>
              <span class="text-amber-400 font-bold">
                {{ getLastPerformance(props.history || [], activeDrillExercise.exercise_number, cardioMode)?.display || 'None recorded' }}
              </span>
            </div>
            <div>
              <span class="text-slate-500 block text-[10px] uppercase font-bold">Entry Standard (D-)</span>
              <span class="text-emerald-400 font-bold">
                {{ getCandidateStandard(activeDrillExercise.exercise_number, 'entry') }}
              </span>
            </div>
          </div>

          <div class="bg-slate-950 p-3 rounded-2xl border border-slate-800 text-left">
            <div class="text-[10px] font-bold text-slate-400 uppercase tracking-wider mb-1">Technique Cue:</div>
            <p class="text-xs text-slate-300 leading-relaxed">{{ activeDrillExercise.instructions }}</p>
          </div>

          <button
            type="button"
            @click="beginDrillCountdown"
            class="w-full py-3.5 rounded-2xl bg-gradient-to-r from-emerald-500 to-cyan-500 hover:from-emerald-400 hover:to-cyan-400 text-slate-950 font-black text-sm uppercase tracking-wider shadow-lg shadow-emerald-500/20 active:scale-[0.98] transition cursor-pointer"
          >
            I Am Ready • Start Timer ⏱️
          </button>
        </div>

        <!-- STAGE 2: 3-2-1 COUNTDOWN -->
        <div v-else-if="drillStage === 'countdown'" class="py-10 space-y-4 flex flex-col items-center justify-center">
          <div class="text-xs font-bold text-slate-400 uppercase tracking-widest">Get Set...</div>
          <div class="text-7xl font-black text-cyan-400 animate-pulse tabular-nums">
            {{ drillCountdown }}
          </div>
          <p class="text-xs text-slate-400">Assume position and prepare for exercise initiation.</p>
        </div>

        <!-- STAGE 3: RUNNING DRILL TIMER -->
        <div v-else-if="drillStage === 'running'" class="py-3 space-y-4 flex flex-col items-center">
          <!-- Timer Display -->
          <div class="relative flex flex-col items-center justify-center">
            <div class="text-6xl font-black font-mono tracking-tight text-white tabular-nums">
              {{ precisionTimer.formattedRemaining.value }}
            </div>
            <div class="text-xs font-mono text-slate-400 mt-1">
              Elapsed: <span class="text-cyan-400 font-bold">{{ precisionTimer.formattedElapsed.value }}</span>
            </div>
          </div>

          <div class="flex items-center gap-2">
            <span class="w-2.5 h-2.5 rounded-full bg-emerald-400 animate-pulse"></span>
            <span class="text-xs font-semibold text-slate-300">
              {{ precisionTimer.isPaused.value ? 'Drill Paused' : 'Clock Running — Maintain Strict Cadence' }}
            </span>
          </div>

          <!-- Speed banner in running drill for roadwork/treadmill -->
          <div v-if="activeDrillExercise.exercise_number === 5 && cardioMode !== 'stationary' && candidateTreadmillSpeed.mph > 0" class="w-full p-2 rounded-xl bg-slate-950 border border-cyan-500/30 text-xs font-mono text-cyan-300 flex items-center justify-between">
            <span class="text-slate-400">Treadmill Target:</span>
            <span class="font-bold">≥ {{ candidateTreadmillSpeed.display }}</span>
          </div>

          <!-- Controls -->
          <div class="flex gap-3 w-full pt-1">
            <button
              type="button"
              @click="toggleDrillPause"
              class="flex-1 py-3 px-4 rounded-xl font-bold text-xs uppercase tracking-wider transition cursor-pointer"
              :class="precisionTimer.isPaused.value ? 'bg-emerald-500 text-slate-950' : 'bg-slate-800 hover:bg-slate-700 text-white'"
            >
              {{ precisionTimer.isPaused.value ? '▶ Resume Clock' : '⏸ Pause Clock' }}
            </button>

            <button
              type="button"
              @click="finishDrillEarly"
              class="flex-1 py-3 px-4 rounded-xl bg-cyan-600 hover:bg-cyan-500 text-slate-950 font-black text-xs uppercase tracking-wider transition cursor-pointer shadow"
            >
              Finish Drill Early ✓
            </button>
          </div>
        </div>

        <!-- STAGE 4: DRILL COMPLETE & PERFORMANCE ENTRY -->
        <div v-else-if="drillStage === 'record'" class="py-2 space-y-4 text-center">
          <div class="w-12 h-12 rounded-full bg-emerald-500/20 text-emerald-400 text-2xl flex items-center justify-center mx-auto">
            ✓
          </div>
          <h4 class="text-lg font-black text-white uppercase tracking-tight">
            Drill Complete!
          </h4>
          <p class="text-xs text-slate-400">
            Confirm your recorded performance within the official time envelope:
          </p>

          <!-- Input for Exercises 1 to 4 or Stationary 5 -->
          <div v-if="activeDrillExercise.exercise_number < 5 || cardioMode === 'stationary'" class="flex items-center justify-center gap-3 py-2">
            <button
              type="button"
              @click="drillRecordedReps = Math.max(0, drillRecordedReps - 1)"
              class="w-10 h-10 rounded-xl bg-slate-800 hover:bg-slate-700 text-white font-black text-lg flex items-center justify-center cursor-pointer"
            >-</button>
            <input
              v-model.number="drillRecordedReps"
              type="number"
              min="0"
              max="1500"
              class="w-24 bg-slate-950 border-2 border-emerald-500 rounded-2xl py-2.5 text-center font-black text-2xl text-emerald-400 focus:outline-none"
            />
            <button
              type="button"
              @click="drillRecordedReps = drillRecordedReps + 1"
              class="w-10 h-10 rounded-xl bg-slate-800 hover:bg-slate-700 text-white font-black text-lg flex items-center justify-center cursor-pointer"
            >+</button>
            <span class="text-xs text-slate-400 font-medium">
              {{ activeDrillExercise.exercise_number === 5 ? 'steps' : 'reps' }}
            </span>
          </div>

          <!-- Input for Cardio Run/Walk (mm:ss) -->
          <div v-else class="flex flex-col items-center justify-center gap-2 py-2">
            <div class="text-[11px] font-mono text-cyan-400 font-bold">
              Elapsed Time Captured: {{ drillRecordedMin }}m {{ drillRecordedSec }}s
            </div>
            <div class="flex items-center bg-slate-950 border-2 border-cyan-500 rounded-2xl px-4 py-2">
              <input
                v-model.number="drillRecordedMin"
                type="number"
                min="0"
                max="59"
                class="w-12 bg-transparent text-white font-mono font-black text-xl text-center focus:outline-none"
              />
              <span class="text-slate-400 text-sm font-bold px-1">m</span>
              <span class="text-slate-500 font-bold">:</span>
              <input
                v-model.number="drillRecordedSec"
                type="number"
                min="0"
                max="59"
                class="w-12 bg-transparent text-white font-mono font-black text-xl text-center focus:outline-none"
              />
              <span class="text-slate-400 text-sm font-bold px-1">s</span>
            </div>
          </div>

          <button
            type="button"
            @click="saveDrillPerformance"
            class="w-full py-3.5 rounded-2xl bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-black text-sm uppercase tracking-wider transition shadow-lg shadow-cyan-500/20 cursor-pointer"
          >
            Record to Assessment Sheet ➔
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
