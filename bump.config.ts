import { defineConfig } from "bumpp";

export default defineConfig({
	files: ["package.json", "Cargo.toml", "src-tauri/tauri.conf.json"],
	commit: "chore: release v%s",
	tag: "v%s",
	push: true,
});
