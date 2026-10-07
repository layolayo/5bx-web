<script setup lang="ts">
import { UserProfile } from '../types';

defineProps<{
  profile: UserProfile | null;
  activeTab: string;
  isKissMode?: boolean;
}>();

const emit = defineEmits<{
  (e: 'navigate', tab: string): void;
  (e: 'open-login'): void;
  (e: 'open-adjust'): void;
  (e: 'logout'): void;
  (e: 'toggle-kiss'): void;
}>();
</script>

<template>
  <header class="no-print bg-slate-950/80 backdrop-blur-md border-b border-slate-800/80 sticky top-0 z-40 transition-all">
    <div class="max-w-6xl mx-auto px-4 py-3 flex items-center justify-between">
      <!-- Brand Emblem & Identity -->
      <div class="flex items-center gap-3 cursor-pointer select-none" @click="emit('navigate', 'workout')">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center font-black text-slate-950 text-base shadow-lg shadow-cyan-500/20 tracking-tighter">
          5BX
        </div>
        <div>
          <div class="flex items-center gap-2">
            <h1 class="text-base font-black text-white uppercase tracking-tight leading-none">RCAF 5BX</h1>
            <span class="text-[9px] font-bold uppercase tracking-wider px-1.5 py-0.5 rounded bg-cyan-950 border border-cyan-500/30 text-cyan-400">
              Protocol
            </span>
          </div>
          <p class="text-[11px] text-slate-400 font-medium">11 Minutes • Zero Equipment</p>
        </div>
      </div>

      <!-- Navigation Tabs (When signed in) -->
      <nav v-if="profile" class="hidden md:flex items-center gap-1 bg-slate-900/80 p-1 rounded-xl border border-slate-800">
        <button
          @click="emit('navigate', 'workout')"
          class="px-3 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer"
          :class="activeTab === 'workout' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          Daily Mission
        </button>
        <button
          @click="emit('navigate', 'charts')"
          class="px-3 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer"
          :class="activeTab === 'charts' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          System Charts
        </button>
        <button
          @click="emit('navigate', 'sheet')"
          class="px-3 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer"
          :class="activeTab === 'sheet' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          Print Gym Sheet
        </button>
        <button
          @click="emit('navigate', 'history')"
          class="px-3 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer"
          :class="activeTab === 'history' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          Flight Log
        </button>
        <button
          @click="emit('navigate', 'badges')"
          class="px-3 py-1.5 rounded-lg text-xs font-bold transition-all cursor-pointer"
          :class="activeTab === 'badges' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          Milestones
        </button>
      </nav>

      <!-- Public Nav (When logged out) -->
      <nav v-else class="hidden md:flex items-center gap-4 text-xs font-semibold text-slate-300">
        <button @click="emit('navigate', 'charts')" class="hover:text-cyan-400 transition-colors cursor-pointer">
          System Charts (1–6)
        </button>
        <button @click="emit('navigate', 'sheet')" class="hover:text-cyan-400 transition-colors cursor-pointer">
          Single-Sheet Gym Form
        </button>
      </nav>

      <!-- User Account Cockpit Actions -->
      <div class="flex items-center gap-2">
        <!-- KISS Mode Toggle Button -->
        <button
          @click="emit('toggle-kiss')"
          :title="isKissMode ? 'Switch to Full Cockpit Mode' : 'Switch to Streamlined KISS Mobile Cockpit'"
          class="px-2.5 py-1.5 rounded-xl border text-xs font-bold transition-all cursor-pointer flex items-center gap-1.5"
          :class="isKissMode ? 'bg-cyan-500/20 border-cyan-500/50 text-cyan-300' : 'bg-slate-900 border-slate-800 text-slate-400 hover:text-white'"
        >
          <span>📱</span>
          <span class="hidden sm:inline">{{ isKissMode ? 'KISS Mode' : 'KISS Layout' }}</span>
        </button>

        <!-- Logged In Pilot Profile Card -->
        <div v-if="profile" class="flex items-center gap-2.5">
          <div class="text-right hidden sm:block">
            <div class="text-xs font-black text-white flex items-center gap-1.5 justify-end">
              <span class="text-emerald-400 text-[10px]">●</span>
              <span>{{ profile.username }}</span>
              <span class="bg-slate-800 text-slate-300 px-1.5 py-0.2 rounded text-[10px] font-mono">Age {{ profile.age }}</span>
            </div>
            <div class="text-[10px] font-mono text-slate-400">
              S: <span class="text-emerald-400 font-bold">C{{ profile.strength_chart }} {{ profile.strength_level_display }}</span>
              <span class="text-slate-600 mx-1">|</span>
              C: <span class="text-cyan-400 font-bold">C{{ profile.cardio_chart }} {{ profile.cardio_level_display }}</span>
            </div>
          </div>

          <button
            @click="emit('open-adjust')"
            title="Adjust Starting Chart & Level"
            class="p-2 text-slate-400 hover:text-white rounded-lg bg-slate-900 border border-slate-800 hover:border-slate-700 text-xs transition-colors cursor-pointer"
          >
            ⚙️
          </button>

          <button
            @click="emit('logout')"
            class="text-xs font-bold bg-slate-900 hover:bg-slate-800 text-slate-300 hover:text-white px-3 py-1.5 rounded-lg border border-slate-800 transition-colors cursor-pointer"
          >
            Sign Out
          </button>
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

    <!-- Mobile Subnav Bar for logged-in users -->
    <div v-if="profile" class="md:hidden flex items-center justify-around bg-slate-950 border-t border-slate-800/60 py-2 px-2 text-center">
      <button
        @click="emit('navigate', 'workout')"
        class="text-xs font-bold px-2 py-1 rounded"
        :class="activeTab === 'workout' ? 'text-cyan-400' : 'text-slate-400'"
      >
        Mission
      </button>
      <button
        @click="emit('navigate', 'charts')"
        class="text-xs font-bold px-2 py-1 rounded"
        :class="activeTab === 'charts' ? 'text-cyan-400' : 'text-slate-400'"
      >
        Charts
      </button>
      <button
        @click="emit('navigate', 'sheet')"
        class="text-xs font-bold px-2 py-1 rounded"
        :class="activeTab === 'sheet' ? 'text-cyan-400' : 'text-slate-400'"
      >
        Print
      </button>
      <button
        @click="emit('navigate', 'history')"
        class="text-xs font-bold px-2 py-1 rounded"
        :class="activeTab === 'history' ? 'text-cyan-400' : 'text-slate-400'"
      >
        Flight Log
      </button>
      <button
        @click="emit('navigate', 'badges')"
        class="text-xs font-bold px-2 py-1 rounded"
        :class="activeTab === 'badges' ? 'text-cyan-400' : 'text-slate-400'"
      >
        Wings
      </button>
    </div>
  </header>
</template>
