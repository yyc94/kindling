<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { runImport, type ImportType } from "../utils/import";
  import {
    FileText,
    HelpCircle,
    Kanban,
    Trash2,
    Loader2,
    Settings,
    PenTool,
    BookOpen,
    FilePlus,
    Scroll,
  } from "lucide-svelte";
  import { currentProject } from "../stores/project.svelte";
  import { ui } from "../stores/ui.svelte";
  import type { Project } from "../types";
  import Tooltip from "./Tooltip.svelte";
  import ConfirmDialog from "./ConfirmDialog.svelte";
  import KindlingSettingsDialog from "./KindlingSettingsDialog.svelte";
  import BrandMark from "./BrandMark.svelte";
  import { t } from "../i18n.svelte";

  interface Props {
    recentProjects: Project[];
    onImportLongform?: () => void;
    onImportComplete?: (project: Project, type: ImportType) => void;
    onOpenQuickStart?: () => void;
    onNewProject?: () => void;
    onOpenEditorial?: () => void;
  }

  let {
    recentProjects = $bindable(),
    onImportLongform,
    onImportComplete,
    onOpenQuickStart,
    onNewProject,
    onOpenEditorial,
  }: Props = $props();

  let deletingProjectId = $state<string | null>(null);
  let hoveredProjectId = $state<string | null>(null);
  let projectToDelete = $state<Project | null>(null);
  let showSettingsDialog = $state(false);
  let showAllProjects = $state(false);

  async function handleImport(type: ImportType) {
    const project = await runImport(type);
    if (!project) return;
    currentProject.setProject(project);
    ui.setView("editor");
    onImportComplete?.(project, type);
  }

  const importPlottr = () => handleImport("plottr");
  const importMarkdown = () => handleImport("markdown");
  const importYWriter = () => handleImport("ywriter");
  const importLongform = () => handleImport("longform");
  const importScrivener = () => handleImport("scrivener");
  const importNovelWriter = () => handleImport("novelwriter");

  function handleLongformImport() {
    const handler = onImportLongform ?? importLongform;
    handler();
  }

  async function trySampleProject() {
    ui.startImport();
    try {
      const project = await invoke<Project>("create_sample_project");
      currentProject.setProject(project);
      ui.setView("editor");
    } catch (e) {
      console.error("Failed to create sample project:", e);
      ui.showError(t("Failed to create sample project: {error}", { error: String(e) }));
    } finally {
      ui.finishImport();
    }
  }

  async function openProject(project: Project) {
    try {
      const loaded = await invoke<Project>("get_project", { id: project.id });
      currentProject.setProject(loaded);
      ui.setView("editor");
      // Sidebar $effect will run loadChapters
    } catch (e) {
      console.error("Failed to open project:", e);
      ui.showError(t("Failed to open project: {error}", { error: String(e) }));
    }
  }

  async function toggleProjectList() {
    showAllProjects = !showAllProjects;
    try {
      if (showAllProjects) {
        recentProjects = await invoke("get_all_projects");
      } else {
        recentProjects = await invoke("get_recent_projects");
      }
    } catch (e) {
      console.error("Failed to load projects:", e);
      ui.showError(t("Failed to load projects: {error}", { error: String(e) }));
    }
  }

  function showDeleteConfirmation(event: MouseEvent, project: Project) {
    event.stopPropagation();
    projectToDelete = project;
  }

  async function confirmDeleteProject() {
    if (!projectToDelete) return;

    const project = projectToDelete;
    projectToDelete = null;
    deletingProjectId = project.id;

    try {
      await invoke("delete_project", { projectId: project.id });
      recentProjects = recentProjects.filter((p) => p.id !== project.id);
    } catch (e) {
      console.error("Failed to delete project:", e);
      ui.showError(t("Failed to delete project: {error}", { error: String(e) }));
    } finally {
      deletingProjectId = null;
    }
  }

  function cancelDeleteProject() {
    projectToDelete = null;
  }
</script>

<div
  class="flex-1 flex flex-col items-center justify-center p-10 lg:p-14 relative overflow-y-auto overflow-x-hidden"
>
  <!-- Settings and Help buttons in corner -->
  <div class="absolute top-4 right-4 flex items-center gap-1 z-press-raised">
    {#if onOpenQuickStart}
      <Tooltip text={t("Quick Start")} position="left">
        <button
          onclick={onOpenQuickStart}
          class="p-2 text-press-muted hover:text-press-text hover:bg-press-sunken rounded-lg transition-colors"
          aria-label={t("Quick Start")}
        >
          <HelpCircle class="w-5 h-5" />
        </button>
      </Tooltip>
    {/if}
    <Tooltip text={t("Kindling Settings")} position="left">
      <button
        onclick={() => (showSettingsDialog = true)}
        class="p-2 text-press-muted hover:text-press-text hover:bg-press-sunken rounded-lg transition-colors"
        aria-label={t("Kindling Settings")}
        data-testid="kindling-settings-button"
      >
        <Settings class="w-5 h-5" />
      </button>
    </Tooltip>
  </div>

  <div class="w-full max-w-6xl flex flex-col lg:flex-row gap-8 lg:gap-12 flex-1 min-h-0 min-w-0">
    <!-- Left column: branding + actions (golden ratio: ~38.2%) -->
    <div class="flex flex-col gap-6 lg:w-[38.2%] lg:min-w-0 lg:shrink-0">
      <!-- Logo & Tagline (compact) -->
      <div class="text-center lg:text-left lg:pr-4">
        <div class="flex justify-center lg:justify-start mb-2">
          <BrandMark size={72} />
        </div>
        <h1
          class="text-press-h1 lg:text-press-h1 font-heading font-semibold text-press-accent-text"
        >
          kindling
        </h1>
        <p class="text-press-muted text-press-ui lg:text-press-base">{t("Spark your draft")}</p>
      </div>

      <!-- New Project + Sample (stacked on lg) -->
      {#if onNewProject}
        <div class="bg-press-surface rounded-lg p-5 space-y-4">
          <button
            data-testid="new-project-button"
            onclick={onNewProject}
            class="w-full flex items-center gap-3 p-3 bg-press-accent-wash border-2 border-press-accent rounded-lg hover:bg-press-sunken transition-colors cursor-pointer"
          >
            <FilePlus class="w-8 h-8 text-press-accent-text shrink-0" />
            <div class="text-left">
              <span class="text-press-text font-medium block">{t("New Project")}</span>
              <span class="text-press-muted text-press-ui">{t("Start from scratch")}</span>
            </div>
          </button>
          <button
            onclick={trySampleProject}
            class="w-full flex items-center gap-3 p-3 bg-press-sunken rounded-lg hover:bg-press-sunken transition-colors cursor-pointer"
          >
            <BookOpen class="w-8 h-8 text-press-accent-text shrink-0" />
            <div class="text-left">
              <span class="text-press-text font-medium block">{t("Sample Project")}</span>
              <span class="text-press-muted text-press-ui">{t("Explore Kindling first")}</span>
            </div>
          </button>
        </div>
      {/if}

      <!-- Import Options (two-column grid) -->
      {#if onOpenEditorial}<button
          class="w-full p-3 text-left border-t border-press-border text-press-text"
          onclick={onOpenEditorial}>{t("Open a review or feedback package…")}</button
        >{/if}
      <div data-testid="import-section" class="bg-press-surface rounded-lg p-4">
        <h2 class="text-press-base font-heading font-medium text-press-text mb-3">
          {t("Import an Outline")}
        </h2>
        <div class="grid grid-cols-2 gap-2">
          <button
            onclick={importPlottr}
            class="flex flex-col items-center p-4 bg-press-sunken rounded-lg hover:bg-press-sunken transition-colors cursor-pointer"
          >
            <Kanban class="w-8 h-8 text-press-accent-text mb-1" />
            <span class="text-press-text text-press-ui font-medium">Plottr</span>
            <span class="text-press-muted text-press-eyebrow">.pltr</span>
          </button>
          <button
            onclick={importYWriter}
            class="flex flex-col items-center p-4 bg-press-sunken rounded-lg hover:bg-press-sunken transition-colors cursor-pointer"
          >
            <PenTool class="w-8 h-8 text-press-accent-text mb-1" />
            <span class="text-press-text text-press-ui font-medium">yWriter</span>
            <span class="text-press-muted text-press-eyebrow">.yw7</span>
          </button>
          <button
            onclick={importMarkdown}
            class="flex flex-col items-center p-4 bg-press-sunken rounded-lg hover:bg-press-sunken transition-colors cursor-pointer"
          >
            <FileText class="w-8 h-8 text-press-accent-text mb-1" />
            <span class="text-press-text text-press-ui font-medium">Markdown</span>
            <span class="text-press-muted text-press-eyebrow">.md</span>
          </button>
          <button
            onclick={handleLongformImport}
            class="flex flex-col items-center p-4 bg-press-sunken rounded-lg hover:bg-press-sunken transition-colors cursor-pointer"
          >
            <BookOpen class="w-8 h-8 text-press-accent-text mb-1" />
            <span class="text-press-text text-press-ui font-medium">Longform</span>
            <span class="text-press-muted text-press-eyebrow">{t("Index or vault")}</span>
          </button>
          <button
            onclick={importScrivener}
            class="flex flex-col items-center p-4 bg-press-sunken rounded-lg hover:bg-press-sunken transition-colors cursor-pointer"
          >
            <Scroll class="w-8 h-8 text-press-accent-text mb-1" />
            <span class="text-press-text text-press-ui font-medium">Scrivener</span>
            <span class="text-press-muted text-press-eyebrow">.scriv</span>
          </button>
          <button
            onclick={importNovelWriter}
            class="flex flex-col items-center p-4 bg-press-sunken rounded-lg hover:bg-press-sunken transition-colors cursor-pointer"
          >
            <Scroll class="w-8 h-8 text-press-accent-text mb-1" />
            <span class="text-press-text text-press-ui font-medium">novelWriter</span>
            <span class="text-press-muted text-press-eyebrow">{t("Project folder")}</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Right column: project list (golden ratio: ~61.8%) -->
    <div class="flex-1 flex flex-col min-h-0 min-w-0 bg-press-surface rounded-lg p-6 lg:p-8">
      {#if recentProjects.length > 0}
        <div data-testid="recent-projects" class="flex flex-col flex-1 min-h-0">
          <div class="flex items-center justify-between mb-4 shrink-0">
            <h2 class="text-press-h3 font-heading font-medium text-press-text">
              {t(showAllProjects ? "All Projects" : "Recent Projects")}
            </h2>
            <button
              onclick={toggleProjectList}
              class="text-press-eyebrow text-press-muted hover:text-press-text transition-colors"
            >
              {t(showAllProjects ? "Show recent" : "View all")}
            </button>
          </div>
          <div class="space-y-3 overflow-y-auto pr-1 flex-1 min-h-0">
            {#each recentProjects as project}
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div
                class="relative flex items-center bg-press-sunken rounded-lg hover:bg-press-sunken transition-colors"
                onmouseenter={() => (hoveredProjectId = project.id)}
                onmouseleave={() => (hoveredProjectId = null)}
              >
                <button
                  data-testid="project-card"
                  onclick={() => openProject(project)}
                  class="flex-1 flex items-center justify-between p-3 cursor-pointer text-left"
                >
                  <div>
                    <span class="text-press-text font-medium">{project.name}</span>
                    <span class="text-press-muted text-press-ui ml-2">({project.source_type})</span>
                  </div>
                  <span class="text-press-muted text-press-ui">
                    {new Date(project.modified_at).toLocaleDateString(ui.locale)}
                  </span>
                </button>

                <!-- Delete button - visible on hover -->
                <div
                  class="pr-3 transition-opacity"
                  class:opacity-0={hoveredProjectId !== project.id &&
                    deletingProjectId !== project.id}
                  class:opacity-100={hoveredProjectId === project.id ||
                    deletingProjectId === project.id}
                >
                  <Tooltip text={t("Delete project")} position="left">
                    <button
                      onclick={(e) => showDeleteConfirmation(e, project)}
                      disabled={deletingProjectId === project.id}
                      class="p-1.5 text-press-muted hover:text-press-error hover:bg-press-error-wash rounded-lg transition-colors"
                      aria-label={t("Delete project")}
                    >
                      {#if deletingProjectId === project.id}
                        <Loader2 class="w-4 h-4 animate-spin" />
                      {:else}
                        <Trash2 class="w-4 h-4" />
                      {/if}
                    </button>
                  </Tooltip>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {:else}
        <div class="flex-1 flex items-center justify-center text-press-muted text-press-ui">
          <p>{t("Your projects will appear here")}</p>
        </div>
      {/if}
    </div>
  </div>

  <!-- Import Progress -->
  {#if ui.isImporting}
    <div class="fixed inset-0 bg-press-overlay flex items-center justify-center z-press-modal">
      <div class="bg-press-surface rounded-lg p-6 max-w-md w-full mx-4">
        <h3 class="text-press-body-lg font-heading font-medium text-press-text mb-4">
          {t("Importing...")}
        </h3>
        <div class="w-full bg-press-sunken rounded-full h-2 mb-2">
          <div
            class="bg-press-accent h-2 rounded-full transition-all"
            style="width: {ui.importProgress}%"
          ></div>
        </div>
        <p class="text-press-muted text-press-ui">{t(ui.importStatus)}</p>
      </div>
    </div>
  {/if}
</div>

<!-- Delete Project Confirmation -->
{#if projectToDelete}
  <ConfirmDialog
    title={t("Delete Project")}
    message={t(
      'Are you sure you want to delete "{name}"? This will permanently delete the project and all its chapters, scenes, beats, and snapshots. This cannot be undone.',
      { name: projectToDelete.name }
    )}
    confirmLabel={t("Delete Project")}
    onConfirm={confirmDeleteProject}
    onCancel={cancelDeleteProject}
  />
{/if}

<!-- Kindling Settings Dialog -->
{#if showSettingsDialog}
  <KindlingSettingsDialog
    onClose={() => (showSettingsDialog = false)}
    onSave={() => (showSettingsDialog = false)}
  />
{/if}
