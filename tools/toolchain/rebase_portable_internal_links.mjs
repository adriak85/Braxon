#!/usr/bin/env node
import { existsSync, lstatSync, readlinkSync, symlinkSync, unlinkSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const root = resolve(process.argv[2] ?? process.cwd());
const prefix = "/data/data/com.termux/files/home/Braxon/";
const listed = spawnSync("git", ["-C", root, "ls-files", "-z"], { encoding: "buffer", maxBuffer: 64 * 1024 * 1024 });
if (listed.status !== 0) throw new Error(`git ls-files failed with ${listed.status}`);
const paths = listed.stdout.toString("utf8").split("\0").filter(Boolean);
const repaired = [];
const unresolved = [];
for (const path of paths) {
  const absolute = resolve(root, path);
  let stat;
  try {
    stat = lstatSync(absolute);
  } catch {
    continue;
  }
  if (!stat.isSymbolicLink()) continue;
  const target = readlinkSync(absolute);
  if (!target.startsWith(prefix)) {
    if (!existsSync(absolute)) unresolved.push({ path, target, reason: "external_or_missing_nonrebasable_target" });
    continue;
  }
  const internal = resolve(root, target.slice(prefix.length));
  if (!existsSync(internal)) {
    unresolved.push({ path, target, reason: "repository_relative_target_absent" });
    continue;
  }
  const portableTarget = relative(dirname(absolute), internal) || ".";
  unlinkSync(absolute);
  symlinkSync(portableTarget, absolute);
  repaired.push({ path, old_target: target, new_target: portableTarget });
}
console.log(JSON.stringify({ repaired, unresolved }, null, 2));
