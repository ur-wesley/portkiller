import { invoke } from "@tauri-apps/api/core";
import { err, ok, type Result } from "neverthrow";

export async function invokeResult<T>(
	cmd: string,
	args?: Record<string, unknown>,
): Promise<Result<T, string>> {
	try {
		return ok(await invoke<T>(cmd, args));
	} catch (e) {
		return err(String(e));
	}
}
