import { createSignal, Show } from "solid-js";
import { useT } from "../lib/i18n";
import {
	Tabs,
	TabsContent,
	TabsList,
	TabsTrigger,
} from "../components/ui/tabs";
import { Button } from "../components/ui/button";
import { FavoritesTab } from "../features/favorites/FavoritesTab";
import { PortList } from "../features/ports/PortList";
import { SettingsPanel } from "../features/settings/SettingsPanel";
import { useAutoUpdateCheck } from "../features/updates/useAutoUpdateCheck";
import { invokeResult } from "../lib/api/invoke";
import type { UpdateInfo } from "../lib/api/types";

export default function App() {
	const t = useT();
	const [availableUpdate, setAvailableUpdate] =
		createSignal<UpdateInfo | null>(null);

	useAutoUpdateCheck(setAvailableUpdate);

	const installUpdate = async () => {
		await invokeResult<void>("install_update");
	};

	return (
		<div class="flex h-full min-h-0 flex-col bg-zinc-950 text-zinc-100">
			<Show when={availableUpdate()}>
				{(update) => (
					<div class="flex items-center justify-between gap-2 border-b border-emerald-900/50 bg-emerald-950/40 px-4 py-2 text-xs">
						<span>
							{t("settings.updateAvailable", { version: update.version })}
						</span>
						<Button size="sm" onClick={() => void installUpdate()}>
							{t("settings.installUpdate")}
						</Button>
					</div>
				)}
			</Show>
			<header
				class="border-b border-zinc-800 px-4 py-3"
				data-tauri-drag-region
			>
				<h1 class="text-sm font-semibold tracking-wide">{t("app.title")}</h1>
			</header>
			<Tabs defaultValue="ports" class="min-h-0 flex-1">
				<TabsList>
					<TabsTrigger value="ports">{t("ports.all")}</TabsTrigger>
					<TabsTrigger value="favorites">{t("ports.favorites")}</TabsTrigger>
					<TabsTrigger value="settings">{t("settings.title")}</TabsTrigger>
				</TabsList>
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
