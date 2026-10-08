<script setup lang="ts">
import { ref } from 'vue';

const emit = defineEmits<{
  (e: 'open-login'): void;
  (e: 'start-guest-workout'): void;
  (e: 'open-charts'): void;
  (e: 'open-manual'): void;
}>();

// Interactive Age Standards Explorer Data
interface AgeStandard {
  range: string;
  chart: number;
  level: string;
  chartLevel: number;
  targetStrength: string;
  targetCardio: string;
  ex1: number;
  ex2: number;
  ex3: number;
  ex4: number;
  ex5: string;
  flyingCrew: string;
}

const ageStandards: Record<string, AgeStandard> = {
  'under20': {
    range: 'Under 20',
    chart: 5,
    level: 'C+',
    chartLevel: 7,
    targetStrength: 'Chart 5 • Level 7 (C+)',
    targetCardio: 'Chart 5 • Level 7 (C+)',
    ex1: 30, ex2: 31, ex3: 45, ex4: 38, ex5: '535 runs + 50 jumps',
    flyingCrew: 'Chart 6 • Level 3 (D+)'
  },
  '20-29': {
    range: '20–29',
    chart: 5,
    level: 'D-',
    chartLevel: 1,
    targetStrength: 'Chart 5 • Level 1 (D-)',
    targetCardio: 'Chart 5 • Level 1 (D-)',
    ex1: 28, ex2: 27, ex3: 40, ex4: 30, ex5: '475 runs + 45 jumps',
    flyingCrew: 'Chart 6 • Level 1 (D-)'
  },
  '30-34': {
    range: '30–34',
    chart: 4,
    level: 'C+',
    chartLevel: 7,
    targetStrength: 'Chart 4 • Level 7 (C+)',
    targetCardio: 'Chart 4 • Level 7 (C+)',
    ex1: 28, ex2: 27, ex3: 39, ex4: 28, ex5: '470 runs + 45 jumps',
    flyingCrew: 'Chart 5 • Level 4 (C-)'
  },
  '35-39': {
    range: '35–39',
    chart: 4,
    level: 'D-',
    chartLevel: 1,
    targetStrength: 'Chart 4 • Level 1 (D-)',
    targetCardio: 'Chart 4 • Level 1 (D-)',
    ex1: 26, ex2: 24, ex3: 35, ex4: 23, ex5: '425 runs + 40 jumps',
    flyingCrew: 'Chart 5 • Level 1 (D-)'
  },
  '40-44': {
    range: '40–44',
    chart: 3,
    level: 'B-',
    chartLevel: 8,
    targetStrength: 'Chart 3 • Level 8 (B-)',
    targetCardio: 'Chart 3 • Level 8 (B-)',
    ex1: 27, ex2: 22, ex3: 28, ex4: 19, ex5: '415 runs',
    flyingCrew: 'Chart 4 • Level 7 (C+)'
  },
  '45-49': {
    range: '45–49',
    chart: 3,
    level: 'C-',
    chartLevel: 5,
    targetStrength: 'Chart 3 • Level 5 (C-)',
    targetCardio: 'Chart 3 • Level 5 (C-)',
    ex1: 24, ex2: 19, ex3: 23, ex4: 15, ex5: '385 runs',
    flyingCrew: 'Chart 4 • Level 4 (C-)'
  },
  '50-59': {
    range: '50–59',
    chart: 3,
    level: 'D',
    chartLevel: 7,
    targetStrength: 'Chart 3 • Level 7 (D)',
    targetCardio: 'Chart 3 • Level 7 (D)',
    ex1: 26, ex2: 21, ex3: 26, ex4: 17, ex5: '400 runs',
    flyingCrew: 'Chart 4 • Level 1 (D-)'
  },
  '60plus': {
    range: '60+',
    chart: 2,
    level: 'A+',
    chartLevel: 12,
    targetStrength: 'Chart 2 • Level 12 (A+)',
    targetCardio: 'Chart 2 • Level 12 (A+)',
    ex1: 24, ex2: 19, ex3: 24, ex4: 15, ex5: '375 runs',
    flyingCrew: 'Chart 3 • Level 7 (D)'
  }
};

const selectedAge = ref<string>('50-59');

const exercises = [
  {
    num: 1,
    name: 'Forward Bend & Stretch',
    duration: '2 Minutes',
    muscles: 'Spine, Hamstrings & Lower Back',
    desc: 'Stand erect, feet 12 inches apart. Bend forward touching floor between feet, then stretch back upwards. Calibrated for spinal decompression.',
    image: '/images/c1_ex1.png'
  },
  {
    num: 2,
    name: 'Sit-Up',
    duration: '1 Minute',
    muscles: 'Abdominals & Core Stability',
    desc: 'Lie on back, feet 6 inches apart. Roll to sitting position, reach forward past toes, and roll smoothly back down.',
    image: '/images/c1_ex2.png'
  },
  {
    num: 3,
    name: 'Back Extension',
    duration: '1 Minute',
    muscles: 'Spinal Extensors & Posterior Chain',
    desc: 'Lie face down, palms under thighs. Raise head, shoulders, and chest up high off the floor while keeping legs straight.',
    image: '/images/c1_ex3.png'
  },
  {
    num: 4,
    name: 'Push-Up',
    duration: '1 Minute',
    muscles: 'Pectorals, Triceps & Anterior Deltoids',
    desc: 'From prone position, push body up straight keeping back rigid and pelvis aligned, touching chin to floor on descent.',
    image: '/images/c1_ex4.png'
  },
  {
    num: 5,
    name: 'Stationary Run & Jumps',
    duration: '6 Minutes',
    muscles: 'Cardiovascular & Calves',
    desc: 'Run on the spot lifting feet 4 inches, with scissor jumps or semi-squat hops interspersed every 75 steps to fortify aerobic endurance.',
    image: '/images/c1_ex5.png'
  }
];
</script>

<template>
  <div class="relative overflow-hidden">
    <!-- Hero Atmospheric Glow Background -->
    <div class="absolute -top-32 left-1/2 -translate-x-1/2 w-[800px] h-[400px] bg-gradient-to-b from-cyan-500/20 via-blue-600/10 to-transparent blur-3xl pointer-events-none -z-10"></div>
    <div class="absolute top-96 right-10 w-[500px] h-[400px] bg-emerald-500/10 blur-3xl pointer-events-none -z-10"></div>

    <!-- Main Hero Section -->
    <section class="max-w-6xl mx-auto px-4 pt-12 pb-16 sm:pt-20 sm:pb-24 text-center">
      <!-- Eyebrow Badge -->
      <div class="inline-flex items-center gap-2.5 px-3.5 py-1.5 rounded-full bg-cyan-950/60 border border-cyan-500/30 text-cyan-300 text-xs font-semibold tracking-wider uppercase mb-6 shadow-sm">
        <span class="w-2 h-2 rounded-full bg-cyan-400 animate-pulse"></span>
        <span>Royal Canadian Air Force Physical Fitness Protocol</span>
      </div>

      <!-- Hero Title -->
      <h1 class="text-4xl sm:text-6xl md:text-7xl font-black tracking-tight text-white uppercase leading-[1.08] mb-6">
        Eleven Minutes a Day.<br />
        <span class="text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 via-teal-300 to-emerald-400">
          Zero Equipment.
        </span><br />
        Peerless Fitness.
      </h1>

      <!-- Subtitle -->
      <p class="max-w-2xl mx-auto text-base sm:text-lg text-slate-300 font-normal leading-relaxed mb-10">
        Created in 1961 by Dr Bill Orban for RCAF fighter pilots stationed in remote Arctic outposts.
        Scientifically calibrated progression to elevate anyone from complete inactivity to supersonic combat readiness.
      </p>

      <!-- Primary Action Buttons (Standardised Sizing) -->
      <div class="flex flex-wrap items-center justify-center gap-3.5 max-w-2xl mx-auto mb-12">
        <button
          @click="emit('start-guest-workout')"
          class="btn-control-primary bg-gradient-to-r from-emerald-500 to-teal-600 hover:from-emerald-400 hover:to-teal-500 text-slate-950 shadow-lg shadow-emerald-500/25 pulse-radar"
        >
          <span>⚡</span>
          <span>Test Drive 11-Minute Workout</span>
        </button>

        <button
          @click="emit('open-login')"
          class="btn-control-secondary bg-slate-800 hover:bg-slate-700 border border-slate-700 text-white"
        >
          <span>🎖️</span>
          <span>Pilot Sign In / Register</span>
        </button>

        <button
          @click="emit('open-charts')"
          class="btn-control-secondary bg-slate-900/90 hover:bg-slate-800 border border-slate-700 text-cyan-300"
        >
          <span>📊</span>
          <span>System Charts (1–6)</span>
        </button>

        <button
          @click="emit('open-manual')"
          class="btn-control-secondary bg-slate-900/90 hover:bg-slate-800 border border-cyan-500/40 text-cyan-300"
        >
          <span>📖</span>
          <span>RCAF Flight Manual</span>
        </button>
      </div>
    </section>

    <!-- The 5 Exercises Section -->
    <section class="max-w-6xl mx-auto px-4 py-12 border-t border-slate-800/80">
      <div class="text-center max-w-3xl mx-auto mb-10">
        <div class="text-xs font-bold text-cyan-400 uppercase tracking-widest mb-2">The Architecture of Conditioning</div>
        <h2 class="text-3xl sm:text-4xl font-black text-white uppercase">Five Basic Exercises. Full Spectrum Fitness.</h2>
        <p class="text-slate-400 text-sm sm:text-base mt-2">
          Executed strictly in sequence without rest intervals, targeting flexibility, abdominal strength, spinal extension, pushing power, and aerobic capacity.
        </p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-5 gap-4">
        <div
          v-for="m in exercises"
          :key="m.num"
          class="glass-panel glass-panel-hover rounded-2xl p-4 flex flex-col justify-between"
        >
          <div>
            <div class="flex items-center justify-between mb-3">
              <span class="w-7 h-7 rounded-lg bg-cyan-950 border border-cyan-500/30 text-cyan-400 text-xs font-black flex items-center justify-center">
                {{ m.num }}
              </span>
              <span class="text-[11px] font-mono font-bold text-amber-400 bg-amber-950/40 px-2 py-0.5 rounded border border-amber-500/20">
                {{ m.duration }}
              </span>
            </div>

            <!-- Historical RCAF Exercise Diagram -->
            <div class="w-full h-28 bg-white rounded-xl p-2 flex items-center justify-center mb-3 shadow-inner">
              <img :src="m.image" :alt="m.name" class="max-h-full max-w-full object-contain" />
            </div>

            <h3 class="text-sm font-bold text-white leading-tight mb-1">{{ m.name }}</h3>
            <div class="text-[10px] font-semibold text-cyan-300/80 uppercase tracking-wide mb-2">{{ m.muscles }}</div>
            <p class="text-xs text-slate-400 leading-relaxed">{{ m.desc }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Interactive Age Standards Explorer -->
    <section class="max-w-6xl mx-auto px-4 py-12 border-t border-slate-800/80">
      <div class="bg-gradient-to-br from-slate-900 via-slate-850 to-slate-900 border border-slate-700/80 rounded-3xl p-6 sm:p-10 shadow-2xl">
        <!-- Section Header -->
        <div class="mb-6">
          <span class="text-xs font-bold text-emerald-400 uppercase tracking-widest block mb-1">RCAF Calibration Standard</span>
          <h2 class="text-2xl sm:text-3xl font-black text-white uppercase">What Is Your Physical Goal?</h2>
          <p class="text-slate-300 text-xs sm:text-sm mt-1">
            Select your age bracket to inspect your official daily maintenance target and Flying Crew benchmark.
          </p>
        </div>

        <!-- Dedicated Segmented Age Control Bar -->
        <div class="bg-slate-950/80 p-1.5 rounded-2xl border border-slate-800 shadow-inner mb-8">
          <div class="grid grid-cols-4 sm:grid-cols-8 gap-1.5 sm:gap-2">
            <button
              v-for="(val, key) in ageStandards"
              :key="key"
              @click="selectedAge = key"
              class="py-2.5 px-1 sm:px-2 rounded-xl transition-all cursor-pointer text-center flex items-center justify-center select-none"
              :class="selectedAge === key
                ? 'bg-cyan-500 text-slate-950 shadow-md shadow-cyan-500/25 font-black ring-1 ring-cyan-400'
                : 'text-slate-400 hover:text-white hover:bg-slate-850 font-bold'"
            >
              <span class="text-[10px] sm:text-xs whitespace-nowrap">{{ val.range }}</span>
            </button>
          </div>
        </div>

        <!-- Selected Age Details Card -->
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <!-- Main Level Badge -->
          <div class="bg-slate-950/80 border border-slate-800 rounded-2xl p-6 flex flex-col justify-between">
            <div>
              <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider block mb-1">Target Maintenance Level</span>
              <div class="text-3xl font-black text-emerald-400 mb-1">
                {{ ageStandards[selectedAge].targetStrength }}
              </div>
              <p class="text-xs text-slate-400 leading-relaxed">
                Reaching this standard satisfies the physical maintenance criteria of the Royal Canadian Air Force for your age group.
              </p>
            </div>

            <div class="mt-6 pt-4 border-t border-slate-800">
              <span class="text-[10px] font-bold text-amber-400 uppercase tracking-wider block mb-1">Flying Crew Elite Benchmark</span>
              <div class="text-base font-bold text-white">{{ ageStandards[selectedAge].flyingCrew }}</div>
              <span class="text-[11px] text-slate-500">Rigorous qualification required for supersonic flight operations.</span>
            </div>
          </div>

          <!-- Rep Breakdown Grid -->
          <div class="lg:col-span-2 bg-slate-950/80 border border-slate-800 rounded-2xl p-6">
            <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider block mb-4">Daily Rep Targets for {{ ageStandards[selectedAge].range }}</span>

            <div class="grid grid-cols-2 sm:grid-cols-5 gap-3">
              <div class="bg-slate-900 border border-slate-800 rounded-xl p-3 text-center">
                <span class="text-[10px] text-slate-400 uppercase block font-semibold">Ex 1: Bends</span>
                <span class="text-2xl font-black text-white tabular-nums">{{ ageStandards[selectedAge].ex1 }}</span>
                <span class="text-[10px] text-slate-500 block">in 2 mins</span>
              </div>

              <div class="bg-slate-900 border border-slate-800 rounded-xl p-3 text-center">
                <span class="text-[10px] text-slate-400 uppercase block font-semibold">Ex 2: Sit-Ups</span>
                <span class="text-2xl font-black text-white tabular-nums">{{ ageStandards[selectedAge].ex2 }}</span>
                <span class="text-[10px] text-slate-500 block">in 1 min</span>
              </div>

              <div class="bg-slate-900 border border-slate-800 rounded-xl p-3 text-center">
                <span class="text-[10px] text-slate-400 uppercase block font-semibold">Ex 3: Arches</span>
                <span class="text-2xl font-black text-white tabular-nums">{{ ageStandards[selectedAge].ex3 }}</span>
                <span class="text-[10px] text-slate-500 block">in 1 min</span>
              </div>

              <div class="bg-slate-900 border border-slate-800 rounded-xl p-3 text-center">
                <span class="text-[10px] text-slate-400 uppercase block font-semibold">Ex 4: Push-Ups</span>
                <span class="text-2xl font-black text-white tabular-nums">{{ ageStandards[selectedAge].ex4 }}</span>
                <span class="text-[10px] text-slate-500 block">in 1 min</span>
              </div>

              <div class="col-span-2 sm:col-span-1 bg-slate-900 border border-slate-800 rounded-xl p-3 text-center">
                <span class="text-[10px] text-slate-400 uppercase block font-semibold">Ex 5: Cardio</span>
                <span class="text-sm font-bold text-cyan-400 block mt-1 leading-tight">{{ ageStandards[selectedAge].ex5 }}</span>
                <span class="text-[10px] text-slate-500 block mt-1">in 6 mins</span>
              </div>
            </div>

            <div class="mt-5 flex items-center justify-between bg-slate-900/60 p-3 rounded-xl border border-slate-800 text-xs text-slate-300">
              <span>Ready to start your progression from Chart 1?</span>
              <button
                @click="emit('start-guest-workout')"
                class="px-4 py-1.5 rounded-lg bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-bold transition-colors cursor-pointer"
              >
                Launch Now
              </button>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- The Three Golden Rules -->
    <section class="max-w-6xl mx-auto px-4 py-12 border-t border-slate-800/80">
      <div class="text-center max-w-2xl mx-auto mb-10">
        <h2 class="text-2xl sm:text-3xl font-black text-white uppercase">The Progression Rules</h2>
        <p class="text-slate-400 text-xs sm:text-sm mt-1">A transparent algorithm governing your promotion across the 72 rungs.</p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-5">
        <div class="glass-panel rounded-2xl p-6 border-l-4 border-l-emerald-500">
          <div class="text-2xl mb-2">⚡</div>
          <h3 class="text-base font-bold text-white mb-1">1. The Golden Rule</h3>
          <p class="text-xs text-slate-300 leading-relaxed">
            Hit your required reps for all exercises today and you earn an automatic promotion to the next sub-level tomorrow. Consistent daily effort is immediately rewarded.
          </p>
        </div>

        <div class="glass-panel rounded-2xl p-6 border-l-4 border-l-cyan-500">
          <div class="text-2xl mb-2">🚀</div>
          <h3 class="text-base font-bold text-white mb-1">2. Leapfrog Logic</h3>
          <p class="text-xs text-slate-300 leading-relaxed">
            Exceeding your current targets allows you to leapfrog multiple levels within your chart. The system immediately matches your true physical capacity.
          </p>
        </div>

        <div class="glass-panel rounded-2xl p-6 border-l-4 border-l-amber-500">
          <div class="text-2xl mb-2">🛡️</div>
          <h3 class="text-base font-bold text-white mb-1">3. Three-Strikes Grace</h3>
          <p class="text-xs text-slate-300 leading-relaxed">
            If you miss your target for three consecutive workouts, the protocol gracefully lowers your ladder rung by one level, safeguarding against fatigue and injury.
          </p>
        </div>
      </div>
    </section>
  </div>
</template>
