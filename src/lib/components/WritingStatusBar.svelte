<script lang="ts">
  import { currentProject } from "../stores/project.svelte";
  import { writing } from "../stores/writing.svelte";
  import { ui } from "../stores/ui.svelte";
  import { t } from "../i18n.svelte";

  let expanded = $state(false);
  const stats = $derived(
    writing.value?.project_id === currentProject.value?.id ? writing.value : null
  );
  const sceneId = $derived(currentProject.currentScene?.id);
  const chapterId = $derived(
    currentProject.currentScene?.chapter_id ?? currentProject.currentChapter?.id
  );
  const sceneCounts = $derived(Object.values(stats?.scene_words ?? {}));
  const scenesWithProse = $derived(sceneCounts.filter((words) => words > 0).length);
  const averageWords = $derived(
    sceneCounts.length ? (stats?.project_words ?? 0) / sceneCounts.length : 0
  );
  const chapters = $derived(
    currentProject.chapters.filter(
      (chapter) =>
        !chapter.archived && !chapter.is_part && stats?.chapter_words[chapter.id] !== undefined
    )
  );

  function toggleStatistics() {
    expanded = !expanded;
    if (expanded) void writing.refresh();
  }
</script>

{#if currentProject.value && (stats || writing.error)}
  <footer class="shrink-0 border-t border-press-border bg-press-bg font-press-ui">
    {#if writing.error}
      <p role="alert" class="px-4 py-2 text-press-small text-press-error">{writing.error}</p>
    {/if}
    {#if stats}
      {#if expanded}
        <!-- The scrollable panel needs a tab stop for keyboard scrolling. -->
        <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
        <section
          id="writing-statistics-panel"
          aria-labelledby="writing-statistics-title"
          class="statistics-panel overflow-y-auto border-b border-press-border px-4 py-3"
          tabindex="0"
        >
          <h2 id="writing-statistics-title" class="text-press-h3 text-press-text">
            {t("Writing statistics")}
          </h2>
          <dl class="my-3 flex flex-wrap gap-x-8 gap-y-3 text-press-small">
            <div>
              <dt class="text-press-muted">{t("Total words")}</dt>
              <dd class="text-press-text tabular-nums">
                {stats.project_words.toLocaleString(ui.locale)}
              </dd>
            </div>
            <div>
              <dt class="text-press-muted">{t("Scenes with prose")}</dt>
              <dd class="text-press-text tabular-nums">
                {scenesWithProse.toLocaleString(ui.locale)}
              </dd>
            </div>
            <div>
              <dt class="text-press-muted">{t("Empty scenes")}</dt>
              <dd class="text-press-text tabular-nums">
                {(sceneCounts.length - scenesWithProse).toLocaleString(ui.locale)}
              </dd>
            </div>
            <div>
              <dt class="text-press-muted">{t("Average words per scene")}</dt>
              <dd class="text-press-text tabular-nums">
                {averageWords.toLocaleString(ui.locale, { maximumFractionDigits: 1 })}
              </dd>
            </div>
          </dl>
          <p class="mb-3 text-press-eyebrow text-press-muted">
            {t(
              "Saved prose only. Archived content is excluded; the average includes empty scenes."
            )}
          </p>
          {#if chapters.length}
            <table class="w-full table-fixed text-press-small text-press-text">
              <caption class="text-left font-semibold mb-2">{t("Words per chapter")}</caption>
              <thead>
                <tr class="border-b border-press-border">
                  <th scope="col" class="text-left py-1 font-medium">{t("Chapter")}</th>
                  <th scope="col" class="w-24 text-right py-1 font-medium">{t("Words")}</th>
                </tr>
              </thead>
              <tbody>
                {#each chapters as chapter (chapter.id)}
                  <tr class="border-b border-press-border">
                    <th scope="row" class="text-left py-1 pr-4 font-normal break-words">
                      {chapter.title}
                    </th>
                    <td class="text-right py-1 tabular-nums">
                      {stats.chapter_words[chapter.id].toLocaleString(ui.locale)}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          {:else}
            <p class="text-press-small text-press-muted">{t("No chapters yet.")}</p>
          {/if}
        </section>
      {/if}
      <div
        class="flex flex-wrap items-center gap-x-4 gap-y-1 px-4 py-2 text-press-eyebrow text-press-muted"
      >
        <span
          >{t("Scene")}: {sceneId
            ? (stats.scene_words[sceneId]?.toLocaleString(ui.locale) ?? "—")
            : "—"}
          {t("words")}</span
        >
        <span
          >{t("Chapter")}: {chapterId
            ? (stats.chapter_words[chapterId]?.toLocaleString(ui.locale) ?? "—")
            : "—"}
          {t("words")}</span
        >
        <span>{t("Project")}: {stats.project_words.toLocaleString(ui.locale)} {t("words")}</span>
        <span title={t("Net words saved since opening the app or resetting the session.")}>
          {t("Session")}: {stats.session_words.toLocaleString(ui.locale)}
          {t("words")}
        </span>
        <button
          class="ml-auto text-press-muted hover:text-press-text"
          aria-expanded={expanded}
          aria-controls="writing-statistics-panel"
          onclick={toggleStatistics}
          >{t(expanded ? "Hide statistics" : "Writing statistics")}</button
        >
      </div>
    {/if}
  </footer>
{/if}

<style>
  .statistics-panel {
    max-height: 35vh;
  }
</style>
