# Tauri v2 + Vue 3 Starter Template

This template provides a solid foundation for developing desktop applications using Tauri v2 and Vue 3. It incorporates the Vue 3 Composition API and Pinia for state management, offering a modern and efficient development experience.

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
   npm install
   ```

## Available Scripts

In the project directory, you can run:

- `npm run dev`: Starts the Vite development server
- `npm run build`: Builds the Vue app for production
- `npm run preview`: Locally preview the production build
- `npm run dev:tauri`: Starts the Tauri development environment
- `npm run build:tauri`: Builds the Tauri application for production
- `npm run preview:tauri`: Previews the Tauri application
- `npm run debug`: Starts the Tauri development environment with debug mode enabled

## Project Structure

- `src/`: Vue 3 frontend source code
- `src-tauri/`: Tauri backend source code
- `public/`: Static assets
- `src-tauri/src/lib/`: Rust library code for the Tauri backend

## Customization

- Modify `src-tauri/tauri.conf.json` to customize your application's metadata and build settings
- Update the menu configuration in the appropriate Tauri configuration file

## Learn More

- [Tauri Documentation](https://v2.tauri.app) (documentation is not yet complete)
- [Vue 3 Documentation](https://v3.vuejs.org/)
- [Pinia Documentation](https://pinia.vuejs.org/)

## License

[MIT License](LICENSE)
