<script setup lang="ts">
import { ref } from 'vue';
import { UserProfile } from '../types';

defineProps<{
  profile: UserProfile | null;
  activeTab: string;
  layoutPreference?: 'auto' | 'streamlined' | 'comprehensive';
}>();

const emit = defineEmits<{
  (e: 'navigate', tab: string): void;
  (e: 'open-login'): void;
  (e: 'open-adjust'): void;
  (e: 'open-assessment'): void;
  (e: 'open-badges'): void;
  (e: 'open-sheet'): void;
  (e: 'open-settings'): void;
  (e: 'logout'): void;
  (e: 'set-layout-preference', pref: 'auto' | 'streamlined' | 'comprehensive'): void;
}>();

const isMenuOpen = ref(false);
</script>

<template>
  <header class="no-print bg-slate-950/85 backdrop-blur-md border-b border-slate-800/80 sticky top-0 z-40 transition-all select-none">
    <div class="max-w-6xl mx-auto px-4 py-2.5 flex items-center justify-between">
      <!-- Brand Emblem & Identity -->
      <div class="flex items-center gap-3 cursor-pointer" @click="emit('navigate', 'workout')">
        <div class="w-9 h-9 rounded-xl bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center font-black text-slate-950 text-sm shadow-md shadow-cyan-500/20 tracking-tighter shrink-0">
          5BX
        </div>
        <div>
          <div class="flex items-center gap-1.5 leading-none">
            <h1 class="text-sm font-black text-white uppercase tracking-tight">RCAF 5BX</h1>
            <span class="text-[9px] font-bold uppercase tracking-wider px-1.5 py-0.2 rounded bg-cyan-950 border border-cyan-500/30 text-cyan-400">
              Protocol
            </span>
          </div>
          <p class="text-[10px] text-slate-400 font-medium mt-0.5 hidden sm:block">11 Minutes • Zero Equipment</p>
        </div>
      </div>

      <!-- Desktop Primary Navigation Tabs (Strictly 3 clean destinations) -->
      <nav v-if="profile" class="hidden md:flex items-center gap-1 bg-slate-900/90 p-1 rounded-xl border border-slate-800">
        <button
          @click="emit('navigate', 'workout')"
          class="px-4 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer"
          :class="activeTab === 'workout' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          Daily Mission
        </button>
        <button
          @click="emit('navigate', 'charts')"
          class="px-4 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer"
          :class="activeTab === 'charts' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          System Charts
        </button>
        <button
          @click="emit('navigate', 'history')"
          class="px-4 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer"
          :class="activeTab === 'history' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          Flight Log
        </button>
      </nav>

      <!-- Public Navigation (When logged out) -->
      <nav v-else class="hidden md:flex items-center gap-4 text-xs font-semibold text-slate-300">
        <button @click="emit('navigate', 'charts')" class="hover:text-cyan-400 transition-colors cursor-pointer">
          System Charts (1–6)
        </button>
        <button @click="emit('navigate', 'sheet')" class="hover:text-cyan-400 transition-colors cursor-pointer">
          Single-Sheet Gym Form
        </button>
      </nav>

      <!-- Right Side: Unified Pilot Menu or Sign In -->
      <div class="flex items-center gap-2">
        <!-- Logged-in Pilot Profile Flyout Button -->
        <div v-if="profile" class="relative">
          <button
            @click="isMenuOpen = !isMenuOpen"
            class="flex items-center gap-2 px-3 py-1.5 rounded-xl bg-slate-900 hover:bg-slate-850 border border-slate-800 hover:border-slate-700 text-white transition-all cursor-pointer shadow-sm"
            :class="{ 'border-cyan-500/50 bg-slate-850': isMenuOpen }"
          >
            <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
            <span class="text-xs font-bold tracking-tight">{{ profile.username }}</span>
            <span class="text-[10px] font-mono text-cyan-300 bg-cyan-950/80 px-1.5 py-0.2 rounded border border-cyan-500/30">
              C{{ profile.strength_chart }} {{ profile.strength_level_display }}
            </span>
            <svg
              class="w-3.5 h-3.5 text-slate-400 transition-transform duration-200"
              :class="{ 'rotate-180': isMenuOpen }"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
          </button>

          <!-- Backdrop to close on click outside -->
          <div v-if="isMenuOpen" class="fixed inset-0 z-40" @click="isMenuOpen = false"></div>

          <!-- Pilot Settings & Status Flyout Dropdown -->
          <div
            v-if="isMenuOpen"
            class="absolute right-0 mt-2 w-80 bg-slate-900/95 backdrop-blur-xl border border-slate-700/80 rounded-2xl p-4 shadow-2xl z-50 text-left text-xs space-y-3.5"
          >
            <!-- Pilot Header Card -->
            <div class="p-3 bg-slate-950 rounded-xl border border-slate-800">
              <div class="flex items-center justify-between">
                <div>
                  <div class="font-black text-white text-sm">{{ profile.username }}</div>
                  <div class="text-[10px] text-slate-400 font-mono">{{ profile.email }}</div>
                </div>
                <span class="text-[10px] font-mono bg-slate-800 text-slate-300 px-2 py-0.5 rounded font-bold">
                  Age {{ profile.age }}
                </span>
              </div>

              <!-- Dual Ladder Standings -->
              <div class="mt-2.5 grid grid-cols-2 gap-2 text-[10px] font-mono">
                <div class="bg-slate-900/90 p-2 rounded-lg border border-slate-800">
                  <span class="text-slate-400 block text-[9px] uppercase font-bold">Strength Rung</span>
                  <span class="text-emerald-400 font-black text-xs">
                    Chart {{ profile.strength_chart }} • {{ profile.strength_level_display }}
                  </span>
                </div>
                <div class="bg-slate-900/90 p-2 rounded-lg border border-slate-800">
                  <span class="text-slate-400 block text-[9px] uppercase font-bold">Cardio Rung</span>
                  <span class="text-cyan-400 font-black text-xs">
                    Chart {{ profile.cardio_chart }} • {{ profile.cardio_level_display }}
                  </span>
                </div>
              </div>

              <div class="mt-1.5 text-[9px] text-slate-400 font-mono text-center">
                Target Standard: Chart {{ profile.age_target_chart }} • {{ profile.age_target_display }}
              </div>
            </div>

            <!-- Quick Action Links -->
            <div class="space-y-1">
              <button
                @click="emit('open-assessment'); isMenuOpen = false"
                class="w-full text-left px-3 py-2 rounded-xl hover:bg-slate-800 text-slate-200 hover:text-white flex items-center gap-3 transition-colors cursor-pointer"
              >
                <span class="text-base">🧭</span>
                <div>
                  <div class="font-bold text-xs">Assess Level & Layoff</div>
                  <div class="text-[10px] text-slate-400">Establish entry rung or recalibrate after absence</div>
                </div>
              </button>

              <button
                @click="emit('open-adjust'); isMenuOpen = false"
                class="w-full text-left px-3 py-2 rounded-xl hover:bg-slate-800 text-slate-200 hover:text-white flex items-center gap-3 transition-colors cursor-pointer"
              >
                <span class="text-base">⚙️</span>
                <div>
                  <div class="font-bold text-xs">Manual Ladder Override</div>
                  <div class="text-[10px] text-slate-400">Manually adjust strength or cardio levels</div>
                </div>
              </button>

              <button
                @click="emit('open-sheet'); isMenuOpen = false"
                class="w-full text-left px-3 py-2 rounded-xl hover:bg-slate-800 text-slate-200 hover:text-white flex items-center gap-3 transition-colors cursor-pointer"
              >
                <span class="text-base">📄</span>
                <div>
                  <div class="font-bold text-xs">Single-Sheet Gym Form</div>
                  <div class="text-[10px] text-slate-400">Printable offline workout scorecard</div>
                </div>
              </button>

              <button
                @click="emit('open-badges'); isMenuOpen = false"
                class="w-full text-left px-3 py-2 rounded-xl hover:bg-slate-800 text-slate-200 hover:text-white flex items-center gap-3 transition-colors cursor-pointer"
              >
                <span class="text-base">🏆</span>
                <div>
                  <div class="font-bold text-xs">Milestones & Honours</div>
                  <div class="text-[10px] text-slate-400">View flight achievements and wings</div>
                </div>
              </button>

              <button
                @click="emit('open-settings'); isMenuOpen = false"
                class="w-full text-left px-3 py-2 rounded-xl hover:bg-slate-800 text-slate-200 hover:text-white flex items-center gap-3 transition-colors cursor-pointer"
              >
                <span class="text-base">🔐</span>
                <div>
                  <div class="font-bold text-xs">Security &amp; Account</div>
                  <div class="text-[10px] text-slate-400">Change password or manage account</div>
                </div>
              </button>
            </div>

            <!-- Display Layout Preference -->
            <div class="pt-2.5 border-t border-slate-800">
              <div class="text-[10px] font-bold uppercase tracking-wider text-slate-400 mb-1.5 px-1 flex items-center justify-between">
                <span>Display Layout</span>
                <span class="text-[9px] text-slate-500 font-normal">Auto switches by screen size</span>
              </div>
              <div class="grid grid-cols-3 gap-1 bg-slate-950 p-1 rounded-xl border border-slate-800 text-center">
                <button
                  type="button"
                  @click="emit('set-layout-preference', 'auto')"
                  class="py-1 px-1 rounded-lg text-[10px] font-bold transition-all cursor-pointer"
                  :class="layoutPreference === 'auto' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
                >
                  Auto
                </button>
                <button
                  type="button"
                  @click="emit('set-layout-preference', 'streamlined')"
                  class="py-1 px-1 rounded-lg text-[10px] font-bold transition-all cursor-pointer"
                  :class="layoutPreference === 'streamlined' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
                >
                  Cards
                </button>
                <button
                  type="button"
                  @click="emit('set-layout-preference', 'comprehensive')"
                  class="py-1 px-1 rounded-lg text-[10px] font-bold transition-all cursor-pointer"
                  :class="layoutPreference === 'comprehensive' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
                >
                  Full
                </button>
              </div>
            </div>

            <!-- Sign Out -->
            <div class="pt-2.5 border-t border-slate-800">
              <button
                @click="emit('logout'); isMenuOpen = false"
                class="w-full py-2 px-3 text-center rounded-xl bg-slate-950 hover:bg-red-950/60 border border-slate-800 hover:border-red-500/40 text-slate-400 hover:text-red-300 font-bold transition-colors cursor-pointer"
              >
                Sign Out
              </button>
            </div>
          </div>
        </div>

        <!-- Logged Out Sign In Action -->
        <div v-else class="flex items-center gap-2">
          <button
            @click="emit('open-login')"
            class="px-4 py-2 rounded-xl bg-gradient-to-r from-cyan-500 to-blue-600 hover:from-cyan-400 hover:to-blue-500 text-slate-950 font-black text-xs uppercase tracking-wider shadow-md shadow-cyan-500/20 transition-all cursor-pointer"
          >
            Pilot Sign In
          </button>
        </div>
      </div>
    </div>

    <!-- Mobile Bottom Navigation Bar (Visible strictly on small screens < 768px when authenticated) -->
    <nav
      v-if="profile"
      class="md:hidden fixed bottom-0 left-0 right-0 z-30 bg-slate-950/95 backdrop-blur-lg border-t border-slate-800/80 px-4 py-2 flex items-center justify-around select-none"
    >
      <button
        @click="emit('navigate', 'workout')"
        class="flex flex-col items-center gap-1 text-[11px] font-bold py-1 px-4 rounded-xl transition-colors cursor-pointer"
        :class="activeTab === 'workout' ? 'text-cyan-400' : 'text-slate-400 hover:text-slate-200'"
      >
        <span class="text-base">🎯</span>
        <span>Mission</span>
      </button>
      <button
        @click="emit('navigate', 'charts')"
        class="flex flex-col items-center gap-1 text-[11px] font-bold py-1 px-4 rounded-xl transition-colors cursor-pointer"
        :class="activeTab === 'charts' ? 'text-cyan-400' : 'text-slate-400 hover:text-slate-200'"
      >
        <span class="text-base">📊</span>
        <span>Charts</span>
      </button>
      <button
        @click="emit('navigate', 'history')"
        class="flex flex-col items-center gap-1 text-[11px] font-bold py-1 px-4 rounded-xl transition-colors cursor-pointer"
        :class="activeTab === 'history' ? 'text-cyan-400' : 'text-slate-400 hover:text-slate-200'"
      >
        <span class="text-base">📜</span>
        <span>Flight Log</span>
      </button>
    </nav>
  </header>
</template>

