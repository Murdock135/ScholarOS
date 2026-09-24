import { invoke as tauriInvoke } from "@tauri-apps/api/core";
async function invoke<T>(
  method: string,
  args: Record<string, unknown>,
): Promise<T> {
  if (import.meta.env.MODE === "test") {
    const response = await fetch("/__test", {
      method: "POST",
      body: JSON.stringify({ method, args }),
    });
    const result = await response.json();
    if (result.error) throw Error(result.error);
    return result.value as T;
  }
  return tauriInvoke<T>(method, args);
}
export type Section = "scratch" | "log" | "milestones";
export interface Context {
  id: string;
  kind: "area" | "project";
  name: string;
  area_id: string | null;
  last_section: Section;
}
export interface Note {
  id: string;
  kind: "scratch" | "log";
  body: string;
  revision: number;
  created_at: number;
  updated_at: number;
}
export interface WorkspaceFile {
  note_id: string;
  context_id: string;
  kind: "scratch" | "log";
  relative_path: string;
}
export interface Selection {
  context_id: string;
  section: string;
  note_id: string;
  cursor: number;
}
export interface View {
  contexts: Context[];
  workspace: { context_id: string | null; section: Section };
  notes: Note[];
  files: WorkspaceFile[];
  selection: Selection | null;
  milestones: { id: string; project_id: string; name: string }[];
  tasks: {
    id: string;
    milestone_id: string;
    name: string;
    done: boolean;
    completed_at: number | null;
  }[];
}
export interface Preview {
  contexts: number;
  notes: number;
  milestones: number;
  tasks: number;
}
export type Command = { type: string; [key: string]: unknown };
export const execute = (command: Command) =>
  invoke<View>("execute", { command });
export const previewBackup = (json: string) =>
  invoke<Preview>("preview_backup", { json });
export const restoreBackup = (json: string) =>
  invoke<string>("restore_backup", { json });
export const exportBackup = (path: string) =>
  invoke<void>("export_backup", { path });

export const openWorkspace = () => invoke<string>("open_workspace", {});
