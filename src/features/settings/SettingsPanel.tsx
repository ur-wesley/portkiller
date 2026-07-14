import { useMutation, useQuery, useQueryClient } from "@tanstack/solid-query";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";
import { useT } from "../../lib/i18n";
import { createSignal, Show } from "solid-js";
import { Switch } from "../../components/ui/switch";
import { Input } from "../../components/ui/input";
import { Button } from "../../components/ui/button";
import { invokeResult } from "../../lib/api/invoke";
import type { AppSettings, UpdateInfo } from "../../lib/api/types";
import { settingsKeys } from "../../lib/query/keys";
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

	return (
		<div class="space-y-4 p-4 text-sm">
			<Show when={message()}>
				<div class="glass-surface rounded-md px-3 py-2 text-xs">
					{message()}
				</div>
			</Show>

			<SettingRow label={t("settings.autostart")}>
				<Switch
					checked={settingsQuery.data?.autostart ?? false}
					onChange={(checked) => void toggleAutostart(checked)}
				/>
			</SettingRow>

			<SettingRow
				label={t("settings.monitoringEnabled")}
				hint={t("settings.monitoringEnabledHint")}
			>
				<Switch
					checked={settingsQuery.data?.monitoring_enabled ?? true}
					onChange={(checked) => void update({ monitoring_enabled: checked })}
				/>
			</SettingRow>

			<SettingRow
				label={t("settings.closeToTray")}
				hint={t("settings.closeToTrayHint")}
			>
				<Switch
					checked={settingsQuery.data?.close_to_tray ?? true}
					onChange={(checked) => void update({ close_to_tray: checked })}
				/>
			</SettingRow>

			<SettingRow
				label={t("settings.startMinimized")}
				hint={t("settings.startMinimizedHint")}
			>
				<Switch
					checked={settingsQuery.data?.start_minimized ?? false}
					onChange={(checked) => void update({ start_minimized: checked })}
				/>
			</SettingRow>

			<SettingRow label={t("settings.autoCheckUpdates")}>
				<Switch
					checked={settingsQuery.data?.auto_check_updates ?? true}
					onChange={(checked) => void update({ auto_check_updates: checked })}
				/>
			</SettingRow>

			<div class="glass-surface space-y-2 rounded-md p-3">
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
