#!/usr/bin/env bash
set -euo pipefail

# Repo root: .../.agents/skills/init-agent/scripts -> ../../../../
REPO_ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"

mkdir -p "${REPO_ROOT}/.agents/skills" "${REPO_ROOT}/.agents/commands" "${REPO_ROOT}/.agents/rules"

link_one() {
  local tool="$1"
  local name="$2"
  local target_rel="../.agents/${name}"
  local parent="${REPO_ROOT}/${tool}"
  local linkpath="${parent}/${name}"

  mkdir -p "${parent}"

  if [[ -L "${linkpath}" ]]; then
    local current
    current="$(readlink "${linkpath}")"
    if [[ "${current}" == "${target_rel}" ]]; then
      return 0
    fi
    echo "init-agent: wrong symlink target at ${linkpath} (got ${current}, expected ${target_rel})" >&2
    echo "Delete the symlink and re-run." >&2
    return 1
  fi

  if [[ -e "${linkpath}" ]]; then
    echo "init-agent: exists and is not a symlink: ${linkpath}" >&2
    echo "Remove or rename it, then re-run." >&2
    return 1
  fi

  ln -s "${target_rel}" "${linkpath}"
}

for tool in .cursor .claude .codex; do
  for name in skills commands rules; do
    link_one "${tool}" "${name}"
  done
done

if [[ ! -e "${REPO_ROOT}/CLAUDE.md" ]]; then
  (cd "${REPO_ROOT}" && ln -s AGENTS.md CLAUDE.md)
fi

echo "init-agent: done (symlinks under .cursor, .claude, .codex; CLAUDE.md if absent)."
