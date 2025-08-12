# GEMINI.md

## Project Overview

This project is a Tauri (v2) plugin named `tauri-plugin-decorum`. It provides an opinionated take on window decorations (titlebars) for Tauri applications. The main goal is to create custom titlebars that blend well with the application's UI, while retaining native features like Windows Snap Layouts. It supports Windows, macOS, and Linux.

The project is a hybrid Rust and TypeScript project. The core logic is written in Rust, and it exposes a JavaScript API for the frontend.

**Key Technologies:**

*   **Rust:** The core of the plugin is written in Rust.
*   **Tauri:** A framework for building lightweight, cross-platform desktop apps with web technologies.
*   **TypeScript:** The frontend API is written in TypeScript.
*   **Rollup:** Used for bundling the TypeScript code.
*   **Vite:** Used for the example application.
*   **React:** The example application is built with React.

**Architecture:**

*   The Rust code is in the `src` directory. `lib.rs` is the main entry point for the Rust part of the plugin. It defines the public API and handles the platform-specific implementations for window decorations.
*   The TypeScript code is in the `guest-js` directory. `index.ts` defines the JavaScript API that can be used in the frontend of a Tauri application.
*   The `examples/tauri-app` directory contains a full example of how to use the plugin in a Tauri application.

## Building and Running

### Building the plugin

To build the plugin, you need to have Rust and Node.js installed.

1.  Install dependencies: `pnpm install`
2.  Build the TypeScript code: `pnpm build`

This will create the bundled JavaScript files in the `dist-js` directory. The Rust code is built automatically when you build the example application.

### Running the example application

To run the example application, navigate to the `examples/tauri-app` directory and run the following commands:

1.  Install dependencies: `pnpm install`
2.  Run the application in development mode: `pnpm tauri dev`

This will start the Tauri development server and open the example application.

## Development Conventions

*   **Rust:** The Rust code follows the standard Rust conventions.
*   **TypeScript:** The TypeScript code is formatted using Prettier and follows the standard TypeScript conventions.
*   **Testing:** There are no explicit tests in the project, but the example application can be used for manual testing.
*   **Commits:** There is no explicit commit message convention, but the commit history shows that the commits are usually small and focused on a single change.
*   **Permissions:** The plugin requires specific permissions to be set in the `src-tauri/capabilities/default.json` file of the Tauri application. These permissions are documented in the `README.md` file.
