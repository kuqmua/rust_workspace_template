const http = require('node:http');
const fs = require('node:fs');
const path = require('node:path');

const dist = path.resolve(__dirname, '..', 'dist');
const contentTypes = {
  '.css': 'text/css; charset=utf-8',
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.wasm': 'application/wasm',
};

http.createServer((request, response) => {
  const assetPrefix = '/admin/assets/';
  const relative = request.url.startsWith(assetPrefix)
    ? request.url.slice(assetPrefix.length)
    : 'index.html';
  const file = path.join(dist, relative);
  fs.readFile(file, (error, body) => {
    if (error) {
      response.writeHead(404).end('Not found');
      return;
    }
    response.writeHead(200, {
      'Content-Type': contentTypes[path.extname(file)] || 'application/octet-stream',
    });
    response.end(body);
  });
}).listen(8081, '127.0.0.1');
