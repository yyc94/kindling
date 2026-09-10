<script lang="ts">
  import { t } from "../i18n.svelte";

  interface Props {
    partTitle: string;
    childChapterCount: number;
    partLabel?: string;
    chapterLabel?: string;
    onDeletePartOnly: () => void;
    onDeletePartAndChapters: () => void;
    onCancel: () => void;
  }

  let {
    partTitle,
    childChapterCount,
    partLabel = "Part",
    chapterLabel = "Chapter",
    onDeletePartOnly,
    onDeletePartAndChapters,
    onCancel,
  }: Props = $props();

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onCancel();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  data-testid="part-delete-dialog"
  class="fixed inset-0 bg-press-overlay flex items-center justify-center z-press-modal"
  role="dialog"
  aria-modal="true"
  aria-labelledby="dialog-title"
  tabindex="-1"
>
  <div
    class="app-dialog-surface bg-press-surface rounded-lg p-6 max-w-md w-full mx-4 shadow-press-overlay"
  >
    <h3 id="dialog-title" class="text-press-body-lg font-heading font-medium text-press-text mb-4">
      {t("Delete {type}", { type: t(partLabel) })}
    </h3>
    <p class="text-press-muted text-press-ui mb-2">
      {t('"{title}" contains {count} {type}.', {
        title: partTitle,
        count: childChapterCount,
        type: t(chapterLabel),
      })}
    </p>
    <p class="text-press-muted text-press-ui mb-6">{t("What would you like to do?")}</p>

    <div class="space-y-3 mb-6">
      <button
        data-testid="delete-part-only"
        onclick={onDeletePartOnly}
        class="w-full text-left px-4 py-3 bg-press-sunken rounded-lg hover:bg-press-sunken transition-colors border border-transparent hover:border-press-accent"
      >
        <div class="font-medium text-press-text">
          {t("Delete {type} only", { type: t(partLabel) })}
        </div>
        <div class="text-press-eyebrow text-press-muted mt-1">
          {t("The {count} {type} will remain in the project", {
            count: childChapterCount,
            type: t(chapterLabel),
          })}
        </div>
      </button>

      <button
        data-testid="delete-part-and-chapters"
        onclick={onDeletePartAndChapters}
        class="w-full text-left px-4 py-3 bg-press-error-wash rounded-lg hover:bg-press-error-wash transition-colors border border-press-error"
      >
        <div class="font-medium text-press-error">
          {t("Delete {part} and all {chapters}", {
            part: t(partLabel),
            chapters: t(chapterLabel),
          })}
        </div>
        <div class="text-press-eyebrow text-press-error mt-1">
          {t("This will permanently delete the {part} and its {count} {chapters}", {
            part: t(partLabel),
            count: childChapterCount,
            chapters: t(chapterLabel),
          })}
        </div>
      </button>
    </div>

    <div class="flex justify-end">
      <button
        data-testid="dialog-cancel"
        onclick={onCancel}
        class="px-4 py-2 bg-press-sunken rounded hover:bg-press-sunken transition-colors text-press-text"
      >
        {t("Cancel")}
      </button>
    </div>
  </div>
</div>
