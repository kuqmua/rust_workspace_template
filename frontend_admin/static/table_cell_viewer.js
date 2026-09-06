document.addEventListener("click", event => {
  const trigger = event.target.closest?.(".table-cell-preview");
  if (!trigger) return;

  const dialog = document.createElement("dialog");
  dialog.className = "table-cell-dialog";
  dialog.setAttribute("aria-labelledby", "table-cell-dialog-title");

  const header = document.createElement("header");
  const title = document.createElement("h2");
  title.id = "table-cell-dialog-title";
  title.textContent = trigger.closest("td").dataset.label || trigger.title;
  const close = document.createElement("button");
  close.type = "button";
  close.textContent = "close";
  close.autofocus = true;
  close.addEventListener("click", () => dialog.close());
  header.append(title, close);

  const content = document.createElement("pre");
  content.tabIndex = 0;
  content.textContent = trigger.textContent;
  dialog.append(header, content);
  dialog.addEventListener("close", () => {
    dialog.remove();
    if (trigger.isConnected) trigger.focus({ preventScroll: true });
  }, { once: true });
  document.body.append(dialog);
  dialog.showModal();
});
