import { type Component } from "solid-js";
import { Button } from "../../components/ui/button";
import type { PortInfo } from "../../lib/api/types";

export const PortRow: Component<{
	port: PortInfo;
	isFavorite: boolean;
	onKill: (port: PortInfo) => void;
	onToggleFavorite: (port: number) => void;
	killLabel: string;
}> = (props) => (
	<div class="flex items-center gap-2 border-b border-zinc-900 px-3 py-2 text-sm hover:bg-zinc-900/60">
		<button
			type="button"
			class="text-zinc-500 hover:text-amber-400"
			onClick={() => props.onToggleFavorite(props.port.port)}
			aria-label="favorite"
		>
			{props.isFavorite ? "★" : "☆"}
		</button>
		<div class="w-14 font-mono text-emerald-400">:{props.port.port}</div>
		<div class="min-w-0 flex-1">
			<div class="truncate font-medium text-zinc-100">
				{props.port.process_name}
			</div>
			<div class="truncate text-xs text-zinc-500">
				pid {props.port.pid} · {props.port.address}
			</div>
		</div>
		<Button variant="ghost" size="sm" onClick={() => props.onKill(props.port)}>
			{props.killLabel}
		</Button>
	</div>
);
