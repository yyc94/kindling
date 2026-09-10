import { afterEach, describe, expect, it } from "vitest";
import { cleanup, render, screen } from "@testing-library/svelte";
import { t } from "./i18n.svelte";
import { ui } from "./stores/ui.svelte";
import CommandPalette from "./components/CommandPalette.svelte";

describe("translations", () => {
  afterEach(() => {
    cleanup();
    ui.setLocale("en");
  });

  it("uses English as the source-language fallback", () => {
    ui.setLocale("en");
    expect(t("Language")).toBe("Language");
    expect(t("Unknown text")).toBe("Unknown text");
  });

  it("translates Chinese UI text and interpolates values", () => {
    ui.setLocale("zh-CN");
    expect(t("Language")).toBe("语言");
    expect(t("Unknown {count}", { count: 2 })).toBe("Unknown 2");
  });

  it("renders translated component text", () => {
    ui.setLocale("zh-CN");
    HTMLElement.prototype.scrollIntoView = () => {};
    render(CommandPalette, {
      open: true,
      commands: [
        { id: "export", label: "Export project", shortcut: "⌘E", category: "File", action() {} },
      ],
    });

    expect(screen.getByRole("dialog", { name: "命令面板" })).toBeTruthy();
    expect(screen.getByText("导出项目")).toBeTruthy();
  });
});
