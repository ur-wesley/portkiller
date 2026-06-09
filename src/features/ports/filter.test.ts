import { describe, expect, it } from "vitest";
import type { PortInfo } from "../../lib/api/types";
import { filterPorts } from "./filter";

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
		process_name: "vite.exe",
		address: "127.0.0.1",
		user: "dev",
		command: "vite",
	},
];

describe("filterPorts", () => {
	it("filters by search query", () => {
		const result = filterPorts(sample, "vite", false, []);
		expect(result).toHaveLength(1);
		expect(result[0]?.port).toBe(5173);
	});

	it("filters favorites only", () => {
		const result = filterPorts(sample, "", true, [3000]);
		expect(result).toHaveLength(1);
		expect(result[0]?.port).toBe(3000);
	});
});
