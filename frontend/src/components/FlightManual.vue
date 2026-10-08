<script setup lang="ts">
import { ref } from 'vue';
import { UserProfile } from '../types';

defineProps<{
  profile: UserProfile | null;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
}>();

type ChapterTab = 'philosophy' | 'exercises' | 'progression' | 'cardio' | 'archive';
const activeChapter = ref<ChapterTab>('philosophy');

// Interactive Exercise Explorer State
const selectedChartForExercises = ref<number>(1);
const selectedExerciseNumber = ref<number>(1);

interface ExerciseGuideDetail {
  number: number;
  timeLimit: string;
  name: string;
  category: string;
  targetMuscles: string;
  executionCues: string[];
  safetyTips: string[];
  chartProgressionNote: string;
}

const exerciseGuides: Record<number, ExerciseGuideDetail> = {
  1: {
    number: 1,
    timeLimit: '2 Minutes',
    name: 'Flexibility & Spinal Articulation',
    category: 'Forward Bends & Toe Touches',
    targetMuscles: 'Hamstrings, lower lumbar spine, gluteal chain, calves',
    executionCues: [
      'Stand erect with feet approximately 12 inches (30 cm) apart.',
      'Inhale at the peak, then bend smoothly forward from the hips with relaxed knees.',
      'Reach downwards towards the deck without bouncing violently.',
      'Touch the specified standard (fingertips on Chart 1, knuckles on Chart 2, palms on Chart 3, floor between heels on Charts 4–6).',
      'Return to an upright standing posture and extend slightly backward before repeating.'
    ],
    safetyTips: [
      'Do not lock knee joints into hyper-extension if your hamstrings are tight.',
      'Maintain continuous, unhurried breathing; do not hold your breath during forward flexion.'
    ],
    chartProgressionNote: 'Chart 1: Fingertips touch toes → Chart 2: Knuckles touch deck → Chart 3: Palms touch deck → Chart 4: Touch floor then stretch back → Chart 5: Floor touch between feet with bounce.'
  },
  2: {
    number: 2,
    timeLimit: '1 Minute',
    name: 'Anterior Core & Abdominal Strength',
    category: 'Sit-Ups & Leg Raises',
    targetMuscles: 'Rectus abdominis, transversus abdominis, hip flexors',
    executionCues: [
      'Lie supine on your back with feet approximately 6 inches (15 cm) apart.',
      'Arms held either straight forward, across chest, or clasped behind neck according to chart instructions.',
      'Contract the core and roll upward smoothly, peeling vertebrae off the floor sequentially.',
      'Reach forward until your forehead or elbows touch the target position over your knees.',
      'Descend with strict eccentric control rather than collapsing backward onto the deck.'
    ],
    safetyTips: [
      'Anchor your heels firmly to the floor; do not jerk your neck or wrench the cervical spine.',
      'If feet lift uncontrollably, focus on abdominal bracing before initiating movement.'
    ],
    chartProgressionNote: 'Chart 1: Arms extended forward → Chart 2: Arms crossed over chest → Chart 3: Hands behind head → Chart 4: Straight leg tuck sit-ups → Chart 5: Leg lift scissors.'
  },
  3: {
    number: 3,
    timeLimit: '1 Minute',
    name: 'Posterior Chain & Lumbar Extension',
    category: 'Back Arches & Hyperextensions',
    targetMuscles: 'Erector spinae, multifidus, gluteus maximus, posterior deltoids',
    executionCues: [
      'Lie prone flat on your stomach with arms resting alongside your torso or behind your head.',
      'Simultaneously elevate your chest, shoulders, and thighs off the deck.',
      'Hold the apex extension briefly for a controlled pause, contracting your glutes and lower back.',
      'Lower under control until torso and thighs lightly touch the floor, then immediately initiate the next repetition.'
    ],
    safetyTips: [
      'Keep your cervical spine neutral by gazing downwards at the floor rather than cranking your neck upwards.',
      'Focus on length and controlled posterior contraction rather than violent hyperextension.'
    ],
    chartProgressionNote: 'Chart 1: Chest elevation only → Chart 2: Chest and thigh elevation → Chart 3: Hands behind neck with dual elevation → Chart 4: Arms outstretched overhead → Chart 5: Dynamic swimming extensions.'
  },
  4: {
    number: 4,
    timeLimit: '1 Minute',
    name: 'Upper Body Pushing Power',
    category: 'Push-Ups & Press Variations',
    targetMuscles: 'Pectoralis major, anterior deltoids, triceps brachii, core stabilisers',
    executionCues: [
      'Position hands slightly wider than shoulder-width, fingers pointing forward or slightly outward.',
      'Establish a rigid plank from shoulders to toes (or knees on Chart 1).',
      'Lower your entire body in one continuous unit until your sternum lightly grazes the deck or a fist-height off floor.',
      'Maintain elbows tucked at roughly 45 degrees to protect the shoulder capsule.',
      'Drive powerfully through your palms to full arm extension without sagging at the hips.'
    ],
    safetyTips: [
      'Avoid flared elbows (at 90 degrees to torso) to prevent anterior shoulder impingement.',
      'Do not allow the lumbar spine to sag into hyperextension; keep glutes and core braced.'
    ],
    chartProgressionNote: 'Chart 1: Modified knee push-ups → Chart 2: Standard toe push-ups → Chart 3: V-elevated push-ups → Chart 4: Semi-planche push-ups → Chart 5: Hand-slap & clapping push-ups.'
  },
  5: {
    number: 5,
    timeLimit: '6 Minutes',
    name: 'Aerobic Capacity & Cardio Engine',
    category: 'Stationary Running, Roadwork or Walking',
    targetMuscles: 'Cardiovascular system, soleus, gastrocnemius, quadriceps, lung capacity',
    executionCues: [
      'Stationary Running: Run in place, lifting each knee at least 4 inches (10 cm) off the floor.',
      'Cadence: Count each time the left foot strikes the floor.',
      'Scissor Jumps: Every 75 left-foot strikes, execute 10 scissor jumps (or straddle jumps on higher charts).',
      'Continuous Roadwork: Complete a 1-mile run (Charts 1–4) or 2-mile walk (Charts 1–4) within the specified target time.',
      'Treadmill Calibration: Maintain the calibrated pace without supporting weight on handrails.'
    ],
    safetyTips: [
      'Wear supportive footwear with cushioned soles if executing stationary running on hard flooring.',
      'Pace yourself steadily over the full 6-minute window rather than sprinting into oxygen debt in Minute 1.'
    ],
    chartProgressionNote: 'Chart 1: 100–400 steps + 10 jumps → Chart 2: 335–420 steps + 15 jumps → Chart 3: 390–475 steps + 20 jumps → Chart 4: 400–485 steps + 30 jumps → Chart 5: 400–575 steps + 50 jumps → Chart 6: 450–600 steps + 60 jumps.'
  }
};
</script>

<template>
  <div class="max-w-6xl mx-auto px-4 py-6 space-y-6">
    <!-- Header Banner -->
    <div class="bg-gradient-to-br from-slate-900 via-slate-950 to-slate-900 border border-slate-800 rounded-3xl p-6 sm:p-8 shadow-2xl relative overflow-hidden">
      <div class="absolute -right-16 -top-16 w-64 h-64 bg-cyan-500/10 rounded-full blur-3xl pointer-events-none"></div>
      <div class="absolute -left-16 -bottom-16 w-64 h-64 bg-blue-600/10 rounded-full blur-3xl pointer-events-none"></div>

      <div class="relative z-10 flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
        <div>
          <div class="flex items-center gap-2 mb-2">
            <span class="text-[10px] font-black uppercase tracking-widest px-2.5 py-0.5 rounded-full bg-cyan-950 border border-cyan-500/40 text-cyan-400">
              RCAF Physical Fitness Doctrine
            </span>
            <span class="text-[10px] font-mono text-slate-400">
              Pamphlet 30/1 • Revised Edition
            </span>
          </div>
          <h1 class="text-2xl sm:text-3xl font-black text-white tracking-tight">
            5BX Flight Manual
          </h1>
          <p class="text-xs sm:text-sm text-slate-300 mt-1 max-w-2xl leading-relaxed">
            The definitive technical guide to the Royal Canadian Air Force Five Basic Exercises physical fitness plan. 11 minutes a day, zero equipment, lifelong operational readiness.
          </p>
        </div>

        <div class="flex items-center gap-2 shrink-0">
          <a
            href="/docs/5bx-plan.pdf"
            download="5bx-plan-rcaf-1961.pdf"
            class="px-4 py-2 rounded-xl bg-slate-900 hover:bg-slate-800 border border-cyan-500/40 hover:border-cyan-400 text-cyan-300 font-bold text-xs flex items-center gap-2 transition-all shadow-sm cursor-pointer"
          >
            <span>📥</span>
            <span>Download 1961 PDF (3.5 MB)</span>
          </a>
          <button
            @click="emit('close')"
            class="px-3.5 py-2 rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-300 hover:text-white font-medium text-xs transition cursor-pointer"
          >
            Back to Mission
          </button>
        </div>
      </div>

      <!-- Chapter Tab Navigation Bar -->
      <div class="mt-6 flex flex-wrap gap-1.5 p-1.5 bg-slate-950/80 rounded-2xl border border-slate-800/80 text-xs">
        <button
          @click="activeChapter = 'philosophy'"
          class="flex-1 min-w-[140px] py-2 px-3 rounded-xl font-bold transition-all text-center cursor-pointer flex items-center justify-center gap-1.5"
          :class="activeChapter === 'philosophy' ? 'bg-cyan-500 text-slate-950 shadow-md shadow-cyan-500/20' : 'text-slate-400 hover:text-white'"
        >
          <span>🏛️</span>
          <span>1. Philosophy</span>
        </button>

        <button
          @click="activeChapter = 'exercises'"
          class="flex-1 min-w-[140px] py-2 px-3 rounded-xl font-bold transition-all text-center cursor-pointer flex items-center justify-center gap-1.5"
          :class="activeChapter === 'exercises' ? 'bg-cyan-500 text-slate-950 shadow-md shadow-cyan-500/20' : 'text-slate-400 hover:text-white'"
        >
          <span>🤸</span>
          <span>2. The 5 Exercises</span>
        </button>

        <button
          @click="activeChapter = 'progression'"
          class="flex-1 min-w-[140px] py-2 px-3 rounded-xl font-bold transition-all text-center cursor-pointer flex items-center justify-center gap-1.5"
          :class="activeChapter === 'progression' ? 'bg-cyan-500 text-slate-950 shadow-md shadow-cyan-500/20' : 'text-slate-400 hover:text-white'"
        >
          <span>📈</span>
          <span>3. Progression Rules</span>
        </button>

        <button
          @click="activeChapter = 'cardio'"
          class="flex-1 min-w-[140px] py-2 px-3 rounded-xl font-bold transition-all text-center cursor-pointer flex items-center justify-center gap-1.5"
          :class="activeChapter === 'cardio' ? 'bg-cyan-500 text-slate-950 shadow-md shadow-cyan-500/20' : 'text-slate-400 hover:text-white'"
        >
          <span>⏱️</span>
          <span>4. Cardio & Speeds</span>
        </button>

        <button
          @click="activeChapter = 'archive'"
          class="flex-1 min-w-[140px] py-2 px-3 rounded-xl font-bold transition-all text-center cursor-pointer flex items-center justify-center gap-1.5"
          :class="activeChapter === 'archive' ? 'bg-cyan-500 text-slate-950 shadow-md shadow-cyan-500/20' : 'text-slate-400 hover:text-white'"
        >
          <span>📜</span>
          <span>5. 1961 Archive PDF</span>
        </button>
      </div>
    </div>

    <!-- CHAPTER 1: THE RCAF PHILOSOPHY -->
    <section v-if="activeChapter === 'philosophy'" class="space-y-6">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div class="p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-2">
          <div class="text-2xl">⏱️</div>
          <h2 class="text-sm font-black text-white uppercase tracking-tight">The 11-Minute Covenant</h2>
          <p class="text-xs text-slate-400 leading-relaxed">
            The fundamental law of 5BX is that the workout length remains strictly fixed at 11 minutes. You do not graduate to longer workouts as you grow fitter; instead, you perform substantially more physical work within the exact same time envelope.
          </p>
        </div>

        <div class="p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-2">
          <div class="text-2xl">❄️</div>
          <h2 class="text-sm font-black text-white uppercase tracking-tight">Zero Equipment Friction</h2>
          <p class="text-xs text-slate-400 leading-relaxed">
            Engineered by Dr Bill Orban in the late 1950s for Royal Canadian Air Force pilots stationed in remote frozen outposts in northern Canada. No barbells, no gym contracts, no special attire. You require only an 8-foot strip of floor.
          </p>
        </div>

        <div class="p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-2">
          <div class="text-2xl">🪜</div>
          <h2 class="text-sm font-black text-white uppercase tracking-tight">A 72-Rung Ladder</h2>
          <p class="text-xs text-slate-400 leading-relaxed">
            The plan spans 6 progressive Charts, each containing 12 discrete Levels (D-, D, D+, C-, C, C+, B-, B, B+, A-, A, A+). This 72-step continuum guarantees seamless progressive overload from absolute novice to extreme elite physical condition.
          </p>
        </div>
      </div>

      <!-- Time Envelope Breakdown Table -->
      <div class="p-6 rounded-2xl bg-slate-900/90 border border-slate-800 space-y-4">
        <div class="flex items-center justify-between">
          <h2 class="text-sm font-black text-white uppercase tracking-tight flex items-center gap-2">
            <span>⏱️</span>
            <span>The 11-Minute Daily Tactical Allocation</span>
          </h2>
          <span class="text-[10px] font-mono text-cyan-400 font-bold bg-cyan-950 px-2 py-0.5 rounded border border-cyan-500/30">
            Total Elapsed Time: 11m 00s
          </span>
        </div>

        <div class="overflow-x-auto">
          <table class="w-full text-left text-xs border-collapse">
            <thead>
              <tr class="border-b border-slate-800 text-[10px] font-bold uppercase tracking-wider text-slate-400">
                <th class="py-2.5 px-3">Order</th>
                <th class="py-2.5 px-3">Discipline</th>
                <th class="py-2.5 px-3">Duration</th>
                <th class="py-2.5 px-3">Physiological Focus</th>
                <th class="py-2.5 px-3">Objective</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-850 font-mono text-[11px] text-slate-300">
              <tr class="hover:bg-slate-850/50">
                <td class="py-2 px-3 text-cyan-400 font-bold">1</td>
                <td class="py-2 px-3 font-bold text-white">Flexibility &amp; Warm-Up</td>
                <td class="py-2 px-3 text-emerald-400">2 Minutes</td>
                <td class="py-2 px-3">Spine, hamstrings, posterior chain</td>
                <td class="py-2 px-3 font-sans text-xs text-slate-400">Loosen joint capsules and elevate internal tissue temperature</td>
              </tr>
              <tr class="hover:bg-slate-850/50">
                <td class="py-2 px-3 text-cyan-400 font-bold">2</td>
                <td class="py-2 px-3 font-bold text-white">Anterior Abdominal Core</td>
                <td class="py-2 px-3 text-emerald-400">1 Minute</td>
                <td class="py-2 px-3">Rectus abdominis, hip flexors</td>
                <td class="py-2 px-3 font-sans text-xs text-slate-400">Forge abdominal wall strength and spinal protection</td>
              </tr>
              <tr class="hover:bg-slate-850/50">
                <td class="py-2 px-3 text-cyan-400 font-bold">3</td>
                <td class="py-2 px-3 font-bold text-white">Posterior Chain Endurance</td>
                <td class="py-2 px-3 text-emerald-400">1 Minute</td>
                <td class="py-2 px-3">Erector spinae, glutes, upper back</td>
                <td class="py-2 px-3 font-sans text-xs text-slate-400">Counteract postural slump and reinforce lumbar integrity</td>
              </tr>
              <tr class="hover:bg-slate-850/50">
                <td class="py-2 px-3 text-cyan-400 font-bold">4</td>
                <td class="py-2 px-3 font-bold text-white">Upper Body Pushing Power</td>
                <td class="py-2 px-3 text-emerald-400">1 Minute</td>
                <td class="py-2 px-3">Pectorals, deltoids, triceps</td>
                <td class="py-2 px-3 font-sans text-xs text-slate-400">Build muscular endurance and functional pushing force</td>
              </tr>
              <tr class="hover:bg-slate-850/50">
                <td class="py-2 px-3 text-cyan-400 font-bold">5</td>
                <td class="py-2 px-3 font-bold text-white">Cardiovascular Engine</td>
                <td class="py-2 px-3 text-emerald-400">6 Minutes</td>
                <td class="py-2 px-3">Heart, lungs, oxidative capacity</td>
                <td class="py-2 px-3 font-sans text-xs text-slate-400">Sustain high aerobic output and dynamic explosive jumping</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- The Orban Legacy Callout -->
      <div class="p-6 rounded-2xl bg-gradient-to-r from-blue-950/40 via-slate-900 to-cyan-950/40 border border-slate-800 space-y-3">
        <div class="flex items-center gap-2 text-cyan-400 font-bold text-xs uppercase tracking-wider">
          <span>🎖️</span> Historical Provenance
        </div>
        <blockquote class="text-sm text-slate-200 italic border-l-2 border-cyan-500 pl-4 py-1">
          "The 5BX Plan is designed to bring you to a level of physical fitness that will enable you to meet the physical demands of your daily work and play. If you perform these five exercises for just eleven minutes each day, you will achieve and maintain an extraordinary level of personal vitality."
        </blockquote>
        <div class="text-[11px] text-slate-400 font-mono text-right">
          — Dr Bill Orban, Director of Physical Fitness, Royal Canadian Air Force (1961)
        </div>
      </div>
    </section>

    <!-- CHAPTER 2: THE 5 EXERCISES -->
    <section v-else-if="activeChapter === 'exercises'" class="space-y-6">
      <!-- Exercise Selector Buttons -->
      <div class="grid grid-cols-2 sm:grid-cols-5 gap-2">
        <button
          v-for="num in [1, 2, 3, 4, 5]"
          :key="num"
          @click="selectedExerciseNumber = num"
          class="p-3 rounded-2xl border text-left transition-all cursor-pointer"
          :class="selectedExerciseNumber === num ? 'bg-cyan-500 text-slate-950 border-cyan-400 shadow-lg shadow-cyan-500/20' : 'bg-slate-900 text-slate-300 border-slate-800 hover:border-slate-700'"
        >
          <div class="text-[10px] font-mono uppercase font-bold" :class="selectedExerciseNumber === num ? 'text-slate-900' : 'text-cyan-400'">
            Ex {{ num }} • {{ exerciseGuides[num].timeLimit }}
          </div>
          <div class="text-xs font-black truncate mt-0.5">
            {{ exerciseGuides[num].name }}
          </div>
        </button>
      </div>

      <!-- Active Exercise Card -->
      <div class="p-6 rounded-3xl bg-slate-900 border border-slate-800 space-y-6">
        <div class="flex flex-col md:flex-row items-start md:items-center justify-between gap-4 border-b border-slate-800 pb-4">
          <div>
            <div class="flex items-center gap-2">
              <span class="text-xs font-mono font-black text-cyan-400 bg-cyan-950 border border-cyan-500/30 px-2 py-0.5 rounded">
                EXERCISE {{ selectedExerciseNumber }}
              </span>
              <span class="text-xs font-mono text-emerald-400 font-bold bg-emerald-950 border border-emerald-500/30 px-2 py-0.5 rounded">
                ⏱️ {{ exerciseGuides[selectedExerciseNumber].timeLimit }}
              </span>
            </div>
            <h2 class="text-xl font-black text-white mt-1.5">
              {{ exerciseGuides[selectedExerciseNumber].name }}
            </h2>
            <p class="text-xs text-slate-400 mt-0.5">
              Primary Focus: <strong class="text-slate-200">{{ exerciseGuides[selectedExerciseNumber].targetMuscles }}</strong>
            </p>
          </div>

          <!-- Chart Visual Progression Selector -->
          <div class="flex items-center gap-2 bg-slate-950 p-1.5 rounded-xl border border-slate-800 text-xs">
            <span class="text-[10px] font-bold uppercase tracking-wider text-slate-400 px-2">Visual Chart:</span>
            <button
              v-for="c in [1, 2, 3, 4, 5, 6]"
              :key="c"
              @click="selectedChartForExercises = c"
              class="w-7 h-7 rounded-lg font-mono font-bold text-xs flex items-center justify-center transition-all cursor-pointer"
              :class="selectedChartForExercises === c ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white bg-slate-900'"
            >
              {{ c }}
            </button>
          </div>
        </div>

        <!-- Illustration & Cues Grid -->
        <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
          <!-- Illustration Box -->
          <div class="lg:col-span-4 bg-slate-950 rounded-2xl border border-slate-800 p-4 flex flex-col items-center justify-center text-center space-y-3">
            <div class="w-full h-48 bg-white rounded-xl p-3 flex items-center justify-center border border-slate-300 shadow-inner">
              <img
                :src="`/images/c${selectedChartForExercises}_ex${selectedExerciseNumber}.png`"
                :alt="`Chart ${selectedChartForExercises} Exercise ${selectedExerciseNumber}`"
                class="max-h-full max-w-full object-contain"
              />
            </div>
            <div class="text-[10px] font-mono text-slate-400">
              Official 1961 Illustration • Chart {{ selectedChartForExercises }} • Exercise {{ selectedExerciseNumber }}
            </div>
            <div class="text-[11px] text-cyan-300 font-sans leading-snug px-2">
              {{ exerciseGuides[selectedExerciseNumber].chartProgressionNote }}
            </div>
          </div>

          <!-- Execution Standards & Safety Cues -->
          <div class="lg:col-span-8 space-y-4">
            <div class="p-4 rounded-xl bg-slate-950/70 border border-slate-800 space-y-2">
              <h3 class="text-xs font-black uppercase tracking-wider text-emerald-400 flex items-center gap-1.5">
                <span>🎯</span> Exact Execution Protocol
              </h3>
              <ul class="text-xs text-slate-300 space-y-2 leading-relaxed">
                <li v-for="(cue, idx) in exerciseGuides[selectedExerciseNumber].executionCues" :key="idx" class="flex items-start gap-2">
                  <span class="text-cyan-400 font-bold shrink-0">•</span>
                  <span>{{ cue }}</span>
                </li>
              </ul>
            </div>

            <div class="p-4 rounded-xl bg-amber-950/20 border border-amber-500/30 space-y-2">
              <h3 class="text-xs font-black uppercase tracking-wider text-amber-300 flex items-center gap-1.5">
                <span>⚠️</span> Safety &amp; Form Discipline
              </h3>
              <ul class="text-xs text-slate-300 space-y-1.5 leading-relaxed">
                <li v-for="(tip, idx) in exerciseGuides[selectedExerciseNumber].safetyTips" :key="idx" class="flex items-start gap-2">
                  <span class="text-amber-400 font-bold shrink-0">•</span>
                  <span>{{ tip }}</span>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- CHAPTER 3: PROGRESSION & DEMOTION RULES -->
    <section v-else-if="activeChapter === 'progression'" class="space-y-6">
      <!-- Golden Rule Banner -->
      <div class="p-6 rounded-3xl bg-gradient-to-br from-emerald-950/40 via-slate-900 to-slate-950 border border-emerald-500/40 space-y-3 shadow-xl">
        <div class="flex items-center gap-2">
          <span class="text-xs font-mono font-black uppercase tracking-wider text-emerald-400 bg-emerald-950 px-2.5 py-0.5 rounded border border-emerald-500/40">
            The Fundamental Axiom
          </span>
          <span class="text-xs font-bold text-slate-400">RCAF Rule #1</span>
        </div>
        <h2 class="text-xl sm:text-2xl font-black text-white">
          Meet the Target = Level Up. Miss by 1 Rep = Maintain.
        </h2>
        <p class="text-xs sm:text-sm text-slate-300 leading-relaxed max-w-3xl">
          Progression in 5BX is entirely meritocratic and strictly binary. If you achieve the prescribed repetition target or beat the target time for every exercise within your current level, you earn promotion to the next rung. Missing even a single repetition on a single exercise means you must maintain your position and conquer it tomorrow.
        </p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Dual Independent Ladders -->
        <div class="p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-3">
          <div class="flex items-center gap-2 text-cyan-400 font-bold text-xs uppercase tracking-wider">
            <span>⚔️</span> Split Tracks: Strength vs Cardio
          </div>
          <h3 class="text-sm font-black text-white">Independent Aerobic &amp; Muscular Progression</h3>
          <p class="text-xs text-slate-400 leading-relaxed">
            Human physiology is not monolithic. A pilot may possess powerful pushing strength while lagging in cardiovascular stamina, or vice versa. The modern 5BX system evaluates <strong>Strength Track (Exercises 1–4)</strong> and <strong>Cardio Track (Exercise 5)</strong> autonomously.
          </p>
          <div class="p-3 rounded-xl bg-slate-950 border border-slate-850 text-xs text-slate-300 space-y-1 font-mono text-[11px]">
            <div>• Strength: Must pass ALL 4 exercises (1, 2, 3, 4) to advance.</div>
            <div>• Cardio: Evaluated strictly on Exercise 5 steps or time.</div>
            <div class="text-cyan-400 pt-1">• Example: Strength Level Up 🚀 | Cardio Maintain 🛑</div>
          </div>
        </div>

        <!-- Leapfrog Engine -->
        <div class="p-5 rounded-2xl bg-slate-900 border border-slate-800 space-y-3">
          <div class="flex items-center gap-2 text-emerald-400 font-bold text-xs uppercase tracking-wider">
            <span>🚀</span> Leapfrog Mechanics
          </div>
          <h3 class="text-sm font-black text-white">Accelerated Promotion for Exceptional Sorties</h3>
          <p class="text-xs text-slate-400 leading-relaxed">
            If your physical performance in a mission exceeds your current target so dramatically that your reps qualify for higher rungs on your chart, the progression engine will <strong>Leapfrog</strong> you forward by multiple levels in a single day.
          </p>
          <div class="p-3 rounded-xl bg-slate-950 border border-slate-850 text-xs text-slate-300 space-y-1 font-mono text-[11px]">
            <div>• Standard Level Up: C2 B- → C2 B (1 step)</div>
            <div>• Leapfrog: C2 B- → C2 A+ (3 steps skipped)</div>
            <div class="text-emerald-400 pt-1">• Chart Boundary Gate: Leapfrogs stop at Level 12 (A+).</div>
          </div>
        </div>
      </div>

      <!-- Setbacks & Demotions: The 3-Strikes Rule -->
      <div class="p-6 rounded-3xl bg-slate-900 border border-slate-800 space-y-4">
        <div class="flex items-center gap-2 text-rose-400 font-bold text-xs uppercase tracking-wider">
          <span>📉</span> Operational Resilience &amp; Demotion Protocol
        </div>
        <h3 class="text-base font-black text-white">
          The "3 Strikes" Grace Doctrine &amp; The Soft Landing
        </h3>
        <p class="text-xs text-slate-300 leading-relaxed">
          Life involves fatigue, illness, and temporary setbacks. The 5BX system is engineered to challenge you without punitive discouragement. Daily missions operate under the <strong>3-Strikes Rule</strong>:
        </p>

        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 pt-1">
          <div class="p-4 rounded-xl bg-slate-950 border border-slate-800 space-y-1.5">
            <div class="text-xs font-mono font-bold text-amber-400">Sortie 1 Missed</div>
            <div class="text-sm font-black text-white">Strike 1 of 3</div>
            <p class="text-[11px] text-slate-400 leading-snug">
              Verdict: <code>MAINTAIN (1/3 Strikes)</code>. Active rung remains completely unchanged.
            </p>
          </div>

          <div class="p-4 rounded-xl bg-slate-950 border border-slate-800 space-y-1.5">
            <div class="text-xs font-mono font-bold text-amber-500">Sortie 2 Missed</div>
            <div class="text-sm font-black text-white">Strike 2 of 3</div>
            <p class="text-[11px] text-slate-400 leading-snug">
              Verdict: <code>MAINTAIN (2/3 Strikes)</code>. Warning state. Active rung remains completely intact.
            </p>
          </div>

          <div class="p-4 rounded-xl bg-slate-950 border border-rose-500/40 space-y-1.5">
            <div class="text-xs font-mono font-bold text-rose-400">Sortie 3 Missed</div>
            <div class="text-sm font-black text-white">Demotion Triggered</div>
            <p class="text-[11px] text-slate-400 leading-snug">
              Demoted to match actual demonstrated capacity. Failure counter resets upon first passed mission.
            </p>
          </div>
        </div>

        <!-- The Soft Landing Callout -->
        <div class="p-4 rounded-xl bg-slate-950 border border-cyan-500/30 space-y-2">
          <div class="text-xs font-bold text-cyan-300 flex items-center gap-1.5">
            <span>🪂</span> The "Soft Landing" Rule for Chart Boundaries
          </div>
          <p class="text-xs text-slate-300 leading-relaxed">
            You <strong>never drop a chart</strong> during a demotion unless you were already stationed at the absolute bottom of that chart (<strong>Level 1 / D-</strong>). If you fail 3 consecutive sorties at Level 1 (D-), the system parachutes you to <strong>Level 12 (A+)</strong> of the preceding chart. You strictly go down, but you land on your feet at the apex of the previous chart rather than in a downward spiral.
          </p>
        </div>

        <!-- Diagnostic Benchmark Difference -->
        <div class="p-4 rounded-xl bg-amber-950/20 border border-amber-500/30 space-y-2">
          <div class="text-xs font-bold text-amber-300 flex items-center gap-1.5">
            <span>🧭</span> Why Diagnostic Calibration Differs from Daily Missions
          </div>
          <p class="text-xs text-slate-300 leading-relaxed">
            The <strong>3-Strikes Rule</strong> protects your position strictly during daily workout sorties. In contrast, the <strong>Diagnostic Placement Assessment</strong> ("Test Current Fitness") is an objective calibration benchmark against an entire target chart. Because each chart introduces more demanding movements, all 4 strength exercises must clear the entry target simultaneously.
          </p>
          <p class="text-xs text-slate-300 leading-relaxed">
            If you miss any target during a diagnostic assessment (such as following a training layoff), the system honestly calibrates you to the summit of the preceding chart (<strong>Chart X - 1, Level 12 A+</strong>). This honest demotion ensures you build back tendon, joint, and cardiovascular conditioning safely before re-attempting advanced movements.
          </p>
          <p class="text-xs text-slate-400 leading-relaxed">
            <em>Flight Log Note:</em> Deleting an erroneous mission from your flight debrief automatically restores your pilot rung level to its pre-workout state without requiring manual intervention.
          </p>
        </div>
      </div>
    </section>

    <!-- CHAPTER 4: CARDIO & ROADWORK SPEEDS -->
    <section v-else-if="activeChapter === 'cardio'" class="space-y-6">
      <div class="p-6 rounded-3xl bg-slate-900 border border-slate-800 space-y-4">
        <div class="flex items-center justify-between">
          <h2 class="text-base font-black text-white uppercase tracking-tight flex items-center gap-2">
            <span>🏃</span>
            <span>Cardio Disciplines &amp; Roadwork Conversions</span>
          </h2>
          <span class="text-[10px] font-mono text-cyan-400 font-bold bg-cyan-950 px-2 py-0.5 rounded border border-cyan-500/30">
            Indoor &amp; Outdoor Interchangeable
          </span>
        </div>
        <p class="text-xs text-slate-300 leading-relaxed">
          While Dr Orban designed the 6-minute stationary run as the default indoor protocol, the 1961 RCAF manual explicitly permits outdoor or treadmill continuous running and walking on Charts 1 to 4 as valid aerobic substitutions.
        </p>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-3 pt-2">
          <div class="p-4 rounded-xl bg-slate-950 border border-slate-800 space-y-1.5">
            <div class="text-xs font-mono font-bold text-cyan-400">Indoor Stationary Run</div>
            <div class="text-sm font-black text-white">6 Minutes Flat</div>
            <p class="text-[11px] text-slate-400 leading-snug">
              Count each left foot strike. Every 75 steps, execute scissor or straddle jumps. Zero travel required.
            </p>
          </div>

          <div class="p-4 rounded-xl bg-slate-950 border border-slate-800 space-y-1.5">
            <div class="text-xs font-mono font-bold text-emerald-400">Continuous 1-Mile Run (1.6 km)</div>
            <div class="text-sm font-black text-white">Outdoor or Treadmill</div>
            <p class="text-[11px] text-slate-400 leading-snug">
              Charts 1 to 4. Beat the clock (e.g. Chart 3 entry: ≤ 8m 45s; Chart 3 elite: ≤ 8m 00s).
            </p>
          </div>

          <div class="p-4 rounded-xl bg-slate-950 border border-slate-800 space-y-1.5">
            <div class="text-xs font-mono font-bold text-amber-400">Continuous 2-Mile Walk (3.2 km)</div>
            <div class="text-sm font-black text-white">Vigorous Power Cadence</div>
            <p class="text-[11px] text-slate-400 leading-snug">
              Charts 1 to 4. Aerobic walking with arms swinging freely (e.g. Chart 3 entry: ≤ 29m 00s; Chart 3 elite: ≤ 25m 00s).
            </p>
          </div>
        </div>
      </div>

      <!-- Treadmill Speed Calibration Table -->
      <div class="p-6 rounded-3xl bg-slate-900 border border-slate-800 space-y-4">
        <h3 class="text-sm font-black text-white uppercase tracking-tight flex items-center gap-2">
          <span>⚙️</span>
          <span>Treadmill Speed &amp; Cadence Standards</span>
        </h3>
        <p class="text-xs text-slate-300 leading-relaxed">
          When executing roadwork disciplines on a gymnasium or home treadmill, set your belt velocity to meet or exceed these exact speed thresholds without holding the console handrails:
        </p>

        <div class="overflow-x-auto">
          <table class="w-full text-left text-xs border-collapse">
            <thead>
              <tr class="border-b border-slate-800 text-[10px] font-bold uppercase tracking-wider text-slate-400">
                <th class="py-2 px-3">Chart</th>
                <th class="py-2 px-3">Discipline</th>
                <th class="py-2 px-3">Distance</th>
                <th class="py-2 px-3">Target Time</th>
                <th class="py-2 px-3 text-cyan-400">Speed (km/h)</th>
                <th class="py-2 px-3 text-emerald-400">Speed (mph)</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-slate-850 font-mono text-[11px] text-slate-300">
              <tr class="hover:bg-slate-850/50">
                <td class="py-2 px-3 font-bold text-white">Chart 1</td>
                <td class="py-2 px-3">Continuous Run</td>
                <td class="py-2 px-3">0.5 mi (0.8 km)</td>
                <td class="py-2 px-3">8m 00s</td>
                <td class="py-2 px-3 text-cyan-400 font-bold">6.0 km/h</td>
                <td class="py-2 px-3 text-emerald-400 font-bold">3.8 mph</td>
              </tr>
              <tr class="hover:bg-slate-850/50">
                <td class="py-2 px-3 font-bold text-white">Chart 1</td>
                <td class="py-2 px-3">Continuous Walk</td>
                <td class="py-2 px-3">1.0 mi (1.6 km)</td>
                <td class="py-2 px-3">21m 00s</td>
                <td class="py-2 px-3 text-cyan-400 font-bold">4.6 km/h</td>
                <td class="py-2 px-3 text-emerald-400 font-bold">2.9 mph</td>
              </tr>
              <tr class="hover:bg-slate-850/50">
                <td class="py-2 px-3 font-bold text-white">Chart 2</td>
                <td class="py-2 px-3">Continuous Run</td>
                <td class="py-2 px-3">1.0 mi (1.6 km)</td>
                <td class="py-2 px-3">8m 45s – 8m 00s</td>
                <td class="py-2 px-3 text-cyan-400 font-bold">11.0 – 12.1 km/h</td>
                <td class="py-2 px-3 text-emerald-400 font-bold">6.9 – 7.5 mph</td>
              </tr>
              <tr class="hover:bg-slate-850/50">
                <td class="py-2 px-3 font-bold text-white">Chart 2</td>
                <td class="py-2 px-3">Continuous Walk</td>
                <td class="py-2 px-3">2.0 mi (3.2 km)</td>
                <td class="py-2 px-3">30m 00s – 26m 00s</td>
                <td class="py-2 px-3 text-cyan-400 font-bold">6.4 – 7.4 km/h</td>
                <td class="py-2 px-3 text-emerald-400 font-bold">4.0 – 4.6 mph</td>
              </tr>
              <tr class="hover:bg-slate-850/50">
                <td class="py-2 px-3 font-bold text-white">Chart 3</td>
                <td class="py-2 px-3">Continuous Run</td>
                <td class="py-2 px-3">1.0 mi (1.6 km)</td>
                <td class="py-2 px-3">8m 45s – 8m 00s</td>
                <td class="py-2 px-3 text-cyan-400 font-bold">11.0 – 12.1 km/h</td>
                <td class="py-2 px-3 text-emerald-400 font-bold">6.9 – 7.5 mph</td>
              </tr>
              <tr class="hover:bg-slate-850/50">
                <td class="py-2 px-3 font-bold text-white">Chart 3</td>
                <td class="py-2 px-3">Continuous Walk</td>
                <td class="py-2 px-3">2.0 mi (3.2 km)</td>
                <td class="py-2 px-3">29m 00s – 25m 00s</td>
                <td class="py-2 px-3 text-cyan-400 font-bold">6.7 – 7.7 km/h</td>
                <td class="py-2 px-3 text-emerald-400 font-bold">4.1 – 4.8 mph</td>
              </tr>
              <tr class="hover:bg-slate-850/50">
                <td class="py-2 px-3 font-bold text-white">Chart 4</td>
                <td class="py-2 px-3">Continuous Run</td>
                <td class="py-2 px-3">1.0 mi (1.6 km)</td>
                <td class="py-2 px-3">7m 45s – 7m 00s</td>
                <td class="py-2 px-3 text-cyan-400 font-bold">12.5 – 13.8 km/h</td>
                <td class="py-2 px-3 text-emerald-400 font-bold">7.7 – 8.6 mph</td>
              </tr>
              <tr class="hover:bg-slate-850/50">
                <td class="py-2 px-3 font-bold text-white">Chart 4</td>
                <td class="py-2 px-3">Continuous Walk</td>
                <td class="py-2 px-3">2.0 mi (3.2 km)</td>
                <td class="py-2 px-3">27m 00s – 23m 00s</td>
                <td class="py-2 px-3 text-cyan-400 font-bold">7.2 – 8.4 km/h</td>
                <td class="py-2 px-3 text-emerald-400 font-bold">4.4 – 5.2 mph</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </section>

    <!-- CHAPTER 5: 1961 ARCHIVAL MANUAL (EMBEDDED PDF) -->
    <section v-else-if="activeChapter === 'archive'" class="space-y-6">
      <div class="p-6 rounded-3xl bg-slate-900 border border-slate-800 space-y-4">
        <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
          <div>
            <h2 class="text-base font-black text-white uppercase tracking-tight flex items-center gap-2">
              <span>📜</span>
              <span>RCAF Pamphlet 30/1 (1961 Official Publication)</span>
            </h2>
            <p class="text-xs text-slate-400 mt-0.5">
              The original historical documentation authored by the Royal Canadian Air Force and Queen's Printer, Ottawa.
            </p>
          </div>

          <a
            href="/docs/5bx-plan.pdf"
            download="5bx-plan-rcaf-1961.pdf"
            class="px-4 py-2 rounded-xl bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-black text-xs uppercase tracking-wider flex items-center gap-2 transition shadow-md shadow-cyan-500/20 cursor-pointer"
          >
            <span>📥</span>
            <span>Download PDF (3.5 MB)</span>
          </a>
        </div>

        <!-- Embedded PDF Viewer -->
        <div class="w-full h-[700px] rounded-2xl overflow-hidden border border-slate-800 bg-slate-950 flex flex-col">
          <object
            data="/docs/5bx-plan.pdf#toolbar=1&navpanes=1&scrollbar=1"
            type="application/pdf"
            class="w-full h-full"
          >
            <div class="p-8 text-center space-y-4 flex flex-col items-center justify-center h-full">
              <span class="text-4xl">📄</span>
              <p class="text-sm text-slate-300 font-bold">
                Your browser does not support inline PDF viewing.
              </p>
              <p class="text-xs text-slate-400 max-w-md">
                You can download the authentic 1961 RCAF 5BX Pamphlet 30/1 directly to read offline or in your preferred PDF application.
              </p>
              <a
                href="/docs/5bx-plan.pdf"
                download="5bx-plan-rcaf-1961.pdf"
                class="px-5 py-2.5 rounded-xl bg-cyan-500 text-slate-950 font-bold text-xs uppercase tracking-wider shadow cursor-pointer"
              >
                Download Pamphlet 30/1 (3.5 MB)
              </a>
            </div>
          </object>
        </div>
      </div>
    </section>
  </div>
</template>
