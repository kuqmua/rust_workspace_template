'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
global.document = { cookie: '' };
const app = require('./app.js');
test('permission gating follows authenticated permissions', () => {
  app.setAuthStateForTest({ permissions: ['users:read'] });
  assert.equal(app.hasPermission('users:read'), true);
  assert.equal(app.hasPermission('roles:read'), false);
  assert.equal(app.hasPermission(null), true);
});
test('csrf header is decoded from the dedicated cookie', () => {
  global.document.cookie = 'other=value; admin_csrf_token=hello%20token';
  assert.deepEqual(app.csrfHeaders(), { 'X-CSRF-Token': 'hello token' });
});
test('backend error code is surfaced to the page', async () => {
  global.fetch = async () => ({ ok: false, status: 403, json: async () => ({ error: 'forbidden' }) });
  await assert.rejects(app.api('/users'), /forbidden/);
});
