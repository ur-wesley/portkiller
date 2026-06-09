import { isTauri } from "@tauri-apps/api/core";
import { onMount } from "solid-js";
import { invokeResult } from "../../lib/api/invoke";
import type { AppSettings, UpdateInfo } from "../../lib/api/types";

export function useAutoUpdateCheck(
	onUpdateAvailable: (update: UpdateInfo) => void,
) {
	onMount(() => {
		if (!isTauri()) return;

		void (async () => {
			const settings = await invokeResult<AppSettings>("get_settings");
			if (settings.isErr() || !settings.value.auto_check_updates) return;

			const update = await invokeResult<UpdateInfo | null>(
				"check_for_updates",
			);
			if (update.isOk() && update.value) {
				onUpdateAvailable(update.value);
			}
		})();
	});
}
