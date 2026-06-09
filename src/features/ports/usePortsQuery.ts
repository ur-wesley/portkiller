import { useQuery, useQueryClient } from "@tanstack/solid-query";
import { listen } from "@tauri-apps/api/event";
import { onCleanup, onMount } from "solid-js";
import { invokeResult } from "../../lib/api/invoke";
import type { PortInfo } from "../../lib/api/types";
import { portKeys } from "../../lib/query/keys";

export function usePortsQuery() {
	const queryClient = useQueryClient();

	onMount(async () => {
		const unlisten = await listen<PortInfo[]>("ports-updated", (event) => {
			queryClient.setQueryData(portKeys.all, event.payload);
		});
		onCleanup(() => {
			void unlisten();
		});
	});

	return useQuery(() => ({
		queryKey: portKeys.all,
		queryFn: async () => {
			const result = await invokeResult<PortInfo[]>("list_ports");
			if (result.isErr()) throw new Error(result.error);
			return result.value;
		},
	}));
}
