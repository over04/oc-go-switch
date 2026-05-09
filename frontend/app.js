// Theme management
const THEME_KEY = 'oc-go-switch-theme';
let themeMode = localStorage.getItem(THEME_KEY) || 'auto';

function applyTheme(mode) {
  const root = document.documentElement;
  if (mode === 'dark') {
    root.classList.add('dark');
    root.classList.remove('light');
  } else if (mode === 'light') {
    root.classList.remove('dark');
    root.classList.add('light');
  } else {
    // auto: follow system
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      root.classList.add('dark');
      root.classList.remove('light');
    } else {
      root.classList.remove('dark');
      root.classList.add('light');
    }
  }
  document.getElementById('theme-btn').textContent = mode;
}

function toggleTheme() {
  const modes = ['auto', 'dark', 'light'];
  const idx = modes.indexOf(themeMode);
  themeMode = modes[(idx + 1) % 3];
  localStorage.setItem(THEME_KEY, themeMode);
  applyTheme(themeMode);
}

// Listen for system theme changes
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
  if (themeMode === 'auto') applyTheme('auto');
});

applyTheme(themeMode);

// ---------------------------------------------------------------------------
// Data fetching
// ---------------------------------------------------------------------------
async function fetchStatus() {
  try {
    const resp = await fetch('/pool/status');
    if (!resp.ok) throw new Error('HTTP ' + resp.status);
    const data = await resp.json();
    render(data);
    document.getElementById('health-dot').className = 'w-2 h-2 rounded-full bg-green-500 inline-block';
  } catch (e) {
    document.getElementById('health-dot').className = 'w-2 h-2 rounded-full bg-red-500 inline-block';
    console.error('Fetch status failed:', e);
  }
  document.getElementById('last-refresh').textContent = new Date().toLocaleTimeString();
}

// ---------------------------------------------------------------------------
// Render
// ---------------------------------------------------------------------------
function render(data) {
  document.getElementById('status-summary').textContent =
    data.total_keys + ' keys / ' + data.available_keys + ' available / ' + data.depleted_keys + ' depleted' +
    (data.current_key_id ? ' | active: ' + data.current_key_id.split('/').pop() : '');

  const container = document.getElementById('accounts');
  container.innerHTML = '';

  for (const acct of data.accounts) {
    const card = document.createElement('div');
    card.className = 'card bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700';

    const wsCount = acct.workspaces.length;
    const keyCount = acct.workspaces.reduce((s, w) => s + w.keys.length, 0);

    // Card header
    const header = document.createElement('div');
    header.className = 'flex items-center justify-between mb-2 pb-2 border-b border-gray-100 dark:border-gray-700';
    header.innerHTML =
      '<div>' +
        '<span class="font-medium text-xs">' + esc(acct.label) + '</span>' +
        '<span class="text-2xs text-gray-400 ml-2">' + esc(acct.name) + '</span>' +
      '</div>' +
      '<span class="text-2xs text-gray-400">' + wsCount + ' ws / ' + keyCount + ' keys</span>';
    card.appendChild(header);

    // Workspace list
    for (const ws of acct.workspaces) {
      const wsSection = document.createElement('div');
      wsSection.className = 'mb-2 last:mb-0';

      const wsHeader = document.createElement('div');
      wsHeader.className = 'flex items-center gap-2 mb-1';
      wsHeader.innerHTML =
        '<span class="text-2xs font-medium">' + esc(ws.name) + '</span>' +
        (ws.subscribed
          ? '<span class="badge bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300">Go</span>'
          : '<span class="badge bg-gray-100 dark:bg-gray-700 text-gray-500">-</span>');
      wsSection.appendChild(wsHeader);

      // Key rows
      for (const key of ws.keys) {
        const row = document.createElement('div');
        row.className = 'key-row flex items-center justify-between bg-gray-50 dark:bg-gray-850 rounded mb-0.5';

        let statusClass = '';
        let statusText = '';
        if (key.status === 'active') {
          statusClass = 'text-green-600 dark:text-green-400';
          statusText = 'active';
        } else if (key.status === 'depleted') {
          statusClass = 'text-red-500';
          statusText = 'depleted';
        } else {
          statusClass = 'text-gray-400';
          statusText = 'idle';
        }

        const left = document.createElement('div');
        left.className = 'flex items-center gap-2';
        left.innerHTML =
          '<code class="text-2xs text-gray-600 dark:text-gray-400 font-mono">' + esc(key.masked) + '</code>' +
          '<button class="copy-btn bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 rounded text-2xs" onclick="copyKey(\'' + esc(key.id) + '\')" title="Copy key ID">cp</button>';

        const right = document.createElement('div');
        right.className = 'flex items-center gap-2';
        right.innerHTML =
          '<span class="text-2xs font-mono">$' + key.balance_dollars.toFixed(2) + '</span>' +
          '<span class="text-2xs ' + statusClass + '">' + statusText + '</span>';

        row.appendChild(left);
        row.appendChild(right);
        wsSection.appendChild(row);
      }

      card.appendChild(wsSection);
    }

    container.appendChild(card);
  }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------
function esc(s) {
  if (!s) return '';
  return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
}

function copyKey(id) {
  navigator.clipboard.writeText(id).then(() => {
    // brief visual feedback could go here
  }).catch(() => {});
}

// ---------------------------------------------------------------------------
// Polling
// ---------------------------------------------------------------------------
fetchStatus();
setInterval(fetchStatus, 5000);
