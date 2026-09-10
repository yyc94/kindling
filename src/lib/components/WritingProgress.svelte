<script lang="ts">
  import { untrack } from "svelte";
  import { currentProject } from "../stores/project.svelte";
  import { writing } from "../stores/writing.svelte";
  import { proseSaves } from "../utils/proseSaves";
  import { ui } from "../stores/ui.svelte";
  import { t } from "../i18n.svelte";

  let { prepareReset }: { prepareReset?: () => Promise<void> } = $props();
  let resetting = $state(false);
  let resetError = $state<string | null>(null);
  const projectId = $derived(currentProject.value?.id ?? null);
  $effect(() => {
    const id = projectId;
    untrack(() => {
      writing.open(id);
      resetError = null;
    });
    // Refresh after midnight and structural edits, even if no prose was saved.
    const timer = setInterval(() => void writing.refresh(id), 30000);
    return () => {
      clearInterval(timer);
      writing.open(null);
    };
  });

  async function reset() {
    const projectId = currentProject.value?.id;
    if (!projectId) return;
    resetting = true;
    resetError = null;
    try {
      await prepareReset?.();
      await proseSaves.flush(projectId);
      await writing.reset(projectId);
    } catch (error) {
      if (currentProject.value?.id === projectId)
        resetError = t("Save your pending prose before resetting: {error}", {
          error: String(error),
        });
    } finally {
      resetting = false;
    }
  }
</script>

{#if writing.value && writing.value.project_id === currentProject.value?.id}
  {@const stats = writing.value}
  <div class="mt-3 space-y-1 text-press-eyebrow text-press-muted" data-testid="writing-progress">
    <p>{stats.project_words.toLocaleString(ui.locale)} {t("project words")}</p>
    <p title={t("Net words added through saved edits today. Deletions reduce this total.")}>
      {t("Today")}: {stats.today_words.toLocaleString(ui.locale)}{#if stats.daily_goal > 0}
        / {stats.daily_goal.toLocaleString(ui.locale)} {t("words")}{:else}
        {t("words · goal off")}{/if}
    </p>
    {#if stats.daily_goal > 0}
      <progress
        class="w-full"
        aria-label={t("Daily writing goal")}
        max={stats.daily_goal}
        value={Math.max(0, stats.today_words)}
      ></progress>
    {/if}
    <div class="flex items-center justify-between gap-2">
      <span title={t("Net words saved in this project since opening the app or resetting.")}
        >{t("Session")}: {stats.session_words.toLocaleString(ui.locale)} {t("words")}</span
      >
      <button
        class="text-press-muted hover:text-press-text"
        onclick={reset}
        disabled={resetting}
        aria-label={t("Reset writing session")}>{t("Reset")}</button
      >
    </div>
    <p>
      {t(stats.streak === 1 ? "{count} day writing streak" : "{count} days writing streak", {
        count: stats.streak,
      })}
    </p>
  </div>
{/if}
{#if resetError}
  <p role="alert" class="mt-2 text-press-eyebrow text-press-error">{resetError}</p>
{/if}

<style>
  progress {
    appearance: none;
    height: var(--space-3xs);
    background: var(--color-surface-sunken);
    border: none;
  }
  progress::-webkit-progress-bar {
    background: var(--color-surface-sunken);
  }
  progress::-webkit-progress-value {
    background: var(--color-accent);
  }
  progress::-moz-progress-bar {
    background: var(--color-accent);
  }
</style>
