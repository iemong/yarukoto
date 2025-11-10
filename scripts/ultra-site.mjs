import { spawn } from "node:child_process";
import { platform } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const args = process.argv.slice(2);
const shouldApply = args.includes("--apply");
const passthrough = args.filter((arg) => arg !== "--apply");
const biomeArgs = ["check", ".", ...passthrough];

if (shouldApply) {
	biomeArgs.push("--write");
}

const cwd = dirname(fileURLToPath(import.meta.url));
const binDir = join(cwd, "..", "node_modules", ".bin");
const binName = platform() === "win32" ? "biome.cmd" : "biome";
const biomeBin = join(binDir, binName);

const child = spawn(biomeBin, biomeArgs, {
	stdio: "inherit",
	cwd: join(cwd, ".."),
	env: process.env,
});

child.on("exit", (code) => {
	process.exit(code ?? 0);
});
