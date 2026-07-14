import { listen } from "@tauri-apps/api/event";
import { createMemo, createSignal, onCleanup, onMount } from "solid-js";
import type { MonitoringStatus } from "../../lib/api/types";
import { useT } from "../../lib/i18n";

function formatLastUpdated(
	lastUpdatedMs: number | null,
	now: number,
	t: ReturnType<typeof useT>,
): string {
	if (lastUpdatedMs === null) return t("monitoring.notYetUpdated");
	const elapsed = Math.max(0, now - lastUpdatedMs);
	const secs = Math.floor(elapsed / 1000);
	if (secs < 1) return t("monitoring.justNow");
	if (secs < 60) return t("monitoring.secondsAgo", { count: secs });
	return t("monitoring.minutesAgo", { count: Math.floor(secs / 60) });
}

export function useMonitoringStatus() {
	const t = useT();
	const [status, setStatus] = createSignal<MonitoringStatus>({
		active: true,
		last_updated_ms: null,
	});
	const [now, setNow] = createSignal(Date.now());

	onMount(() => {
		let unlisten: (() => void) | undefined;
		let disposed = false;

		const tick = setInterval(() => setNow(Date.now()), 1000);

		void listen<MonitoringStatus>("monitoring-status", (event) => {
			setStatus(event.payload);
		}).then((dispose) => {
			if (disposed) {
				void dispose();
				return;
			}
			unlisten = dispose;
		});

		onCleanup(() => {
			disposed = true;
			clearInterval(tick);
			void unlisten?.();
		});
	});

	const statusLabel = createMemo(() =>
		status().active ? t("monitoring.active") : t("monitoring.paused"),
	);

	const lastUpdatedLabel = createMemo(() =>
		formatLastUpdated(status().last_updated_ms, now(), t),
	);

	return { status, statusLabel, lastUpdatedLabel };
}
