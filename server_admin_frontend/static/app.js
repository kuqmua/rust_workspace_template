'use strict';
const API = '/api/v1/admin';
const NAV_ITEMS = [
  ['/admin/users', 'Users', 'users:read'],
  ['/admin/roles', 'Roles', 'roles:read'],
  ['/admin/permissions', 'Permissions', 'permissions:read'],
  ['/admin/audit-log', 'Audit log', 'audit_log:read'],
  ['/admin/system-settings', 'Settings', 'system_settings:read'],
  ['/admin/metrics', 'Metrics', 'metrics:read'],
  ['/admin/version', 'Version', null],
  ['/admin/swagger-ui', 'API', 'openapi:read'],
];
let authState = null;
function node(tag, text) {
  const element = document.createElement(tag);
  if (text !== undefined) element.textContent = text;
  return element;
}
function renderError(error) {
  const message = node('p', error instanceof Error ? error.message : String(error));
  message.className = 'error';
  document.getElementById('app').replaceChildren(message);
}
async function api(path, options) {
  const response = await fetch(path.startsWith('/api/') ? path : API + path, Object.assign({ credentials: 'same-origin' }, options));
  if (!response.ok) {
    let detail = 'HTTP ' + response.status;
    try {
      const body = await response.json();
      detail = body.message || body.error || detail;
    } catch (_error) {
      detail = detail;
    }
    throw new Error(detail);
  }
  if (response.status === 204) return null;
  const contentType = response.headers.get('content-type') || '';
  return contentType.includes('application/json') ? response.json() : response.text();
}
function hasPermission(permission) {
  return permission === null || (authState && authState.permissions.includes(permission));
}
function renderNavigation() {
  const navigation = document.getElementById('nav');
  navigation.replaceChildren();
  NAV_ITEMS.filter((item) => hasPermission(item[2])).forEach((item) => {
    const link = node('a', item[1]);
    link.href = item[0];
    navigation.appendChild(link);
  });
  const auth = document.getElementById('auth');
  auth.replaceChildren();
  const link = node('a', authState ? 'Sign out' : 'Sign in');
  link.href = authState ? '#' : '/admin/sign-in';
  if (authState) link.onclick = signOut;
  auth.appendChild(link);
}
async function loadAuth() {
  try {
    authState = await api('/auth/me');
  } catch (_error) {
    authState = null;
  }
  renderNavigation();
}
async function signOut(event) {
  event.preventDefault();
  await api('/auth/sign-out', { method: 'POST', headers: csrfHeaders() });
  authState = null;
  location.assign('/admin/sign-in');
}
function csrfHeaders() {
  const prefix = 'admin_csrf_token=';
  const token = document.cookie.split(';').map((cookie) => cookie.trim()).find((cookie) => cookie.startsWith(prefix));
  return token ? { 'X-CSRF-Token': decodeURIComponent(token.slice(prefix.length)) } : {};
}
function renderSignIn() {
  const form = node('form');
  const login = node('input');
  login.name = 'login';
  login.placeholder = 'Login';
  login.autocomplete = 'username';
  const password = node('input');
  password.name = 'password';
  password.type = 'password';
  password.placeholder = 'Password';
  password.autocomplete = 'current-password';
  const submit = node('button', 'Sign in');
  submit.type = 'submit';
  form.append(login, password, submit);
  form.onsubmit = async (event) => {
    event.preventDefault();
    try {
      authState = await api('/auth/sign-in', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ login: login.value, password: password.value }),
      });
      location.assign('/admin');
    } catch (error) {
      renderError(error);
    }
  };
  document.getElementById('app').replaceChildren(node('h1', 'Sign in'), form);
}
function renderHome() {
  const first = NAV_ITEMS.find((item) => item[0] !== '/admin/version' && hasPermission(item[2]));
  location.replace(first ? first[0] : '/admin/version');
}
function jsonHeaders() {
  return Object.assign({ 'Content-Type': 'application/json' }, csrfHeaders());
}
function actionButton(label, permission, action) {
  if (!hasPermission(permission)) return null;
  const button = node('button', label);
  button.type = 'button';
  button.onclick = async () => {
    try {
      await action();
      await renderCurrentPage();
    } catch (error) {
      renderError(error);
    }
  };
  return button;
}
function renderTable(title, columns, rows, actions) {
  const table = node('table');
  const header = node('tr');
  columns.forEach((column) => header.appendChild(node('th', column[0])));
  if (actions) header.appendChild(node('th', 'Actions'));
  const head = node('thead');
  head.appendChild(header);
  const body = node('tbody');
  rows.forEach((row) => {
    const line = node('tr');
    columns.forEach((column) => line.appendChild(node('td', String(row[column[1]] ?? ''))));
    if (actions) {
      const cell = node('td');
      actions(row).filter(Boolean).forEach((button) => cell.appendChild(button));
      line.appendChild(cell);
    }
    body.appendChild(line);
  });
  table.append(head, body);
  document.getElementById('app').replaceChildren(node('h1', title), table);
  return table;
}
function promptValue(label, current) {
  return window.prompt(label, current === undefined ? '' : current);
}
async function renderUsers() {
  const users = await api('/users');
  renderTable('Users', [['ID', 'id'], ['Login', 'login'], ['Display name', 'display_name'], ['Banned', 'is_banned']], users, (user) => [
    actionButton('Edit', 'users:update', async () => {
      const login = promptValue('Login', user.login);
      const displayName = promptValue('Display name', user.display_name);
      if (login !== null && displayName !== null) await api('/users/' + user.id, { method: 'PATCH', headers: jsonHeaders(), body: JSON.stringify({ login, display_name: displayName }) });
    }),
    actionButton(user.is_banned ? 'Unban' : 'Ban', 'users:update', () => api('/users/' + user.id + '/ban', { method: 'POST', headers: jsonHeaders(), body: JSON.stringify({ is_banned: !user.is_banned }) })),
    actionButton('Password', 'users:update', async () => {
      const password = promptValue('New password');
      if (password !== null) await api('/users/' + user.id + '/password', { method: 'POST', headers: jsonHeaders(), body: JSON.stringify({ password }) });
    }),
    actionButton('Roles', 'user_roles:update', async () => {
      const value = promptValue('Role IDs separated by commas');
      if (value !== null) await api('/users/' + user.id + '/roles', { method: 'PUT', headers: jsonHeaders(), body: JSON.stringify({ role_ids: value.split(',').filter(Boolean).map(Number) }) });
    }),
    actionButton('Delete', 'users:delete', async () => {
      if (window.confirm('Delete ' + user.login + '?')) await api('/users/' + user.id, { method: 'DELETE', headers: csrfHeaders() });
    }),
  ]);
  const create = actionButton('Create user', 'users:create', async () => {
    const login = promptValue('Login');
    const displayName = promptValue('Display name');
    const password = promptValue('Password');
    if (login !== null && displayName !== null && password !== null) await api('/users', { method: 'POST', headers: jsonHeaders(), body: JSON.stringify({ login, display_name: displayName, password }) });
  });
  if (create) document.getElementById('app').insertBefore(create, document.querySelector('table'));
}
async function renderRoles() {
  const roles = await api('/roles');
  renderTable('Roles', [['ID', 'id'], ['Name', 'name'], ['System', 'is_system']], roles, (role) => [
    actionButton('Edit', 'roles:update', async () => {
      const name = promptValue('Name', role.name);
      if (name !== null) await api('/roles/' + role.id, { method: 'PATCH', headers: jsonHeaders(), body: JSON.stringify({ name }) });
    }),
    actionButton('Permissions', 'role_permissions:update', async () => {
      const value = promptValue('Permission IDs separated by commas');
      if (value !== null) await api('/roles/' + role.id + '/permissions', { method: 'PUT', headers: jsonHeaders(), body: JSON.stringify({ permission_ids: value.split(',').filter(Boolean).map(Number) }) });
    }),
    actionButton('Delete', 'roles:delete', async () => {
      if (window.confirm('Delete ' + role.name + '?')) await api('/roles/' + role.id, { method: 'DELETE', headers: csrfHeaders() });
    }),
  ]);
  const create = actionButton('Create role', 'roles:create', async () => {
    const name = promptValue('Name');
    if (name !== null) await api('/roles', { method: 'POST', headers: jsonHeaders(), body: JSON.stringify({ name }) });
  });
  if (create) document.getElementById('app').insertBefore(create, document.querySelector('table'));
}
async function renderPermissions() {
  renderTable('Permissions', [['ID', 'id'], ['Name', 'name']], await api('/permissions'));
}
async function renderAudit() {
  renderTable('Audit log', [['Time', 'created_at'], ['User', 'user_id'], ['Action', 'action'], ['Resource', 'resource'], ['Result', 'result']], await api('/audit-log'));
}
async function renderSettings() {
  const settings = await api('/system-settings');
  const form = node('form');
  Object.keys(settings).forEach((key) => {
    const input = node('input');
    input.name = key;
    input.placeholder = key;
    input.value = settings[key] ?? '';
    form.appendChild(input);
  });
  const submit = node('button', 'Save');
  submit.disabled = !hasPermission('system_settings:update');
  form.appendChild(submit);
  form.onsubmit = async (event) => {
    event.preventDefault();
    const body = {};
    new FormData(form).forEach((value, key) => { body[key] = value; });
    await api('/system-settings', { method: 'PATCH', headers: jsonHeaders(), body: JSON.stringify(body) });
  };
  document.getElementById('app').replaceChildren(node('h1', 'Settings'), form);
}
async function renderMetrics() {
  const output = node('pre', await api('/metrics'));
  document.getElementById('app').replaceChildren(node('h1', 'Metrics'), output);
}
async function renderVersion() {
  const version = await api('/api/v1/git_info');
  document.getElementById('app').replaceChildren(node('h1', 'Version'), node('p', version.commit || 'Unknown version'));
}
async function renderCurrentPage() {
  const pages = {
    '/admin/users': renderUsers,
    '/admin/roles': renderRoles,
    '/admin/permissions': renderPermissions,
    '/admin/audit-log': renderAudit,
    '/admin/system-settings': renderSettings,
    '/admin/metrics': renderMetrics,
    '/admin/version': renderVersion,
  };
  const render = pages[location.pathname];
  if (render) await render();
  else renderHome();
}
async function route() {
  await loadAuth();
  if (location.pathname === '/admin/sign-in') {
    renderSignIn();
    return;
  }
  if (!authState) {
    location.assign('/admin/sign-in');
    return;
  }
  await renderCurrentPage();
}
if (typeof module !== 'undefined') module.exports = { api, csrfHeaders, hasPermission, setAuthStateForTest: (value) => { authState = value; } };
if (typeof window !== 'undefined') route().catch(renderError);
