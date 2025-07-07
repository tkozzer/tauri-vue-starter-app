# Tauri v2 + Vue 3 Starter Template

This template provides a sophisticated foundation for developing modern desktop applications using Tauri v2 and Vue 3. It features advanced window management, theme system, debug capabilities, and a beautiful UI built with the latest technologies.

## Application Preview

<p align="center">
  <img src="/images/app_only.png" alt="Main Application Window" width="45%" style="margin-right: 10%;">
  <img src="/images/app_debug.png" alt="Application with Debug Window" width="45%">
</p>
<p align="center">
  <em>Left: Main Application Window &nbsp;&nbsp;&nbsp;&nbsp; Right: Application with Debug Window</em>
</p>

## ✨ Features

### 🎨 Modern UI & Theming
- **Tailwind CSS v4** with new Vite plugin integration
- **Dark/Light theme system** with automatic system theme detection
- **Smooth theme transitions** and hover-based theme toggle
- **Custom window effects** with transparency and blur (macOS)
- **Responsive design** with utility-first CSS approach

### 🖥️ Advanced Window Management
- **Custom titlebar** with macOS-style window controls (close, minimize, fullscreen)
- **Window dragging** support for seamless user experience
- **Fullscreen mode** with adaptive UI controls
- **Window state tracking** (position, size, fullscreen status)
- **Multi-window support** with debug window functionality

### 🔧 Developer Experience
- **Debug window mode** for development with real-time window information
- **Type-safe development** with TypeScript throughout
- **Hot reload** for both Vue components and Rust code
- **Comprehensive type checking** for both frontend and backend
- **Modular architecture** with clean separation of concerns

### 🏗️ Technical Architecture
- **Tauri v2** for lightweight, secure desktop applications
- **Vue 3 Composition API** for reactive and organized frontend development
- **Pinia state management** for centralized app state
- **Event-driven communication** between windows
- **Custom Rust modules** for window management and menu system

## Prerequisites

- [Node.js](https://nodejs.org/) (v18 or newer recommended)
- [Rust](https://www.rust-lang.org/) (latest stable version)
- [pnpm](https://pnpm.io/) (recommended package manager)

## Getting Started

1. Clone this repository
2. Install dependencies:
   ```bash
   pnpm install
   ```

## 📜 Available Scripts

### Development
- `pnpm run dev`: Starts the Vite development server (frontend only)
- `pnpm run dev:tauri`: Starts the Tauri development environment (recommended)
- `pnpm run debug`: Starts Tauri with debug window enabled

### Building
- `pnpm run build`: Builds the Vue app for production
- `pnpm run build:tauri`: Builds the complete Tauri application for production
- `pnpm run preview`: Locally preview the production build
- `pnpm run preview:tauri`: Previews the Tauri application

### Type Checking
- `pnpm run type-check:ts`: Type check TypeScript/Vue files
- `pnpm run type-check:rust`: Type check Rust code
- `pnpm run type-check:all`: Type check both frontend and backend

## 🏗️ Project Structure

```
├── src/                          # Vue 3 frontend source code
│   ├── components/               # Vue components
│   │   └── Greet.vue            # Example greeting component
│   ├── stores/                   # Pinia stores
│   │   └── appStore.ts          # Main application state management
│   ├── App.vue                   # Main application component
│   ├── main.ts                   # Vue application entry point
│   └── index.css                # Tailwind CSS imports and global styles
├── src-tauri/                    # Tauri backend source code
│   ├── src/
│   │   ├── lib/                  # Rust library modules
│   │   │   ├── debug_window.rs   # Debug window functionality
│   │   │   ├── menu.rs           # Application menu system
│   │   │   └── mod.rs            # Module declarations and main run function
│   │   └── main.rs               # Rust application entry point
│   ├── capabilities/             # Tauri permissions and capabilities
│   │   └── default.json          # Default window permissions
│   ├── tauri.conf.json           # Tauri application configuration
│   └── Cargo.toml               # Rust dependencies
├── debug.html                    # Debug window HTML interface
├── public/                       # Static assets
├── vite.config.ts               # Vite configuration with Tailwind
└── package.json                 # Frontend dependencies and scripts
```

## 🎛️ Key Features Explained

### Custom Window Controls
The application features a custom titlebar with macOS-style window controls:
- **Close button** (red) - Closes the application
- **Minimize button** (yellow) - Minimizes the window
- **Fullscreen button** (green) - Toggles fullscreen mode
- Controls automatically hide in fullscreen mode

### Theme System
- **Automatic system theme detection** on startup
- **Manual theme toggle** via hover button in top-right corner
- **Smooth transitions** between light and dark modes
- **Window title updates** to reflect current theme

### Debug Window
Enable debug mode to see real-time window information:
```bash
pnpm run debug
```
The debug window displays:
- Main window position coordinates
- Window size dimensions
- Real-time updates when moving or resizing

### State Management
Centralized state management with Pinia includes:
- Theme preferences and system detection
- Window state (fullscreen, toggle visibility)
- Window position and size tracking
- Event emission for inter-window communication

## 🔧 Customization

### Window Configuration
Modify `src-tauri/tauri.conf.json` to customize:
- Window size and position
- Transparency and decoration settings
- Window effects and styling
- Security and build settings

### Theme Customization
Update `src/index.css` and component styles to customize:
- Color schemes and transitions
- Dark mode variants
- Component styling with Tailwind classes

### Permissions
Update `src-tauri/capabilities/default.json` to modify:
- Window management permissions
- API access capabilities
- Security restrictions

## 🛠️ Development Workflow

### Starting Development
```bash
# Start the full development environment
pnpm run dev:tauri

# Or start with debug window
pnpm run debug
```

### Type Safety
```bash
# Check all TypeScript
pnpm run type-check:ts

# Check Rust code
pnpm run type-check:rust

# Check everything
pnpm run type-check:all
```

### Building for Production
```bash
# Build the complete application
pnpm run build:tauri
```

## 🌐 Technologies

This project is built with cutting-edge technologies:

<p align="center">
  <img src="public/vite.svg" alt="Vite logo" width="100" height="100">
  <img src="public/tauri.svg" alt="Tauri logo" width="100" height="100">
  <img src="src/assets/vue.svg" alt="Vue logo" width="100" height="100">
</p>

- **[Vite](https://vitejs.dev/)**: Next Generation Frontend Tooling
- **[Tauri v2](https://v2.tauri.app/)**: Build smaller, faster, and more secure desktop applications
- **[Vue 3](https://vuejs.org/)**: Progressive JavaScript Framework with Composition API
- **[Tailwind CSS v4](https://tailwindcss.com/)**: Utility-first CSS framework with Vite plugin
- **[Pinia](https://pinia.vuejs.org/)**: Vue.js state management library
- **[TypeScript](https://www.typescriptlang.org/)**: Type-safe JavaScript development

## 📚 Learn More

- [Tauri v2 Documentation](https://v2.tauri.app/start/)
- [Vue 3 Documentation](https://vuejs.org/guide/introduction.html)
- [Pinia Documentation](https://pinia.vuejs.org/core-concepts/)
- [Tailwind CSS v4 Documentation](https://tailwindcss.com/docs/installation/using-vite)
- [Vite Documentation](https://vite.dev/guide/)

## 📄 License

[MIT License](LICENSE)
