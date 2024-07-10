# Module Federation Example

`guest` is a standalone React app that provides a `Button` component via module federation.
`host` is a Tauri + React app that consumes the `Button` component from the `guest` app.

## Development

Run `pnpm dev` in this directory to start the guest development server and the host Tauri app.

## Production

Run `pnpm build` in this directory to build the guest app and host Tauri app.
Currently the host app requires the guest development server to be running to work, I'm not sure what the best way to handle this is yet.
