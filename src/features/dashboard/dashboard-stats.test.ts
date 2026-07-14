import { describe, expect, it } from "vitest";
import type { PortInfo } from "../../lib/api/types";
import { buildDashboardStats, countLocalhostPorts } from "./dashboard-stats";

const sample: PortInfo[] = [
	{
		port: 3000,
		pid: 1,
		process_name: "node.exe",
		address: "127.0.0.1",
		user: "dev",
		command: "node server.js",
	},
	{
		port: 5173,
		pid: 2,
		process_name: "node.exe",
		address: "127.0.0.1",
		user: "dev",
		command: "vite",
	},
	{
		port: 8080,
		pid: 3,
		process_name: "java.exe",
		address: "0.0.0.0",
		user: "dev",
		command: "java -jar app.jar",
	},
];

describe("buildDashboardStats", () => {
	it("aggregates port and process counts", () => {
		const stats = buildDashboardStats(sample, [3000, 8080, 9000]);
		expect(stats.totalPorts).toBe(3);
		expect(stats.uniqueProcesses).toBe(2);
		expect(stats.favoritesInUse).toBe(2);
		expect(stats.favoritesFree).toBe(1);
	});

	it("ranks top processes by port count", () => {
		const stats = buildDashboardStats(sample, []);
		expect(stats.topProcesses[0]?.name).toBe("node.exe");
		expect(stats.topProcesses[0]?.count).toBe(2);
	});

	it("tracks favorite availability", () => {
		const stats = buildDashboardStats(sample, [3000, 9000]);
		const busy = stats.favoriteStatuses.find((f) => f.port === 3000);
		const free = stats.favoriteStatuses.find((f) => f.port === 9000);
		expect(busy?.inUse).toBe(true);
		expect(busy?.process?.process_name).toBe("node.exe");
		expect(free?.inUse).toBe(false);
	});
});

describe("countLocalhostPorts", () => {
	it("counts loopback listeners only", () => {
		expect(countLocalhostPorts(sample)).toBe(2);
	});
});
