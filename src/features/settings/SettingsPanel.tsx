import { useMutation, useQuery, useQueryClient } from "@tanstack/solid-query";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";
import { useT } from "../../lib/i18n";
import { createSignal, Show } from "solid-js";
import { Switch } from "../../components/ui/switch";
import { Input } from "../../components/ui/input";
import { Button } from "../../components/ui/button";
import { invokeResult } from "../../lib/api/invoke";
import type { AppSettings, PathStatus, UpdateInfo } from "../../lib/api/types";
import { pathKeys, settingsKeys } from "../../lib/query/keys";
import pkg from "../../../package.json";

export function SettingsPanel() {
	const t = useT();
	const queryClient = useQueryClient();
	const [message, setMessage] = createSignal<string | null>(null);
	const [pendingUpdate, setPendingUpdate] = createSignal<UpdateInfo | null>(
		null,
	);
	const [updateBusy, setUpdateBusy] = createSignal(false);

	const settingsQuery = useQuery(() => ({
		queryKey: settingsKeys.all,
		queryFn: async () => {
			const result = await invokeResult<AppSettings>("get_settings");
			if (result.isErr()) throw new Error(result.error);
			return result.value;
		},
	}));

	const pathQuery = useQuery(() => ({
		queryKey: pathKeys.all,
		queryFn: async () => {
			const result = await invokeResult<PathStatus>("path_status_cmd");
			if (result.isErr()) throw new Error(result.error);
			return result.value;
		},
	}));

	const saveMutation = useMutation(() => ({
		mutationFn: async (settings: AppSettings) => {
			const result = await invokeResult<void>("save_settings", { settings });
			if (result.isErr()) throw new Error(result.error);
			return settings;
		},
		onSuccess: () => {
			setMessage(t("settings.saved"));
			void queryClient.invalidateQueries({ queryKey: settingsKeys.all });
			setTimeout(() => setMessage(null), 2000);
		},
	}));

	const update = async (patch: Partial<AppSettings>) => {
		const current = settingsQuery.data;
		if (!current) return;
		const next = { ...current, ...patch };
		saveMutation.mutate(next);
	};

	const toggleAutostart = async (checked: boolean) => {
		if (checked) await enable();
		else await disable();
		const enabled = await isEnabled();
		await update({ autostart: enabled });
	};

	const checkForUpdates = async () => {
		setUpdateBusy(true);
		setMessage(t("settings.checkingForUpdates"));
		const result = await invokeResult<UpdateInfo | null>("check_for_updates");
		setUpdateBusy(false);
		if (result.isErr()) {
			setMessage(t("settings.updateFailed"));
			return;
		}
		if (!result.value) {
			setPendingUpdate(null);
			setMessage(t("settings.noUpdateAvailable"));
			setTimeout(() => setMessage(null), 3000);
			return;
		}
		setPendingUpdate(result.value);
		setMessage(t("settings.updateAvailable", { version: result.value.version }));
	};

	const installUpdate = async () => {
		setUpdateBusy(true);
		setMessage(t("settings.updateInstalling"));
		const result = await invokeResult<void>("install_update");
		setUpdateBusy(false);
		if (result.isErr()) {
			setMessage(t("settings.updateFailed"));
		}
	};

	const togglePath = async (checked: boolean) => {
		const cmd = checked ? "path_add" : "path_remove";
		const result = await invokeResult<void>(cmd);
		if (result.isErr()) {
			setMessage(result.error);
			return;
		}
		await update({ add_to_path: checked });
		void queryClient.invalidateQueries({ queryKey: pathKeys.all });
		setMessage(checked ? t("settings.pathAdded") : t("settings.pathRemoved"));
		setTimeout(() => setMessage(null), 3000);
	};

	return (
		<div class="space-y-4 p-4 text-sm">
			<Show when={message()}>
				<div class="rounded-md border border-zinc-700 bg-zinc-900 px-3 py-2 text-xs">
					{message()}
				</div>
			</Show>

			<SettingRow label={t("settings.autostart")}>
				<Switch
					checked={settingsQuery.data?.autostart ?? false}
					onChange={(checked) => void toggleAutostart(checked)}
				/>
			</SettingRow>

			<SettingRow label={t("settings.path")} hint={t("settings.pathHint")}>
				<Switch
					checked={
						pathQuery.data?.in_path ?? settingsQuery.data?.add_to_path ?? false
					}
					onChange={(checked) => void togglePath(checked)}
				/>
			</SettingRow>

			<SettingRow label={t("settings.startMinimized")}>
				<Switch
					checked={settingsQuery.data?.start_minimized ?? true}
					onChange={(checked) => void update({ start_minimized: checked })}
				/>
			</SettingRow>

			<SettingRow label={t("settings.autoCheckUpdates")}>
				<Switch
					checked={settingsQuery.data?.auto_check_updates ?? true}
					onChange={(checked) => void update({ auto_check_updates: checked })}
				/>
			</SettingRow>

			<div class="space-y-2 rounded-md border border-zinc-800 p-3">
				<p class="text-xs text-zinc-500">
					{t("settings.version", { version: pkg.version })}
				</p>
				<div class="flex flex-wrap gap-2">
					<Button
						variant="outline"
						size="sm"
						disabled={updateBusy()}
						onClick={() => void checkForUpdates()}
					>
						{t("settings.checkForUpdates")}
					</Button>
					<Show when={pendingUpdate()}>
						<Button
							size="sm"
							disabled={updateBusy()}
							onClick={() => void installUpdate()}
						>
							{t("settings.installUpdate")}
						</Button>
					</Show>
				</div>
			</div>

			<SettingRow label={t("settings.refreshInterval")}>
				<Input
					type="number"
					min={1}
					class="w-24"
					value={String(settingsQuery.data?.refresh_interval_secs ?? 5)}
					onChange={(e) =>
						void update({
							refresh_interval_secs: Number(e.currentTarget.value) || 5,
						})
					}
				/>
			</SettingRow>

			<Show when={pathQuery.data?.install_dir}>
				<p class="text-xs text-zinc-500">
					Install dir: {pathQuery.data?.install_dir}
				</p>
			</Show>
		</div>
	);
}

function SettingRow(props: {
	label: string;
	hint?: string;
	children: import("solid-js").JSX.Element;
}) {
	return (
		<div class="flex items-center justify-between gap-4">
			<div>
				<div class="text-zinc-100">{props.label}</div>
				<Show when={props.hint}>
					<div class="text-xs text-zinc-500">{props.hint}</div>
				</Show>
			</div>
			{props.children}
		</div>
	);
}
