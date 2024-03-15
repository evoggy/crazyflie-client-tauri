# Crazyflie Tauri client

This client is test to see if it's feasible to write a Crazyflie client using Tauri. The idea
is to do most of the heaving lifting in the Rust backend and only use the React frontend as a
GUI library, just rendering well formatted data from the backend and accepting user input.

## Setup

Clone, install the requirements and run the client:

```bash
git clone https://github.com/evoggy/crazyflie-client-tauri.git
cd crazyflie-client-tauri
npm run tauri dev
```

## Development

### Frontend

The frontend uses React, Bootstrap and TypeScript. It supports hot reloading, so for every change
in the code the application will load it without the need to restart the client.

New dependencies can be added by running ```npm install <package>``` from the main directory.

### Backend

The backend uses Rust and the [crazyflie-lib-rs](https://github.com/ataffanel/crazyflie-lib-rs).
Just like the frontend, the backend also support reloading but has to compile between changes.
New dependencies can be added by entering the ```src-tauri``` directory and running ```cargo add <package>```.

## Generating shared datastructures

For sending data asynchronously between the frontend and the backend various datastructures are used.
These are defined in Rust and the TypeScript equivalent is generated using ts-rs. To generate the
TypeScript definitions you need to run:

```bash
cd src-tauri
cargo test export_bindings
```

This will export the bindings to the file ```src/backend-interface.ts```.
