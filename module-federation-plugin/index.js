const tauriPlugin = () => {
	/** @type {import('@module-federation/runtime/types').FederationRuntimePlugin} */
	const plugin = {
		name: "tauri-module-federation",
		afterResolve: (args) => {
			const url = new URL(args.remoteInfo.entry);
			args.remoteInfo.entry = `module-federation://${url.host}/?fullUrl=${url}`;
			return args;
		},
	};
	return plugin;
};

export default tauriPlugin;
