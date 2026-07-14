import { createSignal, Show } from "solid-js";
import { useT } from "../lib/i18n";
import {
	Tabs,
	TabsContent,
	TabsList,
	TabsTrigger,
} from "../components/ui/tabs";
import { Button } from "../components/ui/button";
import { DashboardTab } from "../features/dashboard/DashboardTab";
import { FavoritesTab } from "../features/favorites/FavoritesTab";
import { PortList } from "../features/ports/PortList";
import { useMonitoringStatus } from "../features/ports/useMonitoringStatus";
import { usePortsQuery } from "../features/ports/usePortsQuery";
import { SettingsPanel } from "../features/settings/SettingsPanel";
import { useAutoUpdateCheck } from "../features/updates/useAutoUpdateCheck";
import { invokeResult } from "../lib/api/invoke";
import type { UpdateInfo } from "../lib/api/types";

export default function App() {
	const t = useT();
	const [availableUpdate, setAvailableUpdate] = createSignal<UpdateInfo | null>(
		null,
	);
	const { status, statusLabel, lastUpdatedLabel } = useMonitoringStatus();
	const portsQuery = usePortsQuery();

	useAutoUpdateCheck(setAvailableUpdate);

	const installUpdate = async () => {
		await invokeResult<void>("install_update");
	};

	return (
		<div class="flex h-full min-h-0 flex-col bg-zinc-950/20 text-zinc-100">
			<Show when={availableUpdate()}>
				{(update) => (
					<div class="glass-surface-subtle flex items-center justify-between gap-2 border-b border-emerald-500/20 px-4 py-2 text-xs">
						<span>
							{t("settings.updateAvailable", { version: update.version })}
						</span>
						<Button size="sm" onClick={() => void installUpdate()}>
							{t("settings.installUpdate")}
						</Button>
					</div>
				)}
			</Show>
			<div class="glass-surface-subtle flex items-center justify-between gap-2 border-b border-white/5 px-4 py-2">
				<div class="flex min-w-0 items-center gap-2 text-xs text-zinc-400">
					<span
						class={`size-2 shrink-0 rounded-full ${
							status().active ? "bg-emerald-500" : "bg-zinc-600"
						}`}
						aria-hidden="true"
					/>
					<span class="truncate">
						{statusLabel()} · {lastUpdatedLabel()}
					</span>
				</div>
				<Button
					variant="ghost"
					size="sm"
					disabled={portsQuery.isFetching}
					onClick={() => void portsQuery.refetch()}
				>
					{portsQuery.isFetching
						? t("dashboard.refreshing")
						: t("monitoring.refresh")}
				</Button>
			</div>
			<Tabs defaultValue="dashboard" class="min-h-0 flex-1">
				<div class="box-border w-full px-3 pt-3">
					<TabsList class="grid grid-cols-4">
						<TabsTrigger value="dashboard">{t("dashboard.title")}</TabsTrigger>
						<TabsTrigger value="ports">{t("ports.all")}</TabsTrigger>
						<TabsTrigger value="favorites">{t("ports.favorites")}</TabsTrigger>
						<TabsTrigger value="settings">{t("settings.title")}</TabsTrigger>
					</TabsList>
				</div>
				<TabsContent value="dashboard">
					<DashboardTab />
				</TabsContent>
				<TabsContent value="ports">
					<PortList />
				</TabsContent>
				<TabsContent value="favorites">
					<FavoritesTab />
				</TabsContent>
				<TabsContent value="settings">
					<SettingsPanel />
				</TabsContent>
			</Tabs>
		</div>
	);
}
