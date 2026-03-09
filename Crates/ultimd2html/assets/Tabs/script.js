(function () {

const STORAGE_PREFIX = "tabs-sync:";

/* -------------------------------- */
/* ACTIVE TAB HANDLING */
/* -------------------------------- */

function setActiveTab(tabsRoot, index) {

  const buttons = tabsRoot.querySelectorAll(".tab-button");
  const panels = tabsRoot.querySelectorAll(".tab-panel");

  buttons.forEach(btn => btn.classList.remove("active"));
  panels.forEach(panel => panel.classList.remove("active"));

  const button = tabsRoot.querySelector(`.tab-button[data-tab="${index}"]`);
  const panel = tabsRoot.querySelector(`.tab-panel[data-tab="${index}"]`);

  if (button) button.classList.add("active");
  if (panel) panel.classList.add("active");
}

/* -------------------------------- */
/* URL PARAM HELPERS */
/* -------------------------------- */

function getUrlTabSelection() {

  const params = new URLSearchParams(window.location.search);

  const tabParam = params.get("tab");

  if (!tabParam) return null;

  const parts = tabParam.split(":");

  if (parts.length !== 2) return null;

  return {
    key: parts[0],
    label: parts[1]
  };
}

function updateUrl(syncKey, label) {

  const params = new URLSearchParams(window.location.search);

  params.set("tab", `${syncKey}:${label.toLowerCase()}`);

  const newUrl =
    window.location.pathname +
    "?" +
    params.toString() +
    window.location.hash;

  window.history.replaceState(null, "", newUrl);
}

/* -------------------------------- */
/* TAB SYNCING */
/* -------------------------------- */

function syncTabs(syncKey, index, label) {

  const groups = document.querySelectorAll(`.tabs[data-sync-key="${syncKey}"]`);

  groups.forEach(group => {
    setActiveTab(group, index);
  });

  try {
    localStorage.setItem(STORAGE_PREFIX + syncKey, index);
  } catch (e) {}

  updateUrl(syncKey, label);
}

/* -------------------------------- */
/* CLICK HANDLER */
/* -------------------------------- */

function handleTabClick(event) {

  const button = event.currentTarget;
  const tabsRoot = button.closest(".tabs");

  const index = button.dataset.tab;
  const label = button.textContent.trim();

  setActiveTab(tabsRoot, index);

  const syncKey = tabsRoot.dataset.syncKey;

  if (syncKey) {
    syncTabs(syncKey, index, label);
  }
}

/* -------------------------------- */
/* KEYBOARD NAVIGATION */
/* -------------------------------- */

function handleKeyboard(root, button, e) {

  const tabs = Array.from(root.querySelectorAll(".tab-button"));
  const currentIndex = tabs.indexOf(button);

  if (e.key === "ArrowRight") {

    const next = tabs[(currentIndex + 1) % tabs.length];
    next.focus();
    next.click();

  }

  if (e.key === "ArrowLeft") {

    const prev = tabs[(currentIndex - 1 + tabs.length) % tabs.length];
    prev.focus();
    prev.click();

  }
}

/* -------------------------------- */
/* LOAD SAVED TAB */
/* -------------------------------- */

function loadSavedTab(root) {

  const syncKey = root.dataset.syncKey;

  if (!syncKey) return;

  try {

    const saved = localStorage.getItem(STORAGE_PREFIX + syncKey);

    if (saved !== null) {
      setActiveTab(root, saved);
    }

  } catch (e) {}

}

/* -------------------------------- */
/* LOAD TAB FROM URL */
/* -------------------------------- */

function loadUrlTab(root) {

  const selection = getUrlTabSelection();

  if (!selection) return;

  const syncKey = root.dataset.syncKey;

  if (!syncKey || selection.key !== syncKey) return;

  const buttons = root.querySelectorAll(".tab-button");

  buttons.forEach(button => {

    const label = button.textContent.trim().toLowerCase();

    if (label === selection.label) {

      const index = button.dataset.tab;

      setActiveTab(root, index);

    }

  });

}

/* -------------------------------- */
/* INIT */
/* -------------------------------- */

function initTabs(root) {

  const buttons = root.querySelectorAll(".tab-button");

  buttons.forEach(button => {

    button.addEventListener("click", handleTabClick);

    button.addEventListener("keydown", function(e) {
      handleKeyboard(root, button, e);
    });

  });

  loadUrlTab(root);
  loadSavedTab(root);
}

/* -------------------------------- */
/* STARTUP */
/* -------------------------------- */

document.addEventListener("DOMContentLoaded", function () {

  const groups = document.querySelectorAll(".tabs");

  groups.forEach(initTabs);

});

})();