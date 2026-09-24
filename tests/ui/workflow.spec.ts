import { test, expect } from "@playwright/test";
import { readdir, rename, unlink, writeFile } from "node:fs/promises";
import path from "node:path";

async function findDirectory(
  root: string,
  namePrefix: string,
): Promise<string> {
  for (const entry of await readdir(root, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;
    const child = path.join(root, entry.name);
    if (entry.name.startsWith(namePrefix)) return child;
    const found = await findDirectory(child, namePrefix).catch(() => "");
    if (found) return found;
  }
  throw Error(
    `Could not find a directory starting with ${namePrefix} below ${root}`,
  );
}
test("real SQLite workflow: scoped notes, autosave, reopen, hierarchy and validated restore", async ({
  page,
  request,
}) => {
  await page.goto("/");
  await page.getByRole("button", { name: "Create your first Area" }).click();
  await page.getByRole("textbox", { name: "Area name" }).fill("Research");
  await page.getByRole("button", { name: "Add", exact: true }).click();
  await expect(
    page.locator(".breadcrumbs").getByText("Research", { exact: true }),
  ).toBeVisible();
  await page.getByRole("button", { name: "+ Project", exact: true }).click();
  await page
    .getByRole("textbox", { name: "Project name" })
    .fill("Evidence synthesis");
  await page.getByRole("button", { name: "Add", exact: true }).click();
  await expect(
    page
      .locator(".breadcrumbs")
      .getByText("Evidence synthesis", { exact: true }),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "New note", exact: true })
    .first()
    .click();
  await page
    .getByRole("textbox", { name: "Note body" })
    .fill("A working hypothesis\nEvidence is not the same as consensus.");
  // Switch before the autosave debounce expires.
  await page.getByRole("tab", { name: "Logs", exact: true }).click();
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveCount(0);
  await page
    .getByRole("button", { name: "New note", exact: true })
    .first()
    .click();
  await page
    .getByRole("textbox", { name: "Note body" })
    .fill("Reviewed the first three papers.");
  await page.getByRole("button", { name: "+ Project", exact: true }).click();
  await page.getByRole("textbox", { name: "Project name" }).fill("Teaching");
  await page.getByRole("button", { name: "Add", exact: true }).click();
  await expect(
    page.locator(".breadcrumbs").getByText("Teaching", { exact: true }),
  ).toBeVisible();
  await expect(
    page.getByText("A working hypothesis", { exact: true }),
  ).toHaveCount(0);
  await page
    .getByRole("button", { name: "New note", exact: true })
    .first()
    .click();
  await page
    .getByRole("textbox", { name: "Note body" })
    .fill("Seminar outline");
  await page
    .locator("summary")
    .filter({ hasText: "Evidence synthesis" })
    .click();
  await expect(
    page.getByRole("tab", { name: "Logs", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveValue(
    "Reviewed the first three papers.",
  );
  await page.getByRole("tab", { name: "Scratchpad", exact: true }).click();
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveValue(
    "A working hypothesis\nEvidence is not the same as consensus.",
  );
  await expect(page.getByText("Seminar outline", { exact: true })).toHaveCount(
    0,
  );
  await page
    .getByRole("button", { name: "New note", exact: true })
    .first()
    .click();
  await page
    .getByRole("textbox", { name: "Note body" })
    .fill("Second scratch note");
  await expect(
    page.getByRole("status").filter({ hasText: "Saved locally" }),
  ).toBeVisible();
  await page.reload();
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveValue(
    "Second scratch note",
  );
  await page.getByRole("tab", { name: "Milestones", exact: true }).click();
  await page
    .getByRole("textbox", { name: "New milestone", exact: true })
    .fill("Complete synthesis");
  await page.getByRole("button", { name: "Add", exact: true }).click();
  await page
    .getByRole("textbox", { name: "Add task to Complete synthesis" })
    .fill("Compare the definitions");
  await page
    .locator(".milestone")
    .getByRole("button", { name: "Add", exact: true })
    .click();
  await page.getByRole("checkbox", { name: "Compare the definitions" }).click();
  await expect(
    page.getByRole("checkbox", { name: "Compare the definitions" }),
  ).toBeChecked();
  await page.reload();
  await expect(
    page.getByRole("checkbox", { name: "Compare the definitions" }),
  ).toBeChecked();
  const res = await request.post("http://127.0.0.1:4319", {
    data: JSON.stringify({ method: "test_backup", args: {} }),
  });
  const backup = (await res.json()).value as string;
  await page.getByRole("tab", { name: "Scratchpad", exact: true }).click();
  await page
    .getByRole("textbox", { name: "Note body" })
    .fill("Changed after backup");
  await expect(
    page.getByRole("status").filter({ hasText: "Saved locally" }),
  ).toBeVisible();
  await page.getByLabel("Restore backup file").setInputFiles({
    name: "bad.json",
    mimeType: "application/json",
    buffer: Buffer.from('{"version":99}'),
  });
  await expect(page.getByRole("alert")).toBeVisible();
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveValue(
    "Changed after backup",
  );
  await page.getByLabel("Restore backup file").setInputFiles({
    name: "valid.json",
    mimeType: "application/json",
    buffer: Buffer.from(backup),
  });
  await expect(page.getByRole("dialog")).toContainText("Validated backup");
  await page
    .getByRole("button", { name: "Replace workspace", exact: true })
    .click();
  await expect(
    page.getByRole("checkbox", { name: "Compare the definitions" }),
  ).toBeChecked();
  await page.getByRole("tab", { name: "Scratchpad", exact: true }).click();
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveValue(
    "Second scratch note",
  );
  await page
    .locator("details.nested-context")
    .filter({ hasText: "Evidence synthesis" })
    .getByRole("button", { name: "A working hypothesis.md", exact: true })
    .click();
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveValue(
    "A working hypothesis\nEvidence is not the same as consensus.",
  );
  await page.screenshot({ path: "test-results/workspace.png", fullPage: true });
});

test("failed saves block navigation and preserve recoverable drafts across reload", async ({
  page,
  request,
}) => {
  await page.goto("/");
  await page.getByRole("button", { name: "+ Project", exact: true }).click();
  await page
    .getByRole("textbox", { name: "Project name" })
    .fill("Recovery check");
  await page.getByRole("button", { name: "Add", exact: true }).click();
  await expect(
    page.locator(".breadcrumbs").getByText("Recovery check", { exact: true }),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "New note", exact: true })
    .first()
    .click();
  let rejectSave = true;
  await page.route("**/__test", async (route) => {
    const payload = route.request().postDataJSON();
    if (
      rejectSave &&
      payload.method === "execute" &&
      payload.args.command.type === "save_note"
    ) {
      await route.fulfill({ json: { error: "Simulated disk write failure" } });
    } else await route.continue();
  });
  await page
    .getByRole("textbox", { name: "Note body" })
    .fill("Keep this recoverable draft.");
  await page.getByRole("tab", { name: "Logs", exact: true }).click();
  await expect(
    page.getByRole("tab", { name: "Scratchpad", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveValue(
    "Keep this recoverable draft.",
  );
  await expect(
    page.getByRole("button", { name: "Retry save", exact: true }),
  ).toBeVisible();
  expect(
    await page.evaluate(() =>
      Object.keys(localStorage).some((k) => k.startsWith("scholaros:draft:")),
    ),
  ).toBe(true);
  page.on("dialog", (dialog) => dialog.accept());
  await page.reload();
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveValue(
    "Keep this recoverable draft.",
  );
  await page.getByRole("tab", { name: "Logs", exact: true }).click();
  await expect(
    page.getByRole("button", { name: "Retry save", exact: true }),
  ).toBeVisible();
  rejectSave = false;
  await page.getByRole("button", { name: "Retry save", exact: true }).click();
  await expect(
    page.getByRole("status").filter({ hasText: "Saved locally" }),
  ).toBeVisible();
  await page.reload();
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveValue(
    "Keep this recoverable draft.",
  );
  expect(
    await page.evaluate(() =>
      Object.keys(localStorage).some((k) => k.startsWith("scholaros:draft:")),
    ),
  ).toBe(false);
  // A second writer commits while this editor still holds its original revision.
  const currentResponse = await request.post("http://127.0.0.1:4319", {
    data: JSON.stringify({
      method: "execute",
      args: { command: { type: "view" } },
    }),
  });
  const current = (await currentResponse.json()).value;
  const existing = current.notes.find(
    (n: { id: string }) => n.id === current.selection.note_id,
  );
  await request.post("http://127.0.0.1:4319", {
    data: JSON.stringify({
      method: "execute",
      args: {
        command: {
          type: "save_note",
          context_id: current.workspace.context_id,
          note_id: existing.id,
          body: "Saved elsewhere",
          revision: existing.revision,
          cursor: 0,
        },
      },
    }),
  });
  await page
    .getByRole("textbox", { name: "Note body" })
    .fill("My competing draft");
  await expect(
    page.getByRole("button", { name: "Save as recovery copy", exact: true }),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Save as recovery copy", exact: true })
    .click();
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveValue(
    "My competing draft",
  );
  await expect(
    page.getByRole("status").filter({ hasText: "Saved locally" }),
  ).toBeVisible();
  await page
    .locator("details.nested-context")
    .filter({ hasText: "Recovery check" })
    .getByRole("button", { name: "Untitled.md", exact: true })
    .click();
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveValue(
    "Saved elsewhere",
  );
  await page.getByRole("tab", { name: "Logs", exact: true }).click();
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveCount(0);
});

test("Markdown created, moved, and deleted outside ScholarOS is reconciled", async ({
  page,
  request,
}) => {
  await page.goto("/");
  await page.getByRole("button", { name: "+ Project", exact: true }).click();
  await page
    .getByRole("textbox", { name: "Project name" })
    .fill("Terminal workspace");
  await page.getByRole("button", { name: "Add", exact: true }).click();

  const response = await request.post("http://127.0.0.1:4319", {
    data: JSON.stringify({ method: "test_workspace_path", args: {} }),
  });
  const workspace = (await response.json()).value as string;
  const projectRoot = await findDirectory(workspace, "Terminal workspace--");
  const scratch = path.join(projectRoot, "Scratchpad");
  const logs = path.join(projectRoot, "Logs");
  const created = path.join(scratch, "Created in terminal.md");
  const moved = path.join(logs, "Renamed in files.md");

  await writeFile(created, "Created outside ScholarOS");
  await page.reload();
  await expect(
    page.getByRole("button", { name: "Created in terminal.md", exact: true }),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Created in terminal.md", exact: true })
    .click();
  await expect(page.getByRole("textbox", { name: "Note body" })).toHaveValue(
    "Created outside ScholarOS",
  );

  await rename(created, moved);
  await page.getByRole("tab", { name: "Logs", exact: true }).click();
  await expect(
    page.getByRole("button", { name: "Created in terminal.md", exact: true }),
  ).toBeVisible();

  await unlink(moved);
  await page.getByRole("tab", { name: "Scratchpad", exact: true }).click();
  await page.getByRole("tab", { name: "Logs", exact: true }).click();
  await expect(
    page.getByRole("button", { name: "Created in terminal.md", exact: true }),
  ).toHaveCount(0);
});
