<script setup lang="ts">
import { UserProfile } from '../types';

defineProps<{
  profile: UserProfile | null;
  activeTab: string;
}>();

const emit = defineEmits<{
  (e: 'navigate', tab: string): void;
  (e: 'open-login'): void;
  (e: 'open-adjust'): void;
  (e: 'logout'): void;
}>();
</script>

<template>
  <header class="no-print bg-slate-900 border-b border-slate-800 sticky top-0 z-40">
    <div class="max-w-5xl mx-auto px-4 py-3 flex items-center justify-between">
      <!-- Brand Logo & Title -->
      <div class="flex items-center gap-3 cursor-pointer" @click="emit('navigate', 'workout')">
        <div class="w-10 h-10 rounded-xl bg-blue-600 flex items-center justify-center font-black text-white text-lg shadow-lg">
          5BX
        </div>
        <div>
          <h1 class="text-lg font-bold text-white leading-tight">RCAF 5BX Plan</h1>
          <p class="text-[11px] text-slate-400">11 Minutes • No Equipment</p>
        </div>
      </div>

      <!-- Navigation Tabs -->
      <nav class="hidden sm:flex items-center gap-1 bg-slate-800/60 p-1 rounded-xl border border-slate-700/60">
        <button
          @click="emit('navigate', 'workout')"
          class="px-3 py-1.5 rounded-lg text-xs font-semibold transition-colors"
          :class="activeTab === 'workout' ? 'bg-blue-600 text-white shadow' : 'text-slate-300 hover:text-white'"
        >
          Today's Plan
        </button>
        <button
          @click="emit('navigate', 'sheet')"
          class="px-3 py-1.5 rounded-lg text-xs font-semibold transition-colors"
          :class="activeTab === 'sheet' ? 'bg-blue-600 text-white shadow' : 'text-slate-300 hover:text-white'"
        >
          Print Form
        </button>
        <button
          @click="emit('navigate', 'history')"
          class="px-3 py-1.5 rounded-lg text-xs font-semibold transition-colors"
          :class="activeTab === 'history' ? 'bg-blue-600 text-white shadow' : 'text-slate-300 hover:text-white'"
        >
          History
        </button>
        <button
          @click="emit('navigate', 'badges')"
          class="px-3 py-1.5 rounded-lg text-xs font-semibold transition-colors"
          :class="activeTab === 'badges' ? 'bg-blue-600 text-white shadow' : 'text-slate-300 hover:text-white'"
        >
          Milestones
        </button>
      </nav>

      <!-- User Account Actions -->
      <div class="flex items-center gap-2">
        <div v-if="profile" class="flex items-center gap-2">
          <div class="text-right hidden sm:block">
            <div class="text-xs font-bold text-white flex items-center gap-1.5 justify-end">
              <span>{{ profile.username }}</span>
              <span class="bg-blue-500/20 text-blue-400 px-1.5 py-0.5 rounded text-[10px]">Age {{ profile.age }}</span>
            </div>
            <div class="text-[10px] text-slate-400">
              S: <span class="text-emerald-400 font-semibold">C{{ profile.strength_chart }} {{ profile.strength_level_display }}</span> |
              C: <span class="text-blue-400 font-semibold">C{{ profile.cardio_chart }} {{ profile.cardio_level_display }}</span>
            </div>
          </div>
          <button
            @click="emit('open-adjust')"
            title="Adjust Level"
            class="p-2 text-slate-400 hover:text-white rounded-lg hover:bg-slate-800 text-xs"
          >
            ⚙️
          </button>
          <button
            @click="emit('logout')"
            class="text-xs font-semibold bg-slate-800 hover:bg-slate-700 text-slate-300 hover:text-white px-2.5 py-1.5 rounded-lg border border-slate-700"
          >
            Logout
          </button>
        </div>
        <div v-else>
          <button
            @click="emit('open-login')"
            class="bg-blue-600 hover:bg-blue-500 text-white text-xs font-bold px-3.5 py-2 rounded-lg shadow"
          >
            Sign In
          </button>
        </div>
      </div>
    </div>

    <!-- Mobile Nav Bar (Bottom or secondary row) -->
    <div class="sm:hidden flex border-t border-slate-800 bg-slate-900/90 px-2 py-1 justify-around text-xs">
      <button
        @click="emit('navigate', 'workout')"
        class="py-1 px-2 font-medium"
        :class="activeTab === 'workout' ? 'text-blue-400 font-bold' : 'text-slate-400'"
      >
        Today
      </button>
      <button
        @click="emit('navigate', 'sheet')"
        class="py-1 px-2 font-medium"
        :class="activeTab === 'sheet' ? 'text-blue-400 font-bold' : 'text-slate-400'"
      >
        Print Form
      </button>
      <button
        @click="emit('navigate', 'history')"
        class="py-1 px-2 font-medium"
        :class="activeTab === 'history' ? 'text-blue-400 font-bold' : 'text-slate-400'"
      >
        History
      </button>
      <button
        @click="emit('navigate', 'badges')"
        class="py-1 px-2 font-medium"
        :class="activeTab === 'badges' ? 'text-blue-400 font-bold' : 'text-slate-400'"
      >
        Badges
      </button>
    </div>
  </header>
</template>
