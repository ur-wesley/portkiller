import { type Component } from "solid-js";
import { Button } from "../../components/ui/button";
import type { PortInfo } from "../../lib/api/types";

function StarIcon(props: { filled: boolean }) {
	return (
		<svg
			viewBox="0 0 24 24"
			class="size-4"
			fill={props.filled ? "currentColor" : "none"}
			stroke="currentColor"
			stroke-width="1.75"
			aria-hidden="true"
		>
			<path d="M12 2.5l2.87 5.82 6.42.93-4.64 4.53 1.1 6.4L12 17.9l-5.75 3.02 1.1-6.4-4.64-4.53 6.42-.93L12 2.5z" />
		</svg>
	);
}

export const PortRow: Component<{
	port: PortInfo;
	isFavorite: boolean;
	killArmed: boolean;
	onKillClick: () => void;
	onToggleFavorite: (port: number) => void;
	killLabel: string;
	killConfirmLabel: string;
	favoriteLabel: string;
}> = (props) => (
	<div class="flex items-center gap-2 border-b border-white/5 px-3 py-2 text-sm hover:bg-white/5">
		<button
			type="button"
			class={`flex size-8 shrink-0 items-center justify-center rounded-md border transition-colors ${
				props.isFavorite
					? "border-amber-500/60 bg-amber-500/15 text-amber-400 hover:bg-amber-500/25"
					: "border-zinc-700 bg-zinc-900/60 text-zinc-500 hover:border-amber-500/40 hover:bg-amber-500/10 hover:text-amber-300"
			}`}
			onClick={() => props.onToggleFavorite(props.port.port)}
			aria-label={props.favoriteLabel}
			aria-pressed={props.isFavorite}
		>
			<StarIcon filled={props.isFavorite} />
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
		<Button
			variant={props.killArmed ? "destructive" : "ghost"}
			size="sm"
			class={
				props.killArmed
					? "transition-all hover:bg-red-500 hover:shadow-md hover:shadow-red-500/25 hover:ring-2 hover:ring-red-400/40"
					: "border border-transparent text-zinc-400 transition-all hover:border-red-500/35 hover:bg-red-500/15 hover:text-red-400 hover:shadow-sm hover:shadow-red-500/10"
			}
			onClick={() => props.onKillClick()}
			aria-pressed={props.killArmed}
		>
			{props.killArmed ? props.killConfirmLabel : props.killLabel}
		</Button>
	</div>
);
