import { filter, pipe, sortBy } from "remeda";
import type { PortInfo } from "../../lib/api/types";

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
		sortBy((p) => p.port),
	);
}
