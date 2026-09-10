import { afterEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import { invoke } from "@tauri-apps/api/core";
import BeatView from "./BeatView.svelte";
import { currentProject } from "../stores/project.svelte";
import { ui } from "../stores/ui.svelte";

vi.hoisted(() => {
  vi.stubGlobal("localStorage", { getItem: () => null, setItem: () => {}, removeItem: () => {} });
});

afterEach(() => {
  cleanup();
  currentProject.setProject(null);
  ui.setExpandedBeat(null);
  vi.restoreAllMocks();
});

it("updates the beat title and scroll target on scene switch without needing a hover", async () => {
  const first = {
    id: "first-beat",
    scene_id: "first-scene",
    content: "First scene's beat",
    prose: null,
    position: 0,
  };
  const second = {
    ...first,
    id: "second-beat",
    scene_id: "second-scene",
    content: "Second scene's beat",
  };
  const view = render(BeatView, { beats: [first] });
  await view.rerender({ beats: [second] });
  expect(screen.queryByText(first.content)).toBeNull();
  expect(screen.getByTestId("beat-header").textContent).toContain(second.content);
  const row = screen.getByTestId("beat-item");
  const scroll = vi.fn();
  row.scrollIntoView = scroll;
  await fireEvent.click(screen.getByTestId("beat-header"));
  expect(scroll).toHaveBeenCalledWith({ behavior: "smooth", block: "start" });
  expect(ui.expandedBeatId).toBe(second.id);
});

it("does not submit a beat rename while an input method is composing", async () => {
  const beat = {
    id: "beat",
    scene_id: "scene",
    content: "Original",
    prose: null,
    position: 0,
  };
  render(BeatView, { beats: [beat] });
  await fireEvent.click(screen.getByTestId("beat-menu-button"));
  await fireEvent.click(screen.getByRole("menuitem", { name: "Rename" }));
  const input = screen.getByDisplayValue("Original");
  await fireEvent.keyDown(input, { key: "Enter", isComposing: true });
  expect(input.isConnected).toBe(true);
  expect(invoke).not.toHaveBeenCalledWith("rename_beat", expect.anything());
});
