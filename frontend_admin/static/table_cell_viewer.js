document.addEventListener("click", event => {
  const trigger = event.target.closest?.(".table-cell-preview");
  if (!trigger) return;

  const dialog = document.createElement("dialog");
  dialog.className = "table-cell-dialog";
  dialog.setAttribute("aria-labelledby", "table-cell-dialog-title");

  const title = document.createElement("h2");
  title.id = "table-cell-dialog-title";
  title.textContent = trigger.closest("td").dataset.label || trigger.title;
  const content = document.createElement("div");
  content.className = "table-cell-content";
  content.tabIndex = 0;
  content.textContent = trigger.textContent;
  const close = document.createElement("button");
  close.type = "button";
  close.textContent = "close";
  close.autofocus = true;
  close.addEventListener("click", () => dialog.close());

  dialog.append(title, content, close);
  dialog.addEventListener("close", () => {
    dialog.remove();
    if (trigger.isConnected) trigger.focus({ preventScroll: true });
  }, { once: true });
  document.body.append(dialog);
  dialog.showModal();
});
