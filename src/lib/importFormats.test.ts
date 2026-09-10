import { describe, expect, it } from "vitest";
import {
  IMPORT_COMMANDS,
  IMPORT_FORMATS,
  importTypeForCommand,
  isImportType,
  supportsSync,
} from "./importFormats";
describe("import format registry", () => {
  it("keeps picker, commands, and source types aligned", () => {
    for (const [type, config] of Object.entries(IMPORT_FORMATS)) {
      expect(isImportType(type)).toBe(true);
      expect(IMPORT_COMMANDS[type as keyof typeof IMPORT_COMMANDS]).toBe(config.command);
      expect(importTypeForCommand(config.command)).toBe(
        type === "longformVault" ? "longform" : type
      );
      expect(supportsSync(config.sourceType)).toBe(config.sync);
    }
    expect(isImportType("__proto__")).toBe(false);
    expect(isImportType("missing")).toBe(false);
    expect(importTypeForCommand("delete_project")).toBeUndefined();
    expect(supportsSync("Blank")).toBe(false);
  });
});
