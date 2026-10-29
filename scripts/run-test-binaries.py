import json, pathlib, subprocess
root = pathlib.Path(__file__).resolve().parent.parent
for file in ("workspace.jsonl", "mdns.jsonl"):
    binaries = {p["executable"] for line in (root/"test-results"/file).read_text().splitlines()
                if (p := json.loads(line)).get("executable") and p.get("profile", {}).get("test")}
    if not binaries:
        raise SystemExit(f"No test binaries found in {file}")
    for exe in sorted(binaries):
        subprocess.run(["sudo", "unshare", "--net", "bash", str(root/"scripts/test-isolated.sh"), exe, "all"], check=True)
