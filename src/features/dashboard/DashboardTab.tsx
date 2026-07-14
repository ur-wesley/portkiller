import { useQuery } from "@tanstack/solid-query";
import { createMemo, For, Show } from "solid-js";
import { useTabs } from "../../components/ui/tabs";
import { Button } from "../../components/ui/button";
import { Card, CardContent } from "../../components/ui/card";
import { ScrollArea } from "../../components/ui/scroll-area";
import { invokeResult } from "../../lib/api/invoke";
import type { AppSettings } from "../../lib/api/types";
import { useT } from "../../lib/i18n";
import { settingsKeys } from "../../lib/query/keys";
import { useMonitoringStatus } from "../ports/useMonitoringStatus";
import { usePortsQuery } from "../ports/usePortsQuery";
import { buildDashboardStats, countLocalhostPorts } from "./dashboard-stats";

function StatCard(props: {
	label: string;
	value: string | number;
	hint?: string;
	accent?: "emerald" | "amber" | "sky" | "zinc";
}) {
	const accentClass = () => {
		switch (props.accent ?? "zinc") {
			case "emerald":
				return "text-emerald-400";
			case "amber":
				return "text-amber-400";
			case "sky":
				return "text-sky-400";
			default:
				return "text-zinc-100";
		}
	};

	return (
		<Card class="glass-surface">
			<CardContent class="space-y-1 p-3">
				<div class="text-[11px] uppercase tracking-wide text-zinc-500">
					{props.label}
				</div>
				<div class={`text-2xl font-semibold tabular-nums ${accentClass()}`}>
					{props.value}
				</div>
				<Show when={props.hint}>
					<div class="text-xs text-zinc-500">{props.hint}</div>
				</Show>
			</CardContent>
		</Card>
	);
}

export function DashboardTab() {
	const t = useT();
	const tabs = useTabs();
	const portsQuery = usePortsQuery();
	const { statusLabel, lastUpdatedLabel } = useMonitoringStatus();
	const settingsQuery = useQuery(() => ({
		queryKey: settingsKeys.all,
		queryFn: async () => {
			const result = await invokeResult<AppSettings>("get_settings");
			if (result.isErr()) throw new Error(result.error);
			return result.value;
		},
	}));

	const stats = createMemo(() =>
		buildDashboardStats(
			portsQuery.data ?? [],
			settingsQuery.data?.favorites ?? [],
		),
	);

	const localhostCount = createMemo(() =>
		countLocalhostPorts(portsQuery.data ?? []),
	);

	const isLoading = () => portsQuery.isPending || settingsQuery.isPending;

	return (
		<ScrollArea class="h-full min-h-0">
			<div class="flex flex-col gap-4 p-3">
				<div class="flex items-center justify-between gap-2">
					<div>
						<h2 class="text-sm font-semibold text-zinc-100">
							{t("dashboard.title")}
						</h2>
						<p class="text-xs text-zinc-500">{t("dashboard.subtitle")}</p>
					</div>
					<Button
						variant="ghost"
						size="sm"
						disabled={portsQuery.isFetching}
						onClick={() => void portsQuery.refetch()}
					>
						{portsQuery.isFetching
							? t("dashboard.refreshing")
							: t("dashboard.refresh")}
					</Button>
				</div>

				<Show
					when={!isLoading()}
					fallback={
						<Card>
							<CardContent class="text-sm text-zinc-500">
								{t("dashboard.loading")}
							</CardContent>
						</Card>
					}
				>
					<div class="grid grid-cols-2 gap-2">
						<StatCard
							label={t("dashboard.totalPorts")}
							value={stats().totalPorts}
							hint={t("dashboard.localhost", {
								count: localhostCount(),
							})}
							accent="emerald"
						/>
						<StatCard
							label={t("dashboard.uniqueProcesses")}
							value={stats().uniqueProcesses}
							accent="sky"
						/>
						<StatCard
							label={t("dashboard.favoritesInUse")}
							value={stats().favoritesInUse}
							hint={t("dashboard.favoritesFree", {
								count: stats().favoritesFree,
							})}
							accent="amber"
						/>
						<StatCard
							label={t("dashboard.refreshInterval")}
							value={`${settingsQuery.data?.refresh_interval_secs ?? 5}s`}
							hint={t("dashboard.monitoringStatus", {
								status: statusLabel(),
								time: lastUpdatedLabel(),
							})}
						/>
					</div>

					<section class="space-y-2">
						<div class="flex items-center justify-between">
							<h3 class="text-xs font-medium uppercase tracking-wide text-zinc-400">
								{t("dashboard.favoritesWatch")}
							</h3>
							<button
								type="button"
								class="text-xs text-zinc-500 hover:text-zinc-300"
								onClick={() => tabs.setValue("favorites")}
							>
								{t("dashboard.viewAll")}
							</button>
						</div>
						<Show
							when={stats().favoriteStatuses.length > 0}
							fallback={
								<Card>
									<CardContent class="text-sm text-zinc-500">
										{t("dashboard.noFavorites")}
									</CardContent>
								</Card>
							}
						>
							<Card class="glass-surface overflow-hidden">
								<For each={stats().favoriteStatuses}>
									{(favorite) => (
										<div class="flex items-center gap-2 border-b border-zinc-900 px-3 py-2 text-sm last:border-b-0">
											<span
												class={`size-2 rounded-full ${
													favorite.inUse ? "bg-amber-400" : "bg-emerald-500"
												}`}
												aria-hidden="true"
											/>
											<span class="w-12 font-mono text-emerald-400">
												:{favorite.port}
											</span>
											<span class="min-w-0 flex-1 truncate text-zinc-300">
												{favorite.inUse
													? favorite.process?.process_name
													: t("dashboard.portFree")}
											</span>
											<Show when={favorite.inUse}>
												<Button
													variant="ghost"
													size="sm"
													onClick={() => tabs.setValue("ports")}
												>
													{t("dashboard.inspect")}
												</Button>
											</Show>
										</div>
									)}
								</For>
							</Card>
						</Show>
					</section>

					<section class="space-y-2">
						<div class="flex items-center justify-between">
							<h3 class="text-xs font-medium uppercase tracking-wide text-zinc-400">
								{t("dashboard.topProcesses")}
							</h3>
							<button
								type="button"
								class="text-xs text-zinc-500 hover:text-zinc-300"
								onClick={() => tabs.setValue("ports")}
							>
								{t("dashboard.viewPorts")}
							</button>
						</div>
						<Show
							when={stats().topProcesses.length > 0}
							fallback={
								<Card>
									<CardContent class="text-sm text-zinc-500">
										{t("ports.empty")}
									</CardContent>
								</Card>
							}
						>
							<Card class="glass-surface overflow-hidden">
								<For each={stats().topProcesses}>
									{(process) => (
										<div class="flex items-center gap-2 border-b border-zinc-900 px-3 py-2 text-sm last:border-b-0">
											<div class="min-w-0 flex-1">
												<div class="truncate font-medium text-zinc-100">
													{process.name}
												</div>
												<div class="truncate text-xs text-zinc-500">
													{process.ports.map((p) => `:${p}`).join(" · ")}
												</div>
											</div>
											<span class="rounded-md bg-zinc-800 px-2 py-0.5 text-xs tabular-nums text-zinc-300">
												{t("dashboard.portCount", { count: process.count })}
											</span>
										</div>
									)}
								</For>
							</Card>
						</Show>
					</section>
				</Show>
			</div>
		</ScrollArea>
	);
}
