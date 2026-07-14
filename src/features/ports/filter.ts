import { filter, pipe, sortBy } from "remeda";
import type { PortInfo } from "../../lib/api/types";

export function sortPorts(ports: PortInfo[], favorites: number[]) {
	return pipe(
		ports,
		sortBy((p) => [favorites.includes(p.port) ? 0 : 1, p.port] as const),
	);
}

export function filterPorts(
	ports: PortInfo[],
	query: string,
	favoritesOnly: boolean,
	favorites: number[],
) {
	const q = query.trim().toLowerCase();
	return pipe(
		ports,
		filter((p) => {
			if (favoritesOnly && !favorites.includes(p.port)) return false;
			if (!q) return true;
			return (
				p.port.toString().includes(q) ||
				p.process_name.toLowerCase().includes(q) ||
				p.command.toLowerCase().includes(q)
			);
		}),
		(items) => sortPorts(items, favorites),
	);
}
