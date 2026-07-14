import { entries, groupBy, map, pipe, sortBy, take } from "remeda";
import type { PortInfo } from "../../lib/api/types";

export interface ProcessSummary {
	name: string;
	count: number;
	ports: number[];
}

export interface FavoriteStatus {
	port: number;
	inUse: boolean;
	process: PortInfo | null;
}

export interface DashboardStats {
	totalPorts: number;
	uniqueProcesses: number;
	favoritesInUse: number;
	favoritesFree: number;
	topProcesses: ProcessSummary[];
	favoriteStatuses: FavoriteStatus[];
}

export function buildDashboardStats(
	ports: PortInfo[],
	favorites: number[],
): DashboardStats {
	const byProcess = groupBy(ports, (p) => p.process_name);
	const topProcesses = pipe(
		entries(byProcess),
		map(([name, items]) => ({
			name,
			count: items.length,
			ports: items.map((p) => p.port).sort((a, b) => a - b),
		})),
		sortBy((p) => -p.count),
		take(5),
	);

	const portByNumber = new Map(ports.map((p) => [p.port, p]));
	const favoriteStatuses = favorites.map((port) => {
		const process = portByNumber.get(port) ?? null;
		return { port, inUse: process !== null, process };
	});

	const favoritesInUse = favoriteStatuses.filter((f) => f.inUse).length;

	return {
		totalPorts: ports.length,
		uniqueProcesses: Object.keys(byProcess).length,
		favoritesInUse,
		favoritesFree: favorites.length - favoritesInUse,
		topProcesses,
		favoriteStatuses,
	};
}

export function countLocalhostPorts(ports: PortInfo[]): number {
	return ports.filter((p) => {
		const addr = p.address.toLowerCase();
		return (
			addr.includes("127.0.0.1") || addr.includes("::1") || addr === "localhost"
		);
	}).length;
}
