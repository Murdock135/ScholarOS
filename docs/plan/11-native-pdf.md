# 11. Native PDF companion plan

## 11.1 Feasibility boundary

“Click any PDF window and automatically extract everything” is feasible only in degrees. PDF applications expose different amounts of information.

The desktop companion should use a tiered strategy:

1. **First-party in-app PDF reader**
   - Most reliable.
   - Exact page number, page count, title, and reading duration.

2. **Browser PDF viewer integration**
   - Browser extension can identify the URL and sometimes page state.

3. **Native viewer adapters**
   - Okular through Linux window metadata, DBus where available, recent documents, and file association.
   - Adobe Acrobat through supported automation/accessibility interfaces where available.
   - Platform accessibility APIs for active-window title and controls.

4. **Fallback**
   - Identify a likely file from window title.
   - Ask the user to confirm it once.
   - Remember the viewer-title-to-path mapping.
   - Let the user enter the current page if the viewer does not expose it.

Do not use screen scraping or OCR as the primary mechanism.

## 11.2 Interaction

Provide a global shortcut such as:

```text
Ctrl+Alt+L — Log reading progress
```

Flow:

1. Detect foreground window and process.
2. Resolve the PDF path or URL.
3. Extract PDF metadata and total page count.
4. Attempt to retrieve the current page.
5. Show a small overlay.
6. Let the user correct the page, title, author, and date.
7. Record progress and takeaway.
8. Link the event to an optional project and milestone.

## 11.3 Implementation order

Start with:

1. The application’s own PDF reader.
2. Browser PDFs.
3. Okular on Linux.
4. Adobe Acrobat on Windows.
5. macOS Preview.
6. Additional readers based on demand.

Trying to support every PDF reader in the first release would delay the useful core product.

---

[← Previous](10-browser-extension.md) · [Plan index](README.md) · [Next →](12-personalized-message.md)
