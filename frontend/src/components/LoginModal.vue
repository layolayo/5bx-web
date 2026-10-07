<script setup lang="ts">
import { ref } from 'vue';

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'login', payload: { username_or_email: string; password: string }): void;
  (e: 'register', payload: any): void;
}>();

const mode = ref<'login' | 'register'>('login');

const username = ref('');
const password = ref('');
const email = ref('');
const dob = ref('1990-01-01');
const errorMsg = ref('');

function submitLogin() {
  if (!username.value || !password.value) {
    errorMsg.value = 'Please enter both username/email and password.';
    return;
  }
  emit('login', { username_or_email: username.value, password: password.value });
}

function submitRegister() {
  if (!username.value || !email.value || !password.value || !dob.value) {
    errorMsg.value = 'All fields are required for registration.';
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
  <div class="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4">
    <div class="bg-slate-800 border border-slate-700 rounded-2xl w-full max-w-md p-6 shadow-2xl">
      <div class="flex justify-between items-center pb-3 border-b border-slate-700 mb-4">
        <h2 class="text-xl font-bold text-white">{{ mode === 'login' ? 'Sign In to 5BX' : 'Create 5BX Pilot Profile' }}</h2>
        <button @click="emit('close')" class="text-slate-400 hover:text-white p-2">✕</button>
      </div>

      <!-- Mode Switch Tabs -->
      <div class="flex bg-slate-900/60 p-1 rounded-xl mb-4 border border-slate-700">
        <button
          @click="mode = 'login'; errorMsg = ''"
          class="flex-1 py-1.5 text-xs font-bold rounded-lg transition-colors"
          :class="mode === 'login' ? 'bg-blue-600 text-white shadow' : 'text-slate-400 hover:text-white'"
        >
          Sign In
        </button>
        <button
          @click="mode = 'register'; errorMsg = ''"
          class="flex-1 py-1.5 text-xs font-bold rounded-lg transition-colors"
          :class="mode === 'register' ? 'bg-blue-600 text-white shadow' : 'text-slate-400 hover:text-white'"
        >
          Register New Account
        </button>
      </div>

      <div v-if="errorMsg" class="mb-3 bg-red-500/20 border border-red-500/40 text-red-200 text-xs p-2.5 rounded-lg">
        {{ errorMsg }}
      </div>

      <!-- Login Form -->
      <form v-if="mode === 'login'" @submit.prevent="submitLogin" class="space-y-3">
        <div>
          <label class="text-xs font-semibold text-slate-300 block mb-1">Username or Email</label>
          <input
            v-model="username"
            type="text"
            required
            placeholder="e.g. Matthew or matthew@5bx.local"
            class="w-full bg-slate-900 border border-slate-700 rounded-lg p-2.5 text-white text-sm focus:border-blue-500 focus:outline-none"
          />
        </div>

        <div>
          <label class="text-xs font-semibold text-slate-300 block mb-1">Password</label>
          <input
            v-model="password"
            type="password"
            required
            placeholder="••••••••"
            class="w-full bg-slate-900 border border-slate-700 rounded-lg p-2.5 text-white text-sm focus:border-blue-500 focus:outline-none"
          />
        </div>

        <div class="bg-slate-900/40 p-2.5 rounded-lg border border-slate-700/50 text-[11px] text-slate-400">
          Tip: Migrated accounts (Matthew, Harvey, Maya, Test) have initial password <code class="text-blue-300 font-mono">FiveBX2026!</code>
        </div>

        <button
          type="submit"
          class="w-full py-3 bg-blue-600 hover:bg-blue-500 text-white font-bold rounded-xl shadow-lg mt-2 transition-colors"
        >
          Sign In
        </button>
      </form>

      <!-- Register Form -->
      <form v-else @submit.prevent="submitRegister" class="space-y-3">
        <div>
          <label class="text-xs font-semibold text-slate-300 block mb-1">Callsign / Username</label>
          <input
            v-model="username"
            type="text"
            required
            placeholder="Your name or callsign"
            class="w-full bg-slate-900 border border-slate-700 rounded-lg p-2 text-white text-sm"
          />
        </div>

        <div>
          <label class="text-xs font-semibold text-slate-300 block mb-1">Email Address</label>
          <input
            v-model="email"
            type="email"
            required
            placeholder="pilot@example.com"
            class="w-full bg-slate-900 border border-slate-700 rounded-lg p-2 text-white text-sm"
          />
        </div>

        <div>
          <label class="text-xs font-semibold text-slate-300 block mb-1">Date of Birth (For Age Target calculation)</label>
          <input
            v-model="dob"
            type="date"
            required
            class="w-full bg-slate-900 border border-slate-700 rounded-lg p-2 text-white text-sm"
          />
        </div>

        <div>
          <label class="text-xs font-semibold text-slate-300 block mb-1">Password</label>
          <input
            v-model="password"
            type="password"
            required
            placeholder="Minimum 8 characters"
            class="w-full bg-slate-900 border border-slate-700 rounded-lg p-2 text-white text-sm"
          />
        </div>

        <button
          type="submit"
          class="w-full py-3 bg-emerald-600 hover:bg-emerald-500 text-white font-bold rounded-xl shadow-lg mt-2 transition-colors"
        >
          Complete Registration
        </button>
      </form>
    </div>
  </div>
</template>
