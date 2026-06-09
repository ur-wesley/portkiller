import { Show, type Component } from "solid-js";
import { Button } from "./button";

export const AlertDialog: Component<{
	open: boolean;
	title: string;
	description: string;
	confirmLabel: string;
	cancelLabel: string;
	onConfirm: () => void;
	onCancel: () => void;
}> = (props) => (
	<Show when={props.open}>
		<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4">
			<div class="w-full max-w-sm rounded-lg border border-zinc-700 bg-zinc-950 p-4 shadow-xl">
				<h2 class="text-base font-semibold text-zinc-100">{props.title}</h2>
				<p class="mt-2 text-sm text-zinc-400">{props.description}</p>
				<div class="mt-4 flex justify-end gap-2">
					<Button variant="outline" onClick={props.onCancel}>
						{props.cancelLabel}
					</Button>
					<Button variant="destructive" onClick={props.onConfirm}>
						{props.confirmLabel}
					</Button>
				</div>
			</div>
		</div>
	</Show>
);
