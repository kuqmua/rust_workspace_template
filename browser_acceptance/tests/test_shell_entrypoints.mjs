import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repositoryDirectory = fileURLToPath(new URL("../../", import.meta.url));
const linuxFixture = { skip: process.platform !== "linux" && "requires Linux Bash fixtures" };

function shellEntrypointFixture(context, source, caseIndex = 0) {
  const root = join(tmpdir(), `workspace-shell-tests-${process.pid}-${context.name}-${caseIndex}`);
  mkdirSync(root, { mode: 0o700 });
  context.after(() => rmSync(root, { recursive: true }));
  const executableDirectory = join(root, "bin");
  const scriptDirectory = join(root, dirname(source));
  [executableDirectory, scriptDirectory, join(root, "target"), join(root, "server")]
    .forEach(directory => mkdirSync(directory, { recursive: true }));
  const script = join(scriptDirectory, basename(source));
  copyFileSync(join(repositoryDirectory, source), script);
  const log = join(root, "calls.jsonl");
  const environmentKeys = [
    "CARGO_PROFILE_RELEASE_LTO", "CARGO_PROFILE_RELEASE_STRIP", "DATABASE_URL",
    "ADMIN_SESSION_LIMIT", "ADMIN_SWAGGER_ENABLED", "ADMIN_COOKIE_SECURE",
    "SOURCE_PLACE_TYPE", "SVC_MODE", "SERVICE_SOCKET_ADDRESS",
  ];
  const executable = `#!${process.execPath}
import { appendFileSync, statSync } from "node:fs";
import { basename } from "node:path";
const command = basename(process.argv[1]);
const environment = Object.fromEntries(${JSON.stringify(environmentKeys)}.map(key => [key, process.env[key] ?? null]));
const passwordFileMode = command === "cargo" && process.argv.includes("administrator") ? statSync(process.argv.at(-1)).mode & 0o777 : null;
appendFileSync(process.env.FIXTURE_LOG, JSON.stringify({ command, args: process.argv.slice(2), cwd: process.cwd(), environment, passwordFileMode }) + "\\n");
const exitVariable = command === "cargo" ? (process.argv.includes("server") ? "FIXTURE_SERVER_EXIT" : "FIXTURE_CARGO_EXIT") : "FIXTURE_PSQL_EXIT";
process.exit(Number(process.env[exitVariable] ?? 0));
`;
  ["cargo", "psql"].forEach(command => {
    writeFileSync(join(executableDirectory, command), executable, { mode: 0o700, flag: "wx" });
  });
  const environment = { PATH: `${executableDirectory}:/usr/bin:/bin`, FIXTURE_LOG: log };
  return {
    root,
    run(args = [], extraEnvironment = {}) {
      const result = spawnSync("/usr/bin/bash", [script, ...args], {
        cwd: root,
        env: { ...environment, ...extraEnvironment },
        encoding: "utf8",
      });
      assert.equal(result.error, undefined);
      assert.equal(result.signal, null);
      return {
        ...result,
        calls: existsSync(log) ? readFileSync(log, "utf8").trim().split("\n").map(line => JSON.parse(line)) : [],
      };
    },
  };
}

test("test_manifest_shell_rejects_invalid_arguments_before_cargo", linuxFixture, context => {
  [[], ["missing.yaml"], ["."], ["first", "second"]].forEach((args, caseIndex) => {
    const fixture = shellEntrypointFixture(context, "deploy/validate-production-manifest.sh", caseIndex);
    const result = fixture.run(args);
    assert.equal(result.status, 2);
    assert.match(result.stderr, /usage: validate-production-manifest/);
    assert.deepEqual(result.calls, []);
  });
});

test("test_manifest_shell_preserves_path_and_normalizes_cargo_exit_status", linuxFixture, context => {
  [0, 7].forEach((status, caseIndex) => {
    const fixture = shellEntrypointFixture(context, "deploy/validate-production-manifest.sh", caseIndex);
    const manifest = join(fixture.root, "rendered manifest.yaml");
    writeFileSync(manifest, "fixture\n", { flag: "wx" });
    const result = fixture.run([manifest], { FIXTURE_CARGO_EXIT: String(status) });
    assert.equal(result.status, status === 0 ? 0 : 1);
    assert.equal(result.calls.length, 1);
    assert.deepEqual(result.calls[0].args, [
      "run", "--quiet", "--locked", "--manifest-path", join(fixture.root, "Cargo.toml"),
      "--package", "workspace_scaffold", "--", "manifest", manifest,
    ]);
  });
});

test("test_fuzz_shell_preserves_arguments_overrides_profiles_and_forwards_failure", linuxFixture, context => {
  [0, 7].forEach((status, caseIndex) => {
    const fixture = shellEntrypointFixture(context, "fuzz/run_domain.sh", caseIndex);
    const args = status === 0 ? [] : ["--runs=10", "path with spaces", "", "--", "-value"];
    const result = fixture.run(args, {
      CARGO_PROFILE_RELEASE_LTO: "true", CARGO_PROFILE_RELEASE_STRIP: "debuginfo",
      FIXTURE_CARGO_EXIT: String(status),
    });
    assert.equal(result.status, status);
    assert.equal(result.calls.length, 1);
    assert.deepEqual(result.calls[0].args, ["fuzz", "run", "domain_boundaries", "--", ...args]);
    assert.equal(result.calls[0].environment.CARGO_PROFILE_RELEASE_LTO, "false");
    assert.equal(result.calls[0].environment.CARGO_PROFILE_RELEASE_STRIP, "none");
  });
});

test("test_browser_server_shell_rejects_missing_or_non_disposable_database", linuxFixture, context => {
  [undefined, "", "postgres://fixture/production", "postgres://fixture/example_browser_test_extra"]
    .forEach((database, caseIndex) => {
      const fixture = shellEntrypointFixture(context, "browser_acceptance/run-server.sh", caseIndex);
      const result = fixture.run([], database === undefined ? {} : { BROWSER_ACCEPTANCE_DATABASE_URL: database });
      assert.equal(result.status, database ? 2 : 1);
      assert.deepEqual(result.calls, []);
      assert.equal(existsSync(join(fixture.root, "target/browser_acceptance_admin_password")), false);
    });
});

test("test_browser_server_shell_accepts_disposable_urls_and_forwards_configuration", linuxFixture, context => {
  [
    { database: "postgres://fixture/example_browser_test", session: "64", swagger: "false" },
    { database: "postgres://fixture/example_browser_test?sslmode=disable", session: "9", swagger: "true" },
  ].forEach(({ database, session, swagger }, caseIndex) => {
    const fixture = shellEntrypointFixture(context, "browser_acceptance/run-server.sh", caseIndex);
    const result = fixture.run([], {
      BROWSER_ACCEPTANCE_DATABASE_URL: database,
      ...(session === "9" ? { BROWSER_ACCEPTANCE_SESSION_LIMIT: session, BROWSER_ACCEPTANCE_SWAGGER_ENABLED: swagger } : {}),
    });
    assert.equal(result.status, 0);
    assert.deepEqual(result.calls.map(call => call.command), ["psql", "cargo", "cargo"]);
    assert.deepEqual(result.calls[0].args, [database, "--set", "ON_ERROR_STOP=1", "--command", "DROP SCHEMA IF EXISTS public CASCADE", "--command", "CREATE SCHEMA public"]);
    assert.deepEqual(result.calls[1].args, ["run", "--package", "administrator_account_initialization_and_password_reset", "--", "administrator", "Initial Administrator", join(fixture.root, "target/browser_acceptance_admin_password")]);
    assert.equal(result.calls[1].passwordFileMode, 0o600);
    assert.deepEqual(result.calls[2].args, ["run", "--package", "server"]);
    assert.equal(result.calls[2].cwd, join(fixture.root, "server"));
    assert.equal(result.calls[2].environment.DATABASE_URL, database);
    assert.equal(result.calls[2].environment.ADMIN_SESSION_LIMIT, session);
    assert.equal(result.calls[2].environment.ADMIN_SWAGGER_ENABLED, swagger);
    assert.equal(result.calls[2].environment.ADMIN_COOKIE_SECURE, "false");
    assert.equal(result.calls[2].environment.SOURCE_PLACE_TYPE, "src");
    assert.equal(result.calls[2].environment.SVC_MODE, "serve");
    assert.equal(result.calls[2].environment.SERVICE_SOCKET_ADDRESS, "127.0.0.1:18080");
    assert.equal(existsSync(join(fixture.root, "target/browser_acceptance_admin_password")), false);
  });
});

test("test_browser_server_shell_stops_before_cargo_when_schema_reset_fails", linuxFixture, context => {
  const fixture = shellEntrypointFixture(context, "browser_acceptance/run-server.sh");
  const result = fixture.run([], { BROWSER_ACCEPTANCE_DATABASE_URL: "postgres://fixture/example_browser_test", FIXTURE_PSQL_EXIT: "7" });
  assert.equal(result.status, 7);
  assert.deepEqual(result.calls.map(call => call.command), ["psql"]);
  assert.equal(existsSync(join(fixture.root, "target/browser_acceptance_admin_password")), false);
});

test("test_browser_server_shell_stops_before_server_when_administrator_command_fails", linuxFixture, context => {
  const fixture = shellEntrypointFixture(context, "browser_acceptance/run-server.sh");
  const result = fixture.run([], { BROWSER_ACCEPTANCE_DATABASE_URL: "postgres://fixture/example_browser_test", FIXTURE_CARGO_EXIT: "7" });
  assert.equal(result.status, 7);
  assert.deepEqual(result.calls.map(call => call.command), ["psql", "cargo"]);
  assert.equal(result.calls[1].passwordFileMode, 0o600);
  assert.equal(existsSync(join(fixture.root, "target/browser_acceptance_admin_password")), true);
});

test("test_browser_server_shell_forwards_final_server_failure_after_password_cleanup", linuxFixture, context => {
  const fixture = shellEntrypointFixture(context, "browser_acceptance/run-server.sh");
  const result = fixture.run([], { BROWSER_ACCEPTANCE_DATABASE_URL: "postgres://fixture/example_browser_test", FIXTURE_SERVER_EXIT: "7" });
  assert.equal(result.status, 7);
  assert.deepEqual(result.calls.map(call => call.command), ["psql", "cargo", "cargo"]);
  assert.deepEqual(result.calls[2].args, ["run", "--package", "server"]);
  assert.equal(existsSync(join(fixture.root, "target/browser_acceptance_admin_password")), false);
});
