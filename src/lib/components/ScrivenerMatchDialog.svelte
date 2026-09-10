<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { Loader2, Link2, AlertCircle, X } from "lucide-svelte";
  import type { ScrivenerMatchPreview } from "../types";
  import { t } from "../i18n.svelte";

  let {
    projectId,
    scrivPath,
    onConfirm,
    onCancel,
  }: {
    projectId: string;
    scrivPath: string;
    onConfirm: () => void;
    onCancel: () => void;
  } = $props();

  let matches = $state<ScrivenerMatchPreview[]>([]);
  let loading = $state(true);
  let loadError = $state<string | null>(null);

  const matchedCount = $derived(matches.filter((m) => m.matched_scriv_title).length);
  const unmatchedCount = $derived(matches.filter((m) => !m.matched_scriv_title).length);

  onMount(async () => {
    try {
      matches = await invoke<ScrivenerMatchPreview[]>("preview_scrivener_matches", {
        projectId,
        scrivPath,
      });
    } catch (e) {
      loadError = e instanceof Error ? e.message : String(e);
    } finally {
      loading = false;
    }
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") onCancel();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  class="fixed inset-0 z-press-modal flex items-center justify-center bg-press-overlay backdrop-blur-sm"
  onclick={onCancel}
  onkeydown={(e) => e.key === "Escape" && onCancel()}
  role="presentation"
  tabindex="-1"
>
  <div
    class="app-dialog-surface bg-press-surface border border-press-border rounded-2xl shadow-press-overlay w-xl max-h-[80vh] flex flex-col"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.stopPropagation()}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
  >
    <div class="flex items-center justify-between px-6 py-4 border-b border-press-border">
      <div>
        <h2 class="text-press-body-lg font-semibold text-press-text">
          {t("Scrivener Match Preview")}
        </h2>
        <p class="text-press-eyebrow text-press-muted mt-0.5">
          {t("Review how scenes will map to Scrivener documents")}
        </p>
      </div>
      <button
        onclick={onCancel}
        class="p-1 text-press-muted hover:text-press-text"
        aria-label={t("Close")}
      >
        <X class="w-5 h-5" />
      </button>
    </div>

    <div class="flex-1 overflow-y-auto px-6 py-4">
      {#if loading}
        <div class="flex items-center justify-center py-12">
          <Loader2 class="w-6 h-6 animate-spin text-press-accent-text" />
          <span class="ml-2 text-press-muted">{t("Analyzing matches...")}</span>
        </div>
      {:else if loadError}
        <div class="flex items-center gap-2 text-press-error py-4">
          <AlertCircle class="w-5 h-5 shrink-0" />
          <p class="text-press-ui">{loadError}</p>
        </div>
      {:else}
        <div class="flex items-center gap-4 mb-4">
          <span class="text-press-eyebrow text-press-muted">
            {t("{count} matched", { count: matchedCount })}
          </span>
          {#if unmatchedCount > 0}
            <span class="text-press-eyebrow text-press-muted">
              {t("{count} unmatched (will be created)", { count: unmatchedCount })}
            </span>
          {/if}
        </div>

        <div class="space-y-1">
          {#each matches as m}
            {@const isMatched = !!m.matched_scriv_title}
            {@const MatchIcon = isMatched ? Link2 : AlertCircle}
            <div
              class="flex items-center gap-3 px-3 py-2 rounded-lg text-press-ui {isMatched
                ? 'bg-press-success-wash'
                : 'bg-press-warning-wash'}"
            >
              <MatchIcon
                class="w-3.5 h-3.5 {isMatched
                  ? 'text-press-success'
                  : 'text-press-warning'} shrink-0"
              />
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-1.5">
                  <span class="text-press-muted text-press-eyebrow truncate">{m.chapter_title}</span
                  >
                  <span class="text-press-muted">/</span>
                  <span class="text-press-text truncate">{m.scene_title}</span>
                </div>
                <p
                  class="text-press-eyebrow mt-0.5 {isMatched
                    ? 'text-press-muted'
                    : 'text-press-warning'}"
                >
                  {isMatched
                    ? t("→ {title} (via {method})", {
                        title: m.matched_scriv_title ?? "",
                        method: t(m.match_method === "source_id" ? "ID" : "title"),
                      })
                    : t("New document")}
                </p>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <div class="flex items-center justify-end gap-3 px-6 py-4 border-t border-press-border">
      <button
        onclick={onCancel}
        class="px-4 py-2 text-press-ui text-press-muted hover:text-press-text transition-colors"
      >
        {t("Cancel")}
      </button>
      <button
        onclick={onConfirm}
        disabled={loading || !!loadError}
        class="px-4 py-2 bg-press-accent text-press-on-accent text-press-ui font-medium rounded-lg hover:bg-press-accent-text transition-colors"
      >
        {t("Proceed with Export")}
      </button>
    </div>
  </div>
</div>
