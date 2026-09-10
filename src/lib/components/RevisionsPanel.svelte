<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { ChevronDown, History, Plus, RotateCcw } from "lucide-svelte";
  import { proseText } from "../utils/proseSearch";
  import {
    activeDocuments,
    diffText,
    draftOf,
    revisionStatuses,
    type SceneReview,
    type ReviewData,
    type ReviewDraft,
    type RevisionOverview,
  } from "../utils/revisions";
  import { ui } from "../stores/ui.svelte";
  import { t } from "../i18n.svelte";
  let {
    sceneId,
    projectId,
    title,
    locked,
    onApplied,
    onClose,
  }: {
    sceneId: string;
    projectId: string;
    title: string;
    locked: boolean;
    onApplied: (review: SceneReview) => void;
    onClose: () => void;
  } = $props();
  let dialog: HTMLDialogElement;
  let review = $state.raw<SceneReview>();
  let overview = $state<RevisionOverview[]>([]);
  let busy = $state(false);
  let error = $state("");
  let tab = $state<"history" | "overview">("history");
  let name = $state("");
  let before = $state(0);
  let after = $state(-1);
  let restoreIndex = $state<number | null>(null);
  const oldDraft = $derived(review?.data.drafts[before]);
  const newDraft = $derived(after < 0 ? review : review?.data.drafts[after]);
  const comparison = $derived(
    oldDraft && newDraft
      ? diffText(
          activeDocuments({ ...oldDraft, scene_id: sceneId })
            .map((d) => proseText(d.html).trimEnd())
            .join("\n"),
          activeDocuments({ ...newDraft, scene_id: sceneId })
            .map((d) => proseText(d.html).trimEnd())
            .join("\n")
        )
      : []
  );
  const hasChanges = $derived(comparison.some((part) => part.kind !== "same"));
  function dateLabel(value: string) {
    return new Date(value).toLocaleString(ui.locale, {
      month: "short",
      day: "numeric",
      year: "numeric",
      hour: "numeric",
      minute: "2-digit",
    });
  }

  onMount(() => {
    dialog.showModal();
    void load();
  });
  async function load() {
    busy = true;
    error = "";
    try {
      review = await invoke<SceneReview>("get_scene_review", { sceneId });
      before = Math.max(0, review.data.drafts.length - 1);
      overview = await invoke<RevisionOverview[]>("get_revision_overview", { projectId });
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
  async function save(data: ReviewData, next: ReviewDraft | null = null) {
    if (!review || busy || locked) return false;
    busy = true;
    error = "";
    try {
      review = await invoke<SceneReview>("save_scene_review", { expected: review, data, next });
      if (next) {
        onApplied(review);
      }
      // Update the overview locally after the committed write, so a failed
      // auxiliary read cannot make a successful decision appear to have failed.
      overview = overview.map((row) =>
        row.scene_id === sceneId ? { ...row, status: data.status, drafts: data.drafts.length } : row
      );
      return true;
    } catch (e) {
      error = String(e);
      return false;
    } finally {
      busy = false;
    }
  }
  async function createDraft() {
    if (!review || !name.trim()) return;
    const data = window.structuredClone(review.data);
    data.drafts.push(draftOf(review, name));
    if (await save(data)) {
      before = data.drafts.length - 1;
      name = "";
    }
  }
  async function restore() {
    if (!review || restoreIndex === null) return;
    const next = review.data.drafts[restoreIndex];
    const data = window.structuredClone(review.data);
    data.drafts.push(draftOf(review, t("Before restoring {name}", { name: next.name })));
    if (await save(data, next)) restoreIndex = null;
  }
  async function setStatus(status: string) {
    if (review) await save({ ...review.data, status });
  }
</script>

<dialog
  bind:this={dialog}
  aria-labelledby="revisions-title"
  oncancel={(e) => {
    e.preventDefault();
    if (!busy) onClose();
  }}
  onkeydown={(e) => e.stopPropagation()}
>
  <header class="history-header">
    <div>
      <span class="eyebrow">{title}</span>
      <h2 id="revisions-title">{t("Draft history")}</h2>
    </div>
    <div class="header-actions">
      {#if review}<fieldset disabled={busy || locked}>
          <label class="status-label"
            >{t("Revision status")}<span class="compact-select"
              ><select value={review.data.status} onchange={(e) => setStatus(e.currentTarget.value)}
                >{#each Object.entries(revisionStatuses) as [value, label]}<option {value}
                    >{t(label)}</option
                  >{/each}</select
              ><ChevronDown size={14} /></span
            ></label
          >
        </fieldset>{/if}
      <button type="button" disabled={busy} onclick={onClose}>{t("Close")}</button>
    </div>
  </header>
  <nav aria-label={t("Revision views")} class="history-tabs">
    <button aria-pressed={tab === "history"} onclick={() => (tab = "history")}
      >{t("Draft history")}</button
    >
    <button aria-pressed={tab === "overview"} onclick={() => (tab = "overview")}
      >{t("All scenes")}</button
    >
  </nav>
  {#if error}<p role="alert" class="history-error">{error}</p>{/if}
  {#if !review}
    <div class="history-empty">
      <p>{t(busy ? "Loading revisions…" : "Could not load revisions.")}</p>
      {#if !busy}<button onclick={load}>{t("Retry")}</button>{/if}
    </div>
  {:else}
    {#if locked}<p class="locked-notice">{t("This scene is locked. History is read-only.")}</p>{/if}
    {#if tab === "history"}
      <div class="history-layout">
        <aside class="draft-list" aria-label={t("Saved drafts")}>
          <h3>{t("Saved drafts")} <span>{review.data.drafts.length}</span></h3>
          <fieldset disabled={busy || locked} class="save-draft">
            <label
              >{t("Draft name")}<input
                bind:value={name}
                placeholder={t("Post-editor pass")}
              /></label
            >
            <button disabled={!name.trim()} onclick={createDraft}
              ><Plus size={14} />{t("Save named draft")}</button
            >
          </fieldset>
          <div class="draft-entries">
            {#each review.data.drafts.map((draft, index) => ({ draft, index })).reverse() as item}
              <button
                class="draft-entry"
                aria-pressed={before === item.index}
                onclick={() => {
                  before = item.index;
                  restoreIndex = null;
                }}
                ><span class="draft-number">{t("Draft {number}", { number: item.index + 1 })}</span
                ><strong>{item.draft.name}</strong><time datetime={item.draft.created_at}
                  >{dateLabel(item.draft.created_at)}</time
                ></button
              >
            {/each}
          </div>
        </aside>
        <section class="draft-detail" aria-label={t("Draft comparison")}>
          {#if !review.data.drafts.length}
            <div class="history-empty">
              <History size={28} />
              <h3>{t("Keep a version of this scene")}</h3>
              <p>
                {t("No saved drafts yet. Save a named draft to keep this scene’s current prose.")}
              </p>
            </div>
          {:else if oldDraft && newDraft}
            <div class="comparison-toolbar">
              <label
                >{t("Compare with")}<span class="compact-select"
                  ><select aria-label={t("Compare with")} bind:value={after}
                    ><option value={-1}>{t("Current prose")}</option
                    >{#each review.data.drafts as d, i}<option value={i}
                        >{t("Draft {number}", { number: i + 1 })} · {d.name}</option
                      >{/each}</select
                  ><ChevronDown size={14} /></span
                ></label
              >
              <button disabled={busy || locked} onclick={() => (restoreIndex = before)}
                ><RotateCcw size={14} />{t("Restore selected draft")}</button
              >
            </div>
            {#if restoreIndex !== null}
              <div class="restore-confirm" role="region" aria-label={t("Confirm draft restore")}>
                <p>
                  {t(
                    "Restore “{name}”? Current prose will be preserved as another draft. Beat structure must still match.",
                    { name: review.data.drafts[restoreIndex].name }
                  )}
                </p>
                <div class="restore-actions">
                  <button disabled={busy || locked} onclick={restore}
                    >{t("Restore and preserve current prose")}</button
                  ><button disabled={busy} onclick={() => (restoreIndex = null)}
                    >{t("Cancel restore")}</button
                  >
                </div>
              </div>
            {/if}
            <p class="comparison-note">
              {t(
                hasChanges
                  ? "Removed text is marked on the left; added text is marked on the right. Formatting is not compared."
                  : "No prose text changes between these versions. Formatting is not compared."
              )}
            </p>
            <div class="comparison-pages">
              <section class="comparison-version" aria-label={t("Saved draft")}>
                <header>
                  <span class="version-kind"
                    >{t("Saved draft")} · {t(
                      oldDraft.mode === "page" ? "Page prose" : "Beat prose"
                    )}</span
                  >
                  <h3>{oldDraft.name}</h3>
                  <time datetime={oldDraft.created_at}>{dateLabel(oldDraft.created_at)}</time>
                </header>
                <div class="diff-prose">
                  {#each comparison as part}{#if part.kind === "delete"}<del>{part.text}</del
                      >{:else if part.kind !== "insert"}{part.text}{/if}{/each}
                </div>
              </section>
              <section class="comparison-version" aria-label={t("Comparison version")}>
                <header>
                  <span class="version-kind"
                    >{t(after < 0 ? "Working manuscript" : "Saved draft")} · {t(
                      newDraft.mode === "page" ? "Page prose" : "Beat prose"
                    )}</span
                  >
                  <h3>{after < 0 ? t("Current prose") : review.data.drafts[after].name}</h3>
                  {#if after >= 0}<time datetime={review.data.drafts[after].created_at}
                      >{dateLabel(review.data.drafts[after].created_at)}</time
                    >{/if}
                </header>
                <div class="diff-prose">
                  {#each comparison as part}{#if part.kind === "insert"}<ins>{part.text}</ins
                      >{:else if part.kind !== "delete"}{part.text}{/if}{/each}
                </div>
              </section>
            </div>
          {/if}
        </section>
      </div>
    {:else}
      <div class="overview-scroll">
        <table>
          <thead
            ><tr
              ><th>{t("Chapter")}</th><th>{t("Scene")}</th><th>{t("Revision status")}</th><th
                >{t("Saved drafts")}</th
              ></tr
            ></thead
          ><tbody
            >{#each overview as row}<tr
                ><td>{row.chapter}</td><td>{row.title}</td><td
                  >{t(revisionStatuses[row.status as keyof typeof revisionStatuses])}</td
                ><td>{row.drafts}</td></tr
              >{/each}</tbody
          >
        </table>
      </div>
    {/if}
  {/if}
  {#if busy}<p role="status" class="busy-notice">{t("Saving or loading…")}</p>{/if}
</dialog>

<style>
  dialog {
    width: min(80rem, calc(100vw - var(--space-xl)));
    height: min(56rem, calc(100vh - var(--space-xl)));
    max-height: calc(100vh - var(--space-xl));
    margin: auto;
    padding: 0;
    background: var(--color-surface);
    color: var(--color-text);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-m);
    box-shadow: var(--shadow-overlay);
    font-family: var(--font-ui);
    font-size: var(--text-small);
    overflow: hidden;
  }
  dialog[open] {
    display: flex;
    flex-direction: column;
  }
  dialog::backdrop {
    background: var(--color-overlay-scrim);
  }
  .history-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--space-s);
    padding: var(--space-m);
    border-bottom: 1px solid var(--color-border);
  }
  h2 {
    font-family: var(--font-display);
    font-size: var(--text-h3);
    margin: 0;
  }
  .eyebrow,
  .version-kind,
  time {
    font-size: var(--text-eyebrow);
    color: var(--color-text-muted);
  }
  .header-actions {
    display: flex;
    align-items: end;
    gap: var(--space-s);
  }
  fieldset {
    border: 0;
    padding: 0;
    margin: 0;
    min-width: 0;
  }
  label {
    display: flex;
    flex-direction: column;
    gap: var(--space-2xs);
    min-width: 0;
  }
  .status-label {
    font-size: var(--text-eyebrow);
    color: var(--color-text-muted);
  }
  input,
  select {
    font-family: var(--font-ui);
    font-size: var(--text-small);
    line-height: var(--leading);
    padding: var(--space-3xs) var(--space-2xs);
    min-width: 0;
    max-width: 100%;
  }
  .compact-select {
    display: grid;
    align-items: center;
    min-width: 0;
  }
  .compact-select select {
    grid-area: 1 / 1;
    appearance: none;
    padding-right: var(--space-l);
    width: 100%;
  }
  .compact-select :global(svg) {
    grid-area: 1 / 1;
    justify-self: end;
    margin-right: var(--space-2xs);
    pointer-events: none;
    color: var(--color-text-muted);
  }
  button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-2xs);
    padding: var(--space-2xs) var(--space-xs);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-s);
    cursor: pointer;
  }
  button:hover {
    background: var(--color-surface-sunken);
  }
  .history-tabs {
    display: flex;
    gap: var(--space-m);
    padding-inline: var(--space-m);
    border-bottom: 1px solid var(--color-border);
  }
  .history-tabs button {
    border: 0;
    border-bottom: 2px solid transparent;
    border-radius: 0;
    padding: var(--space-xs) 0;
  }
  .history-tabs button[aria-pressed="true"] {
    color: var(--color-accent-text);
    border-bottom-color: var(--color-accent);
  }
  .history-layout {
    display: grid;
    grid-template-columns: 16rem minmax(0, 1fr);
    min-height: 0;
    flex: 1;
  }
  .draft-list {
    display: flex;
    flex-direction: column;
    min-height: 0;
    border-right: 1px solid var(--color-border);
    background: var(--color-bg);
  }
  .draft-list h3 {
    font-size: var(--text-small);
    margin: var(--space-s);
  }
  .draft-list h3 span {
    color: var(--color-text-muted);
    margin-left: var(--space-2xs);
    font-weight: 400;
  }
  .save-draft {
    display: flex;
    flex-direction: column;
    gap: var(--space-2xs);
    margin: 0 var(--space-s) var(--space-s);
  }
  .draft-entries {
    overflow: auto;
    min-height: 0;
  }
  .draft-entry {
    display: flex;
    flex-direction: column;
    align-items: start;
    text-align: left;
    width: 100%;
    border: 0;
    border-top: 1px solid var(--color-border);
    border-left: 2px solid transparent;
    border-radius: 0;
    padding: var(--space-s);
    overflow-wrap: anywhere;
  }
  .draft-entry[aria-pressed="true"] {
    background: var(--color-accent-wash);
    border-left-color: var(--color-accent);
  }
  .draft-number {
    font-size: var(--text-eyebrow);
    color: var(--color-text-muted);
  }
  .draft-entry strong {
    font-weight: 500;
  }
  .draft-detail {
    display: flex;
    flex-direction: column;
    min-height: 0;
    min-width: 0;
  }
  .comparison-toolbar {
    display: flex;
    align-items: end;
    justify-content: space-between;
    gap: var(--space-s);
    padding: var(--space-s) var(--space-m);
  }
  .comparison-toolbar label {
    flex: 1;
    max-width: var(--measure);
  }
  .comparison-toolbar button {
    flex-shrink: 0;
  }
  .comparison-note {
    font-size: var(--text-eyebrow);
    color: var(--color-text-muted);
    margin: 0;
    padding: 0 var(--space-m) var(--space-s);
    border-bottom: 1px solid var(--color-border);
  }
  .comparison-pages {
    display: grid;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
    min-height: 0;
    overflow: auto;
    flex: 1;
  }
  .comparison-version {
    padding: var(--space-m);
    min-width: 0;
  }
  .comparison-version + .comparison-version {
    border-left: 1px solid var(--color-border);
  }
  .comparison-version header {
    border-bottom: 1px solid var(--color-border);
    padding-bottom: var(--space-s);
    margin-bottom: var(--space-m);
  }
  .comparison-version h3 {
    font-family: var(--font-display);
    font-size: var(--text-body-lg);
    margin: var(--space-2xs) 0;
    overflow-wrap: anywhere;
  }
  .diff-prose {
    font-family: var(--font-body);
    font-size: var(--text-body);
    line-height: var(--leading-relaxed);
    max-width: var(--measure);
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
  del {
    color: var(--color-error);
    background: var(--color-error-wash);
  }
  ins {
    color: var(--color-success);
    background: var(--color-success-wash);
  }
  .restore-confirm {
    padding: var(--space-s) var(--space-m);
    background: var(--color-surface-sunken);
    border-block: 1px solid var(--color-border);
  }
  .restore-confirm p {
    margin: 0 0 var(--space-xs);
  }
  .restore-actions {
    display: flex;
    gap: var(--space-2xs);
    flex-wrap: wrap;
  }
  .history-empty {
    margin: auto;
    padding: var(--space-l);
    max-width: var(--measure);
    color: var(--color-text-muted);
  }
  .history-empty h3 {
    font-family: var(--font-display);
    font-size: var(--text-h3);
    color: var(--color-text);
  }
  .history-error {
    color: var(--color-error);
    padding-inline: var(--space-m);
  }
  .locked-notice,
  .busy-notice {
    color: var(--color-text-muted);
    margin: 0;
    padding: var(--space-xs) var(--space-m);
  }
  .overview-scroll {
    overflow: auto;
    min-height: 0;
  }
  table {
    width: 100%;
    text-align: left;
    border-collapse: collapse;
  }
  td,
  th {
    padding: var(--space-s) var(--space-m);
    border-bottom: 1px solid var(--color-border);
  }
</style>
