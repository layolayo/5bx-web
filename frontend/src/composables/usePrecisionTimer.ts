import { ref, computed, onUnmounted } from 'vue';
import { playCountdownBeep, playTransitionChime } from '../audio';

export function usePrecisionTimer() {
  const secondsRemaining = ref(0);
  const initialDuration = ref(0);
  const isRunning = ref(false);
  const isPaused = ref(false);
  const isWakeLockActive = ref(false);

  let targetEndTime = 0;
  let timerInterval: any = null;
  let wakeLockSentinel: any = null;
  let completeCallback: (() => void) | null = null;
  let lastBeepSecond = -1;

  const isWakeLockSupported = typeof navigator !== 'undefined' && 'wakeLock' in navigator;

  const elapsedSeconds = computed(() => {
    return Math.max(0, initialDuration.value - secondsRemaining.value);
  });

  const formattedRemaining = computed(() => {
    const m = Math.floor(secondsRemaining.value / 60);
    const s = secondsRemaining.value % 60;
    return `${m}:${s.toString().padStart(2, '0')}`;
  });

  const formattedElapsed = computed(() => {
    const m = Math.floor(elapsedSeconds.value / 60);
    const s = elapsedSeconds.value % 60;
    return `${m}:${s.toString().padStart(2, '0')}`;
  });

  async function requestWakeLock() {
    if (!isWakeLockSupported) return;
    try {
      if (!wakeLockSentinel || wakeLockSentinel.released) {
        wakeLockSentinel = await (navigator as any).wakeLock.request('screen');
        isWakeLockActive.value = true;
        wakeLockSentinel.addEventListener('release', () => {
          isWakeLockActive.value = false;
        });
      }
    } catch (err) {
      // Screen Wake Lock may fail if low battery mode or tab hidden
      isWakeLockActive.value = false;
    }
  }

  async function releaseWakeLock() {
    if (wakeLockSentinel && !wakeLockSentinel.released) {
      try {
        await wakeLockSentinel.release();
      } catch (_) {}
    }
    wakeLockSentinel = null;
    isWakeLockActive.value = false;
  }

  function handleVisibilityChange() {
    if (typeof document === 'undefined') return;
    if (document.visibilityState === 'visible' && isRunning.value && !isPaused.value) {
      // Instant wall-clock resynchronisation upon waking display
      const now = Date.now();
      const remaining = Math.max(0, Math.ceil((targetEndTime - now) / 1000));
      secondsRemaining.value = remaining;

      // Re-acquire Screen Wake Lock because the operating system releases it on backgrounding
      requestWakeLock();

      if (remaining <= 0) {
        finishTimer();
      }
    }
  }

  if (typeof document !== 'undefined') {
    document.addEventListener('visibilitychange', handleVisibilityChange);
  }

  function startTimer(durationSeconds: number, onComplete?: () => void) {
    stopTimer();
    initialDuration.value = Math.max(0, Math.round(durationSeconds));
    secondsRemaining.value = initialDuration.value;
    isRunning.value = true;
    isPaused.value = false;
    lastBeepSecond = -1;
    completeCallback = onComplete || null;

    targetEndTime = Date.now() + secondsRemaining.value * 1000;
    requestWakeLock();

    // High-precision 250ms tick to prevent audio drift and accurately catch seconds transitions
    timerInterval = setInterval(() => {
      if (isPaused.value) return;

      const now = Date.now();
      const remaining = Math.max(0, Math.ceil((targetEndTime - now) / 1000));
      secondsRemaining.value = remaining;

      // Auditory cadence countdown beeps for final 3 seconds
      if (remaining <= 3 && remaining > 0 && remaining !== lastBeepSecond) {
        lastBeepSecond = remaining;
        playCountdownBeep(remaining === 1);
      }

      if (remaining <= 0) {
        finishTimer();
      }
    }, 250);
  }

  function pauseTimer() {
    if (!isRunning.value || isPaused.value) return;
    isPaused.value = true;
    // Calculate remaining seconds from hardware clock
    const now = Date.now();
    secondsRemaining.value = Math.max(0, Math.ceil((targetEndTime - now) / 1000));
    releaseWakeLock();
  }

  function resumeTimer() {
    if (!isRunning.value || !isPaused.value) return;
    isPaused.value = false;
    targetEndTime = Date.now() + secondsRemaining.value * 1000;
    requestWakeLock();
  }

  function togglePause() {
    if (isPaused.value) {
      resumeTimer();
    } else {
      pauseTimer();
    }
  }

  function finishTimer() {
    clearInterval(timerInterval);
    timerInterval = null;
    isRunning.value = false;
    isPaused.value = false;
    secondsRemaining.value = 0;
    releaseWakeLock();
    playTransitionChime();
    if (completeCallback) {
      completeCallback();
    }
  }

  function stopTimer() {
    if (timerInterval) {
      clearInterval(timerInterval);
      timerInterval = null;
    }
    isRunning.value = false;
    isPaused.value = false;
    releaseWakeLock();
  }

  onUnmounted(() => {
    stopTimer();
    if (typeof document !== 'undefined') {
      document.removeEventListener('visibilitychange', handleVisibilityChange);
    }
  });

  return {
    secondsRemaining,
    initialDuration,
    elapsedSeconds,
    isRunning,
    isPaused,
    isWakeLockActive,
    isWakeLockSupported,
    formattedRemaining,
    formattedElapsed,
    startTimer,
    pauseTimer,
    resumeTimer,
    togglePause,
    stopTimer,
    finishTimer,
  };
}
