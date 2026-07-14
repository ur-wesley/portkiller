import { getCurrentWindow } from "@tauri-apps/api/window";
import { createMemo, createSignal, onCleanup, onMount, Show } from "solid-js";
import { useTabs } from "../../components/ui/tabs";
import { useMutation, useQuery, useQueryClient } from "@tanstack/solid-query";
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

const KILL_ARM_MS = 3000;

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

	onMount(() => {
		let unlisten: (() => void) | undefined;
		let disposed = false;

		void (async () => {
			const window = getCurrentWindow();
			if (await window.isFocused()) focusSearch();

			const stop = await window.onFocusChanged(({ payload: focused }) => {
				if (focused) focusSearch();
			});

			if (disposed) {
				stop();
				return;
			}
			unlisten = stop;
		})();

		onCleanup(() => {
			disposed = true;
			unlisten?.();
		});
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
	const [armedKillPort, setArmedKillPort] = createSignal<number | null>(null);
	const [toast, setToast] = createSignal<string | null>(null);
	let armTimeout: ReturnType<typeof setTimeout> | undefined;

	const disarmKill = () => {
		setArmedKillPort(null);
		if (armTimeout) clearTimeout(armTimeout);
		armTimeout = undefined;
	};

	onCleanup(disarmKill);

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
			disarmKill();
			setTimeout(() => setToast(null), 2500);
		},
	}));

	const handleKillClick = (port: PortInfo) => {
		if (armedKillPort() === port.port) {
			disarmKill();
			killMutation.mutate(port);
			return;
		}

		setArmedKillPort(port.port);
		if (armTimeout) clearTimeout(armTimeout);
		armTimeout = setTimeout(disarmKill, KILL_ARM_MS);
	};

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
				<div class="glass-surface rounded-md px-3 py-2 text-xs text-zinc-200">
					{toast()}
				</div>
			</Show>
			<ScrollArea class="glass-surface min-h-0 flex-1 rounded-md">
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
							killArmed={armedKillPort() === port.port}
							killLabel={t("ports.kill")}
							killConfirmLabel={t("ports.killConfirm")}
							favoriteLabel={t("ports.favorite")}
							onKillClick={() => handleKillClick(port)}
							onToggleFavorite={(p) => favoriteMutation.mutate(p)}
						/>
					))}
				</Show>
			</ScrollArea>
		</div>
	);
}
