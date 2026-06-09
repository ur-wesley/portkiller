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
	add_to_path: boolean;
	locale: string;
	auto_check_updates: boolean;
}

export interface UpdateInfo {
	version: string;
	currentVersion: string;
	notes: string;
}

export interface PathStatus {
	in_path: boolean;
	install_dir: string;
}
