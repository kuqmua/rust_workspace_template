'use strict';
async function loadOpenApi() {
  const response = await fetch('/api/v1/admin/openapi.json', { credentials: 'same-origin' });
  if (response.status === 401) {
    location.assign('/admin/sign-in');
    return;
  }
  if (!response.ok) throw new Error('OpenAPI request failed with HTTP ' + response.status);
  const documentValue = await response.json();
  document.getElementById('openapi').textContent = JSON.stringify(documentValue, null, 2);
}
loadOpenApi().catch((error) => {
  document.getElementById('openapi').textContent = error instanceof Error ? error.message : String(error);
});
