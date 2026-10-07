<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  initialUsername?: string;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'login', payload: { username_or_email: string; password: string }): void;
  (e: 'register', payload: any): void;
}>();

const mode = ref<'login' | 'register'>('login');

const username = ref(props.initialUsername || '');
const password = ref('');
const email = ref('');
const dob = ref('1990-01-01');
const errorMsg = ref('');

watch(() => props.initialUsername, (newVal) => {
  if (newVal) {
    username.value = newVal;
  }
});

function submitLogin() {
  if (!username.value || !password.value) {
    errorMsg.value = 'Please provide your pilot username/email and access password.';
    return;
  }
  emit('login', { username_or_email: username.value, password: password.value });
}

function submitRegister() {
  if (!username.value || !email.value || !password.value || !dob.value) {
    errorMsg.value = 'All fields are mandatory to establish a new flight profile.';
    return;
  }
  emit('register', {
    username: username.value,
    email: email.value,
    password: password.value,
    dob: dob.value,
  });
}
</script>

<template>
  <div class="fixed inset-0 z-50 bg-slate-950/85 backdrop-blur-md flex items-center justify-center p-4">
    <div class="bg-slate-900 border border-slate-700/80 rounded-3xl w-full max-w-md p-6 sm:p-8 shadow-2xl relative overflow-hidden">
      <!-- Glow ambient accent -->
      <div class="absolute -top-20 -right-20 w-48 h-48 bg-cyan-500/10 blur-2xl pointer-events-none"></div>

      <!-- Modal Header -->
      <div class="flex justify-between items-center pb-4 border-b border-slate-800 mb-5">
        <div class="flex items-center gap-2.5">
          <div class="w-8 h-8 rounded-lg bg-cyan-500/20 border border-cyan-500/40 text-cyan-400 flex items-center justify-center text-sm font-black">
            ✈️
          </div>
          <div>
            <h2 class="text-lg font-black text-white uppercase tracking-tight">
              {{ mode === 'login' ? 'Pilot Authentication' : 'Enlist New Pilot' }}
            </h2>
            <p class="text-[11px] text-slate-400">Royal Canadian Air Force 5BX Service</p>
          </div>
        </div>
        <button
          @click="emit('close')"
          class="w-8 h-8 rounded-full bg-slate-800 hover:bg-slate-700 text-slate-400 hover:text-white flex items-center justify-center text-sm transition-colors cursor-pointer"
        >
          ✕
        </button>
      </div>

      <!-- Mode Switch Pills -->
      <div class="flex bg-slate-950/80 p-1 rounded-xl mb-5 border border-slate-800">
        <button
          @click="mode = 'login'; errorMsg = ''"
          class="flex-1 py-2 text-xs font-bold rounded-lg transition-all cursor-pointer"
          :class="mode === 'login' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          Sign In
        </button>
        <button
          @click="mode = 'register'; errorMsg = ''"
          class="flex-1 py-2 text-xs font-bold rounded-lg transition-all cursor-pointer"
          :class="mode === 'register' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          Create Profile
        </button>
      </div>

      <!-- Error Message Banner -->
      <div v-if="errorMsg" class="mb-4 bg-red-500/15 border border-red-500/30 text-red-300 text-xs p-3 rounded-xl flex items-center gap-2">
        <span>⚠️</span>
        <span>{{ errorMsg }}</span>
      </div>

      <!-- Login Form -->
      <form v-if="mode === 'login'" @submit.prevent="submitLogin" class="space-y-4">
        <div>
          <label class="text-xs font-bold text-slate-300 uppercase tracking-wider block mb-1.5">
            Callsign or Email
          </label>
          <input
            v-model="username"
            type="text"
            required
            placeholder="e.g. Matthew or matthew@5bx.local"
            class="w-full bg-slate-950 border border-slate-700 rounded-xl p-3 text-white text-sm focus:border-cyan-500 focus:outline-none focus:ring-1 focus:ring-cyan-500 transition-all"
          />
        </div>

        <div>
          <label class="text-xs font-bold text-slate-300 uppercase tracking-wider block mb-1.5">
            Password
          </label>
          <input
            v-model="password"
            type="password"
            required
            placeholder="••••••••"
            class="w-full bg-slate-950 border border-slate-700 rounded-xl p-3 text-white text-sm focus:border-cyan-500 focus:outline-none focus:ring-1 focus:ring-cyan-500 transition-all font-mono"
          />
        </div>

        <button
          type="submit"
          class="w-full py-3.5 rounded-xl bg-gradient-to-r from-cyan-500 to-blue-600 hover:from-cyan-400 hover:to-blue-500 text-slate-950 font-black text-sm uppercase tracking-wider shadow-lg shadow-cyan-500/20 transition-all cursor-pointer mt-2"
        >
          Authenticate & Access Mission
        </button>
      </form>

      <!-- Register Form -->
      <form v-else @submit.prevent="submitRegister" class="space-y-3">
        <div>
          <label class="text-xs font-bold text-slate-300 uppercase tracking-wider block mb-1">
            Pilot Callsign (Username)
          </label>
          <input
            v-model="username"
            type="text"
            required
            placeholder="e.g. Maverick"
            class="w-full bg-slate-950 border border-slate-700 rounded-xl p-2.5 text-white text-sm focus:border-cyan-500 focus:outline-none"
          />
        </div>

        <div>
          <label class="text-xs font-bold text-slate-300 uppercase tracking-wider block mb-1">
            Email Address
          </label>
          <input
            v-model="email"
            type="email"
            required
            placeholder="pilot@example.com"
            class="w-full bg-slate-950 border border-slate-700 rounded-xl p-2.5 text-white text-sm focus:border-cyan-500 focus:outline-none"
          />
        </div>

        <div>
          <label class="text-xs font-bold text-slate-300 uppercase tracking-wider block mb-1">
            Password
          </label>
          <input
            v-model="password"
            type="password"
            required
            placeholder="Minimum 8 characters"
            class="w-full bg-slate-950 border border-slate-700 rounded-xl p-2.5 text-white text-sm focus:border-cyan-500 focus:outline-none"
          />
        </div>

        <div>
          <label class="text-xs font-bold text-slate-300 uppercase tracking-wider block mb-1">
            Date of Birth (Determines Calibration Benchmark)
          </label>
          <input
            v-model="dob"
            type="date"
            required
            class="w-full bg-slate-950 border border-slate-700 rounded-xl p-2.5 text-white text-sm focus:border-cyan-500 focus:outline-none"
          />
        </div>

        <button
          type="submit"
          class="w-full py-3.5 rounded-xl bg-gradient-to-r from-emerald-500 to-teal-600 hover:from-emerald-400 hover:to-teal-500 text-slate-950 font-black text-sm uppercase tracking-wider shadow-lg shadow-emerald-500/20 transition-all cursor-pointer mt-2"
        >
          Enlist & Initialise Ladder
        </button>
      </form>
    </div>
  </div>
</template>
