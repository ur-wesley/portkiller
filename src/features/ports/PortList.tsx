import { getCurrentWindow } from "@tauri-apps/api/window";
import { createMemo, createSignal, onCleanup, onMount, Show } from "solid-js";
import { useTabs } from "../../components/ui/tabs";
import { useMutation, useQuery, useQueryClient } from "@tanstack/solid-query";
import { AlertDialog } from "../../components/ui/alert-dialog";
import { Card, CardContent } from "../../components/ui/card";
import { ScrollArea } from "../../components/ui/scroll-area";
import { invokeResult } from "../../lib/api/invoke";
import { mapKillError } from "../../lib/errors";
import type { PortInfo } from "../../lib/api/types";
import { useT } from "../../lib/i18n";
import { portKeys, settingsKeys } from "../../lib/query/keys";
import { filterPorts } from "./filter";
import { PortRow } from "./PortRow";
import { SearchBar } from "./SearchBar";
import { usePortsQuery } from "./usePortsQuery";

export function PortList(props: { favoritesOnly?: boolean }) {
	const t = useT();
	const tabs = useTabs();
	let searchInput: HTMLInputElement | undefined;

	const isActiveTab = () =>
		props.favoritesOnly
			? tabs.value() === "favorites"
			: tabs.value() === "ports";

	const focusSearch = () => {
		if (isActiveTab()) searchInput?.focus();
	};

	onMount(async () => {
		const window = getCurrentWindow();
		if (await window.isFocused()) focusSearch();

		const unlisten = await window.onFocusChanged(({ payload: focused }) => {
			if (focused) focusSearch();
		});
		onCleanup(unlisten);
	});
	const queryClient = useQueryClient();
	const portsQuery = usePortsQuery();
	const settingsQuery = useQuery(() => ({
		queryKey: settingsKeys.all,
		queryFn: async () => {
			const result =
				await invokeResult<import("../../lib/api/types").AppSettings>(
					"get_settings",
				);
			if (result.isErr()) throw new Error(result.error);
			return result.value;
		},
	}));

	const [search, setSearch] = createSignal("");
	const [pendingKill, setPendingKill] = createSignal<PortInfo | null>(null);
	const [toast, setToast] = createSignal<string | null>(null);

	const filtered = createMemo(() =>
		filterPorts(
			portsQuery.data ?? [],
			search(),
			props.favoritesOnly ?? false,
			settingsQuery.data?.favorites ?? [],
		),
	);

	const killMutation = useMutation(() => ({
		mutationFn: async (port: PortInfo) => {
			const result = await invokeResult<number>("kill_port_cmd", {
				port: port.port,
				force: false,
			});
			return result;
		},
		onSuccess: (result) => {
			if (result.isOk()) {
				setToast(t("ports.killed"));
				void queryClient.invalidateQueries({ queryKey: portKeys.all });
			} else {
				setToast(
					mapKillError(result.error, {
						denied: t("ports.needAdmin"),
						failed: t("ports.killFailed"),
					}),
				);
			}
			setPendingKill(null);
			setTimeout(() => setToast(null), 2500);
		},
	}));

	const favoriteMutation = useMutation(() => ({
		mutationFn: async (port: number) => {
			const result = await invokeResult<number[]>("toggle_favorite", { port });
			if (result.isErr()) throw new Error(result.error);
			return result.value;
		},
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: settingsKeys.all });
		},
	}));

	const pending = () => pendingKill();

	return (
		<div class="flex h-full min-h-0 flex-col gap-3 p-3">
			<SearchBar
				ref={(el) => {
					searchInput = el;
				}}
				value={search()}
				placeholder={t("ports.search")}
				onInput={setSearch}
			/>
			<Show when={toast()}>
				<div class="rounded-md border border-zinc-700 bg-zinc-900 px-3 py-2 text-xs text-zinc-200">
					{toast()}
				</div>
			</Show>
			<ScrollArea class="min-h-0 flex-1 rounded-md border border-zinc-800">
				<Show
					when={filtered().length > 0}
					fallback={
						<Card class="border-0">
							<CardContent class="text-sm text-zinc-500">
								{t("ports.empty")}
							</CardContent>
						</Card>
					}
				>
					{filtered().map((port) => (
						<PortRow
							port={port}
							isFavorite={(settingsQuery.data?.favorites ?? []).includes(
								port.port,
							)}
							killLabel={t("ports.kill")}
							onKill={setPendingKill}
							onToggleFavorite={(p) => favoriteMutation.mutate(p)}
						/>
					))}
				</Show>
			</ScrollArea>
			<AlertDialog
				open={pending() !== null}
				title={t("ports.confirmTitle")}
				description={`Stop the process on port ${pending()?.port ?? ""}?`}
				confirmLabel={t("ports.kill")}
				cancelLabel={t("ports.cancel")}
				onCancel={() => setPendingKill(null)}
				onConfirm={() => {
					const target = pending();
					if (target) killMutation.mutate(target);
				}}
			/>
		</div>
	);
}
