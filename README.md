# Tauri v2 + Vue 3 Starter Template

This template provides a solid foundation for developing desktop applications using Tauri v2 and Vue 3. It incorporates the Vue 3 Composition API and Pinia for state management, offering a modern and efficient development experience.

## Application Preview

<p align="center">
  <img src="/images/app_only.png" alt="Main Application Window" width="45%" style="margin-right: 10%;">
  <img src="/images/app_debug.png" alt="Application with Debug Window" width="45%">
</p>
<p align="center">
  <em>Left: Main Application Window &nbsp;&nbsp;&nbsp;&nbsp; Right: Application with Debug Window</em>
</p>

## Features

- Tauri v2 for creating lightweight, secure desktop applications
- Vue 3 with Composition API for reactive and organized frontend development
- Pinia for state management
- Debug window mode for easier development
- Custom menu configuration
- TailwindCSS for utility-first styling

## Prerequisites

- [Node.js](https://nodejs.org/) (v14 or newer recommended)
- [Rust](https://www.rust-lang.org/) (latest stable version)

## Getting Started

1. Clone this repository
2. Install dependencies:
   ```
   pnpm install
   ```

## Available Scripts

In the project directory, you can run:

- `pnpm run dev`: Starts the Vite development server
- `pnpm run build`: Builds the Vue app for production
- `pnpm run preview`: Locally preview the production build
- `pnpm run dev:tauri`: Starts the Tauri development environment
- `pnpm run build:tauri`: Builds the Tauri application for production
- `pnpm run preview:tauri`: Previews the Tauri application
- `pnpm run debug`: Starts the Tauri development environment with debug mode enabled

## Project Structure

- `src/`: Vue 3 frontend source code
- `src-tauri/`: Tauri backend source code
- `public/`: Static assets
- `src-tauri/src/lib/`: Rust library code for the Tauri backend

## Customization

- Modify `src-tauri/tauri.conf.json` to customize your application's metadata and build settings
- Update the menu configuration in the appropriate Tauri configuration file
- Custom titlebar:
  - The app uses a custom titlebar implemented in `src/App.vue`
  - Window controls (close, minimize, fullscreen) are included in the custom titlebar
  - The window is set to be transparent and without decorations in `src-tauri/tauri.conf.json`
  - Necessary permissions for window management are set in `src-tauri/capabilities/default.json`
  - To modify the titlebar:
    - Adjust the HTML and CSS in `src/App.vue`
    - Update window properties in `src-tauri/tauri.conf.json` if needed
    - Ensure proper permissions are set in `src-tauri/capabilities/default.json`

## Technologies

This project is built with:

<p align="center">
  <img src="public/vite.svg" alt="Vite logo" width="100" height="100">
  <img src="public/tauri.svg" alt="Tauri logo" width="100" height="100">
  <img src="src/assets/vue.svg" alt="Vue logo" width="100" height="100">
</p>

- [Vite](https://vitejs.dev/): Next Generation Frontend Tooling
- [Tauri](https://tauri.app/): Build smaller, faster, and more secure desktop applications with a web frontend
- [Vue.js](https://vuejs.org/): The Progressive JavaScript Framework

## Learn More

- [Tauri Documentation](https://v2.tauri.app) (documentation is not yet complete)
- [Vue 3 Documentation](https://v3.vuejs.org/)
- [Pinia Documentation](https://pinia.vuejs.org/)

## License

[MIT License](LICENSE)
