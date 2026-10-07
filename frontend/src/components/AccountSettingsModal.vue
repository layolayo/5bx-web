<script setup lang="ts">
import { ref } from 'vue';
import { UserProfile } from '../types';
import { changePassword, deleteAccount } from '../api';

const props = defineProps<{
  profile: UserProfile | null;
}>();

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'account-deleted'): void;
}>();

const activeTab = ref<'password' | 'danger'>('password');

// Change Password State
const currentPassword = ref('');
const newPassword = ref('');
const confirmPassword = ref('');
const isSubmittingPassword = ref(false);
const passwordSuccess = ref(false);
const passwordError = ref('');

// Delete Account State
const deleteConfirmPassword = ref('');
const confirmDeleteAcknowledged = ref(false);
const isDeletingAccount = ref(false);
const deleteError = ref('');

async function handleChangePassword() {
  passwordError.value = '';
  passwordSuccess.value = false;

  if (!currentPassword.value) {
    passwordError.value = 'Please enter your current password.';
    return;
  }

  if (newPassword.value.length < 8) {
    passwordError.value = 'New password must be at least 8 characters in length.';
    return;
  }

  if (newPassword.value !== confirmPassword.value) {
    passwordError.value = 'New password and confirmation do not match.';
    return;
  }

  isSubmittingPassword.value = true;
  try {
    await changePassword(currentPassword.value, newPassword.value);
    passwordSuccess.value = true;
    currentPassword.value = '';
    newPassword.value = '';
    confirmPassword.value = '';
    setTimeout(() => {
      passwordSuccess.value = false;
    }, 4000);
  } catch (err: any) {
    passwordError.value = err.message || 'Failed to update password.';
  } finally {
    isSubmittingPassword.value = false;
  }
}

async function handleDeleteAccount() {
  deleteError.value = '';

  if (!deleteConfirmPassword.value) {
    deleteError.value = 'Please enter your password to authorise account deletion.';
    return;
  }

  if (!confirmDeleteAcknowledged.value) {
    deleteError.value = 'Please confirm that you understand this action is permanent and unrecoverable.';
    return;
  }

  isDeletingAccount.value = true;
  try {
    await deleteAccount(deleteConfirmPassword.value);
    emit('account-deleted');
    emit('close');
  } catch (err: any) {
    deleteError.value = err.message || 'Failed to delete account.';
    isDeletingAccount.value = false;
  }
}
</script>

<template>
  <div class="fixed inset-0 z-50 bg-slate-950/85 backdrop-blur-md flex items-center justify-center p-4">
    <div class="bg-slate-900 border border-slate-700/80 rounded-3xl w-full max-w-lg p-6 sm:p-8 shadow-2xl relative overflow-hidden">
      <!-- Glow ambient accent -->
      <div class="absolute -top-24 right-0 w-72 h-72 bg-cyan-500/10 blur-3xl pointer-events-none"></div>

      <!-- Header -->
      <div class="flex items-start justify-between pb-4 border-b border-slate-800 mb-5">
        <div>
          <div class="inline-flex items-center gap-2 px-2.5 py-0.5 rounded-full bg-cyan-950/80 border border-cyan-500/30 text-cyan-300 text-[10px] font-bold uppercase tracking-wider mb-1">
            <span>⚙️</span>
            <span>Security &amp; Account Management</span>
          </div>
          <h2 class="text-xl font-black text-white uppercase tracking-tight">Pilot Account Settings</h2>
          <p class="text-xs text-slate-400 font-mono">{{ profile?.username }} • {{ profile?.email }}</p>
        </div>
        <button
          @click="emit('close')"
          class="w-8 h-8 rounded-full bg-slate-800 text-slate-400 hover:text-white flex items-center justify-center transition-colors cursor-pointer"
        >
          ✕
        </button>
      </div>

      <!-- Navigation Tabs -->
      <div class="flex gap-2 p-1 bg-slate-950 rounded-xl border border-slate-800 mb-5">
        <button
          type="button"
          @click="activeTab = 'password'"
          class="flex-1 py-2 text-xs font-bold rounded-lg transition-all cursor-pointer"
          :class="activeTab === 'password' ? 'bg-cyan-500 text-slate-950 shadow-sm' : 'text-slate-400 hover:text-white'"
        >
          Change Password
        </button>
        <button
          type="button"
          @click="activeTab = 'danger'"
          class="flex-1 py-2 text-xs font-bold rounded-lg transition-all cursor-pointer"
          :class="activeTab === 'danger' ? 'bg-red-500/20 text-red-300 border border-red-500/40' : 'text-slate-400 hover:text-red-300'"
        >
          Danger Zone
        </button>
      </div>

      <!-- Tab 1: Change Password -->
      <div v-if="activeTab === 'password'" class="space-y-4">
        <!-- Error / Success Notices -->
        <div v-if="passwordError" class="p-3 bg-red-500/20 border border-red-500/40 text-red-300 rounded-xl text-xs">
          {{ passwordError }}
        </div>
        <div v-if="passwordSuccess" class="p-3 bg-emerald-500/20 border border-emerald-500/40 text-emerald-300 rounded-xl text-xs font-semibold">
          ✓ Password successfully updated. Your new credentials are active.
        </div>

        <form @submit.prevent="handleChangePassword" class="space-y-3.5">
          <div>
            <label class="block text-[11px] font-bold text-slate-400 uppercase tracking-wider mb-1">
              Current Password
            </label>
            <input
              v-model="currentPassword"
              type="password"
              placeholder="••••••••"
              required
              class="w-full bg-slate-950 border border-slate-800 focus:border-cyan-500 rounded-xl px-3.5 py-2.5 text-xs text-white placeholder-slate-600 focus:outline-none transition-colors"
            />
          </div>

          <div>
            <label class="block text-[11px] font-bold text-slate-400 uppercase tracking-wider mb-1">
              New Password <span class="text-[10px] text-slate-500 font-normal">(Minimum 8 characters)</span>
            </label>
            <input
              v-model="newPassword"
              type="password"
              placeholder="••••••••"
              required
              minlength="8"
              class="w-full bg-slate-950 border border-slate-800 focus:border-cyan-500 rounded-xl px-3.5 py-2.5 text-xs text-white placeholder-slate-600 focus:outline-none transition-colors"
            />
          </div>

          <div>
            <label class="block text-[11px] font-bold text-slate-400 uppercase tracking-wider mb-1">
              Confirm New Password
            </label>
            <input
              v-model="confirmPassword"
              type="password"
              placeholder="••••••••"
              required
              minlength="8"
              class="w-full bg-slate-950 border border-slate-800 focus:border-cyan-500 rounded-xl px-3.5 py-2.5 text-xs text-white placeholder-slate-600 focus:outline-none transition-colors"
            />
          </div>

          <button
            type="submit"
            :disabled="isSubmittingPassword"
            class="w-full py-3 rounded-xl bg-gradient-to-r from-cyan-500 to-blue-600 hover:from-cyan-400 hover:to-blue-500 disabled:opacity-50 text-slate-950 font-black text-xs uppercase tracking-wider shadow-lg shadow-cyan-500/20 transition-all cursor-pointer mt-2"
          >
            {{ isSubmittingPassword ? 'Updating Password...' : 'Save New Password' }}
          </button>
        </form>
      </div>

      <!-- Tab 2: Danger Zone (Delete Account) -->
      <div v-else class="space-y-4">
        <div class="p-4 bg-red-950/30 border border-red-500/40 rounded-2xl text-xs text-slate-300 space-y-2">
          <div class="flex items-center gap-2 text-red-400 font-bold">
            <span>⚠️</span>
            <span class="uppercase tracking-wider">Permanent Account Removal</span>
          </div>
          <p class="leading-relaxed text-slate-400">
            Deleting your account will permanently purge your pilot profile, all recorded flight missions, performance telemetry, and earned milestone honours.
          </p>
          <p class="font-bold text-red-300">
            This action is immediate and cannot be reversed.
          </p>
        </div>

        <div v-if="deleteError" class="p-3 bg-red-500/20 border border-red-500/40 text-red-300 rounded-xl text-xs">
          {{ deleteError }}
        </div>

        <form @submit.prevent="handleDeleteAccount" class="space-y-3.5 pt-1">
          <div>
            <label class="block text-[11px] font-bold text-slate-400 uppercase tracking-wider mb-1">
              Enter Password to Authorise Deletion
            </label>
            <input
              v-model="deleteConfirmPassword"
              type="password"
              placeholder="Your current password"
              required
              class="w-full bg-slate-950 border border-slate-800 focus:border-red-500 rounded-xl px-3.5 py-2.5 text-xs text-white placeholder-slate-600 focus:outline-none transition-colors"
            />
          </div>

          <label class="flex items-start gap-2.5 cursor-pointer select-none bg-slate-950/60 p-3 rounded-xl border border-slate-800">
            <input
              v-model="confirmDeleteAcknowledged"
              type="checkbox"
              required
              class="mt-0.5 rounded border-slate-700 text-red-500 focus:ring-0 cursor-pointer"
            />
            <span class="text-xs text-slate-300 leading-tight">
              I acknowledge that deleting my account permanently erases all my flight telemetry and cannot be undone.
            </span>
          </label>

          <button
            type="submit"
            :disabled="isDeletingAccount || !confirmDeleteAcknowledged"
            class="w-full py-3 rounded-xl bg-red-950 hover:bg-red-900 border border-red-500/50 hover:border-red-400 text-red-300 hover:text-white disabled:opacity-40 font-bold text-xs uppercase tracking-wider transition-all cursor-pointer"
          >
            {{ isDeletingAccount ? 'Permanently Removing Account...' : 'Permanently Delete My Account' }}
          </button>
        </form>
      </div>
    </div>
  </div>
</template>
