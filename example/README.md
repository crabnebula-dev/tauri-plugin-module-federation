# Module Federation Example

`guest` and `guest-2` ars standalone React apps that provides a `Button` component via module federation.
`guest` consumes the `Button` from `guest-2` as a dependency-of-dependency test.
`host` is a Tauri + React app that consumes the `Button` component from the `guest` app.

## Development

Run `pnpm dev` in this directory to start the `guest` development servers and the host Tauri app.

## Production

Run `pnpm build` in this directory to build the guest app and host Tauri app.
Currently the host app requires the guest development server to be running to work.
This will change once [Zephyr Cloud](https://zephyr-cloud.io/) is properly integrated.

## Zephyr Cloud integration

To use the Zephyr Cloud integration, run the above commands with `WITH_ZEPHYR=true` set.
This is currently disabled by default as it's not working properly.
