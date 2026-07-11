(function () {
  const root = document.querySelector("[data-install-root]");
  if (!root) return;

  const archSelect = root.querySelector("[data-install-arch]");
  const panels = [...root.querySelectorAll("[data-install-option]")];

  function selectMethod(panel, methodIndex) {
    const tabs = [...panel.querySelectorAll("[data-install-method]")];
    const commands = [...panel.querySelectorAll("[data-install-command]")];

    tabs.forEach((tab) => {
      tab.setAttribute("aria-selected", String(tab.dataset.installMethod === methodIndex));
    });

    commands.forEach((command) => {
      command.hidden = command.dataset.installCommand !== methodIndex;
    });
  }

  function selectArchitecture(id) {
    archSelect.value = id;

    panels.forEach((panel) => {
      const selected = panel.dataset.installOption === id;
      panel.hidden = !selected;
      if (selected) {
        selectMethod(panel, "0");
      }
    });
  }

  async function copyCommand(button) {
    const command = button.closest(".install-command-row")?.querySelector("code")?.textContent;
    if (!command) return;

    try {
      await navigator.clipboard.writeText(command);
    } catch {
      const fallback = document.createElement("textarea");
      fallback.value = command;
      fallback.style.position = "fixed";
      fallback.style.opacity = "0";
      document.body.append(fallback);
      fallback.select();
      document.execCommand("copy");
      fallback.remove();
    }

    button.textContent = "Copied";
    button.classList.add("is-copied");
    window.setTimeout(() => {
      button.textContent = "Copy";
      button.classList.remove("is-copied");
    }, 1600);
  }

  panels.forEach((panel) => {
    panel.querySelectorAll("[data-install-method]").forEach((tab) => {
      tab.addEventListener("click", () => selectMethod(panel, tab.dataset.installMethod));
    });
  });

  archSelect.addEventListener("change", () => selectArchitecture(archSelect.value));

  root.querySelectorAll("[data-install-copy]").forEach((button) => {
    button.addEventListener("click", () => copyCommand(button));
  });

  // Guess the visitor's platform from the user agent and preselect it. We can't
  // tell Apple Silicon from Intel in the browser, so macOS defaults to arm.
  function guessPlatform() {
    const ua = `${navigator.userAgent} ${navigator.platform}`.toLowerCase();
    if (ua.includes("win")) return "windows-x64";
    if (ua.includes("mac")) return "macos-arm64";
    if (ua.includes("linux")) return "linux-x64";
    return null;
  }

  const guess = guessPlatform();
  if (guess && archSelect.querySelector(`option[value="${guess}"]`)) {
    selectArchitecture(guess);
  }
})();
