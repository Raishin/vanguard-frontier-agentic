#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

const PLATFORM_CONFIG = {
  codex: {
    variants: [["codex", ".codex/agents", ".toml"]],
  },
  copilot: {
    variants: [["copilot", ".github/agents", ".agent.md"]],
  },
  "claude-code": {
    variants: [["claude-code", ".claude/agents", ".md"]],
  },
  cursor: {
    variants: [["cursor", ".cursor/agents", ".md"]],
  },
  gemini: {
    variants: [["gemini", ".gemini/agents", ".md"]],
  },
  "kiro-ide": {
    variants: [["kiro-ide", ".kiro/agents", ".md"]],
  },
  "kiro-cli": {
    variants: [["kiro-cli", ".kiro/agents", ".json"]],
  },
  kiro: {
    variants: [
      ["kiro-ide", ".kiro/agents", ".md"],
      ["kiro-cli", ".kiro/agents", ".json"],
    ],
  },
};

const PLATFORM_ALIASES = {
  claude: "claude-code",
  kiroide: "kiro-ide",
  kirocli: "kiro-cli",
};

function usage(exitCode = 0) {
  const message = `
Export selected marketplace agents into a consumer repository.

Usage:
  vfa-export-agents --platform <platform> --agents <agent-id[,agent-id...]> [--repo <path>] [--force]
  vfa-export-agents --platform <platform> --all [--repo <path>] [--force]
  vfa-export-agents --list

Platforms:
  codex, copilot, claude-code, cursor, gemini, kiro, kiro-ide, kiro-cli

Examples:
  vfa-export-agents --list
  vfa-export-agents --platform claude-code --agents azure-cosmosdb-platform-operator-agent
  vfa-export-agents --platform kiro --agents azure-cosmosdb-platform-operator-agent --repo ../consumer-repo
  vfa-export-agents --platform copilot --all --repo /path/to/project --force
`.trim();
  console[exitCode === 0 ? "log" : "error"](message);
  process.exit(exitCode);
}

function parseArgs(argv) {
  const args = {
    repo: process.cwd(),
    force: false,
    list: false,
    all: false,
    agents: [],
    platform: null,
  };

  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    if (arg === "--help" || arg === "-h") usage(0);
    if (arg === "--list") {
      args.list = true;
      continue;
    }
    if (arg === "--force") {
      args.force = true;
      continue;
    }
    if (arg === "--all") {
      args.all = true;
      continue;
    }
    if (arg === "--repo") {
      args.repo = path.resolve(argv[++i] ?? "");
      continue;
    }
    if (arg === "--platform") {
      args.platform = argv[++i] ?? "";
      continue;
    }
    if (arg === "--agents") {
      args.agents = (argv[++i] ?? "")
        .split(",")
        .map((value) => value.trim())
        .filter(Boolean);
      continue;
    }
    usage(1);
  }

  return args;
}

function walk(dir, matcher, results = []) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      walk(fullPath, matcher, results);
      continue;
    }
    if (matcher(fullPath)) results.push(fullPath);
  }
  return results;
}

function loadAgents() {
  const metadataPaths = walk(path.join(repoRoot, "agents"), (fullPath) =>
    fullPath.endsWith(`${path.sep}metadata.json`)
  );

  const agents = metadataPaths.map((metadataPath) => {
    const raw = fs.readFileSync(metadataPath, "utf8");
    const metadata = JSON.parse(raw);
    return {
      id: metadata.id,
      name: metadata.name,
      provider: metadata.provider,
      summary: metadata.summary,
      harness_variants: metadata.harness_variants ?? {},
      metadataPath,
    };
  });

  const byId = new Map(agents.map((agent) => [agent.id, agent]));
  return { agents, byId };
}

function normalizePlatform(platform) {
  const lowered = platform.toLowerCase();
  return PLATFORM_ALIASES[lowered] ?? lowered;
}

function ensurePlatform(platform) {
  if (!platform) usage(1);
  const normalized = normalizePlatform(platform);
  if (!PLATFORM_CONFIG[normalized]) {
    console.error(`Unsupported platform: ${platform}`);
    usage(1);
  }
  return normalized;
}

function assertWithin(parent, child, label) {
  const resolvedParent = path.resolve(parent);
  const resolvedChild = path.resolve(child);
  const sep = path.sep;
  const parentWithSep = resolvedParent.endsWith(sep) ? resolvedParent : resolvedParent + sep;
  if (resolvedChild !== resolvedParent && !resolvedChild.startsWith(parentWithSep)) {
    throw new Error(
      `Refusing to ${label}: path '${resolvedChild}' escapes '${resolvedParent}'. ` +
      `This indicates a malformed metadata.json or path traversal attempt.`
    );
  }
}

function copyFile(source, destination, force) {
  if (!force && fs.existsSync(destination)) {
    throw new Error(`Refusing to overwrite existing file without --force: ${destination}`);
  }
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.copyFileSync(source, destination);
}

function listAgents(agents) {
  for (const agent of agents.sort((a, b) => a.id.localeCompare(b.id))) {
    console.log(`${agent.id}\t${agent.provider}\t${agent.name}`);
  }
}

function buildDestinations(agent, platform) {
  const config = PLATFORM_CONFIG[platform];
  const destinations = [];

  for (const [variantKey, folder, extension] of config.variants) {
    const relativeSource = agent.harness_variants[variantKey];
    if (!relativeSource) {
      throw new Error(`Agent ${agent.id} does not have a ${variantKey} harness variant.`);
    }
    if (typeof relativeSource !== "string" || /[\\/]\.\.[\\/]|^\.\.[\\/]|[\\/]\.\.$|^\.\.$/.test(relativeSource) || path.isAbsolute(relativeSource)) {
      throw new Error(
        `Agent ${agent.id} ${variantKey} harness path '${relativeSource}' is invalid: ` +
        `must be a relative path within the repository, no '..' traversal, no absolute paths.`
      );
    }
    if (!/^[a-z0-9][a-z0-9-]*$/.test(agent.id)) {
      throw new Error(
        `Agent id '${agent.id}' fails schema pattern ^[a-z0-9][a-z0-9-]*$. ` +
        `Cannot derive a safe destination filename.`
      );
    }
    const source = path.join(repoRoot, relativeSource);
    assertWithin(repoRoot, source, "read source");
    destinations.push({
      variantKey,
      source,
      destRelative: path.join(folder, `${agent.id}${extension}`),
    });
  }

  return destinations;
}

function main() {
  const args = parseArgs(process.argv.slice(2));
  const { agents, byId } = loadAgents();

  if (args.list) {
    listAgents(agents);
    return;
  }

  const platform = ensurePlatform(args.platform);
  const selectedAgents = args.all
    ? agents
    : args.agents.map((agentId) => {
        const agent = byId.get(agentId);
        if (!agent) {
          throw new Error(`Unknown agent id: ${agentId}`);
        }
        return agent;
      });

  if (selectedAgents.length === 0) {
    throw new Error("No agents selected. Use --agents or --all.");
  }

  const operations = [];
  for (const agent of selectedAgents) {
    for (const destination of buildDestinations(agent, platform)) {
      operations.push({
        ...destination,
        dest: path.join(args.repo, destination.destRelative),
        agentId: agent.id,
      });
    }
  }

  for (const operation of operations) {
    assertWithin(args.repo, operation.dest, "write destination");
    copyFile(operation.source, operation.dest, args.force);
    console.log(
      `installed\t${operation.agentId}\t${operation.variantKey}\t${path.relative(args.repo, operation.dest)}`
    );
  }
}

try {
  main();
} catch (error) {
  console.error(error.message);
  process.exit(1);
}
