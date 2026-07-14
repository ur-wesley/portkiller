export interface PortInfo {
	port: number;
	pid: number;
	process_name: string;
	address: string;
	user: string;
	command: string;
}

export interface AppSettings {
	favorites: number[];
	refresh_interval_secs: number;
	start_minimized: boolean;
	autostart: boolean;
	close_to_tray: boolean;
	monitoring_enabled: boolean;
	locale: string;
	auto_check_updates: boolean;
}

export interface MonitoringStatus {
	active: boolean;
	last_updated_ms: number | null;
}

export interface UpdateInfo {
	version: string;
	currentVersion: string;
	notes: string;
}
