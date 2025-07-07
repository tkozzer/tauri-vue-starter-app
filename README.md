# Tauri v2 + Vue 3 + Tailwind CSS v4 Starter Template

A comprehensive, production-ready template for building modern desktop applications using Tauri v2, Vue 3, and Tailwind CSS v4. This template provides advanced window management, theme system, debug capabilities, and a beautiful UI with cutting-edge technologies.

## Application Preview

<p align="center">
  <img src="/images/app_only.png" alt="Main Application Window" width="45%" style="margin-right: 10%;">
  <img src="/images/app_debug.png" alt="Application with Debug Window" width="45%">
</p>
<p align="center">
  <em>Left: Main Application Window &nbsp;&nbsp;&nbsp;&nbsp; Right: Application with Debug Window</em>
</p>

## ✨ Key Features

### 🎨 Modern UI & Design System
- **Tailwind CSS v4** with new Vite plugin integration
- **shadcn-vue** components for consistent, accessible UI
- **Reka-ui** headless components for maximum flexibility
- **Lucide Vue** icons for beautiful, scalable iconography
- **Dark/Light theme system** with automatic system detection
- **Smooth theme transitions** and hover-based theme toggle
- **Custom CSS properties** for consistent theming
- **Responsive design** with utility-first approach

### 🖥️ Advanced Window Management
- **Custom titlebar** with native-style window controls (close, minimize, fullscreen)
- **Window dragging** support for seamless desktop experience
- **Transparent windows** with blur effects (macOS)
- **Fullscreen mode** with adaptive UI controls
- **Window state tracking** (position, size, fullscreen status)
- **Multi-window support** with debug window functionality
- **Window positioning** and monitor detection
- **Window effects** with rounded corners and shadows

### 🔧 Developer Experience
- **Debug window mode** for development with real-time window information
- **Type-safe development** with TypeScript throughout (frontend & backend)
- **Hot reload** for both Vue components and Rust code
- **Comprehensive linting** with ESLint (Antfu config) and Rust Clippy
- **Code formatting** with automatic Rust formatting
- **Modular architecture** with clean separation of concerns
- **Environment-based configurations** for different build modes

### 🏗️ Technical Architecture
- **Tauri v2** for lightweight, secure desktop applications
- **Vue 3 Composition API** for reactive and organized frontend development
- **Pinia state management** for centralized app state
- **Event-driven communication** between windows and components
- **Custom Rust modules** for window management and menu system
- **VueUse** for Vue composition utilities
- **TypeScript** for type safety across the entire stack

### 🎯 Frontend Features
- **Component-based architecture** with Vue 3 Composition API
- **Reactive state management** with Pinia
- **Custom UI components** built with shadcn-vue
- **Theme persistence** and system theme detection
- **Window controls integration** with Vue reactivity
- **Event handling** for inter-window communication
- **Type-safe Tauri API** integration

### 🦀 Backend Features
- **Modular Rust architecture** with separate modules
- **Custom window management** with precise positioning
- **Application menu system** with native feel
- **Debug window creation** with environment-based activation
- **Window event handling** and state management
- **Cross-platform compatibility** with platform-specific features
- **Secure IPC** between frontend and backend

## Prerequisites

- [Node.js](https://nodejs.org/) (v18 or newer)
- [Rust](https://www.rust-lang.org/) (latest stable version)
- [pnpm](https://pnpm.io/) (recommended package manager)

## Quick Start

1. **Clone this repository**
   ```bash
   git clone <repository-url>
   cd tauri-vue-starter-app
   ```

2. **Install dependencies**
   ```bash
   pnpm install
   ```

3. **Start development**
   ```bash
   pnpm run dev:tauri
   ```

## 📜 Available Scripts

### Development
- `pnpm run dev` - Start Vite development server (frontend only)
- `pnpm run dev:tauri` - Start full Tauri development environment ⭐
- `pnpm run debug` - Start Tauri with debug window enabled

### Building
- `pnpm run build` - Build Vue app for production
- `pnpm run build:tauri` - Build complete Tauri application ⭐
- `pnpm run preview` - Preview production build (frontend only)
- `pnpm run preview:tauri` - Preview complete Tauri application

### Code Quality
- `pnpm run type-check:ts` - Type check TypeScript/Vue files
- `pnpm run type-check:rust` - Type check Rust code
- `pnpm run type-check:all` - Type check both frontend and backend ⭐
- `pnpm run lint` - Run ESLint on frontend code
- `pnpm run lint:fix` - Auto-fix ESLint issues
- `pnpm run lint:clippy` - Run Rust Clippy linter
- `pnpm run lint:fmt` - Check Rust code formatting
- `pnpm run lint:fmt-fix` - Auto-format Rust code

## 🏗️ Project Structure

```
├── src/                          # Vue 3 Frontend
│   ├── components/               # Vue Components
│   │   ├── ui/                   # shadcn-vue UI Components
│   │   │   ├── button/           # Button component
│   │   │   └── input/            # Input component
│   │   └── Greet.vue             # Example component with Tauri integration
│   ├── stores/                   # Pinia Stores
│   │   └── appStore.ts           # Main app state (theme, window management)
│   ├── lib/                      # Utility Libraries
│   │   └── utils.ts              # Tailwind class utilities
│   ├── assets/                   # Static Assets
│   ├── App.vue                   # Main Vue component
│   ├── main.ts                   # Vue app entry point
│   └── index.css                 # Tailwind CSS v4 imports & theme
├── src-tauri/                    # Tauri Backend
│   ├── src/
│   │   ├── lib/                  # Rust Library Modules
│   │   │   ├── debug_window.rs   # Debug window functionality
│   │   │   ├── menu.rs           # Application menu system
│   │   │   └── mod.rs            # Module declarations & main runner
│   │   └── main.rs               # Rust entry point
│   ├── capabilities/             # Tauri Permissions
│   │   └── default.json          # Default window capabilities
│   ├── tauri.conf.json           # Tauri configuration
│   └── Cargo.toml                # Rust dependencies
├── html/                         # HTML Templates
│   ├── index.html                # Main window template
│   └── debug.html                # Debug window template
├── components.json               # shadcn-vue configuration
├── vite.config.ts               # Vite configuration
├── eslint.config.ts             # ESLint configuration
└── package.json                 # Frontend dependencies & scripts
```

## 🎛️ Core Features Explained

### Window Management
- **Custom Controls**: Native-style window controls (close, minimize, fullscreen)
- **Drag Support**: Click and drag anywhere on the window to move it
- **Fullscreen Mode**: Toggle fullscreen with automatic UI adaptation
- **Transparent Windows**: Beautiful blur effects on macOS
- **Multi-Monitor**: Smart positioning across multiple displays

### Theme System
- **System Detection**: Automatically detects system dark/light preference
- **Manual Toggle**: Click the theme button (🌙/🌞) to override system theme
- **Smooth Transitions**: CSS transitions between theme changes
- **Persistent State**: Theme preference saved in app state
- **Window Title Updates**: Dynamic title reflects current theme

### Debug Window
Enable debug mode to see real-time window information:
```bash
pnpm run debug
```
Features:
- Live window position tracking
- Real-time size dimensions
- Inter-window communication demo
- Development environment detection

### State Management (Pinia)
- **Theme Management**: Dark/light mode state and system detection
- **Window State**: Fullscreen status, position, and size tracking
- **UI State**: Toggle visibility and interaction states
- **Event Emission**: Cross-window communication events

### UI Components (shadcn-vue)
- **Button**: Multiple variants (default, secondary, destructive, ghost)
- **Input**: Styled form inputs with validation support
- **Accessible**: Full keyboard navigation and screen reader support
- **Customizable**: Easy theming with CSS custom properties

## 🎨 Customization

### Adding New shadcn-vue Components
```bash
# Add individual components
pnpm dlx shadcn-vue@latest add card
pnpm dlx shadcn-vue@latest add dialog
pnpm dlx shadcn-vue@latest add table
```

### Theme Customization
Edit `src/index.css` to customize:
- Color schemes and CSS custom properties
- Dark mode variants
- Component styling with Tailwind classes
- Animation and transition effects

### Window Configuration
Modify `src-tauri/tauri.conf.json`:
- Window size and position
- Transparency and decoration settings
- Window effects and styling
- Security and build settings

### Backend Extension
Add new Rust modules in `src-tauri/src/lib/`:
- Custom window management
- System integration
- File system operations
- Native functionality

## 🔧 Development Workflow

### Starting Development
```bash
# Full development environment (recommended)
pnpm run dev:tauri

# With debug window
pnpm run debug

# Frontend only (for UI development)
pnpm run dev
```

### Type Safety
```bash
# Check TypeScript
pnpm run type-check:ts

# Check Rust
pnpm run type-check:rust

# Check everything
pnpm run type-check:all
```

### Code Quality
```bash
# Frontend linting
pnpm run lint
pnpm run lint:fix

# Backend linting
pnpm run lint:clippy
pnpm run lint:fmt
```

### Building
```bash
# Production build
pnpm run build:tauri

# Frontend build only
pnpm run build
```

## 🌐 Technology Stack

<p align="center">
  <img src="public/vite.svg" alt="Vite logo" width="80" height="80">
  <img src="public/tauri.svg" alt="Tauri logo" width="80" height="80">
  <img src="src/assets/vue.svg" alt="Vue logo" width="80" height="80">
</p>

### Frontend
- **[Vue 3](https://vuejs.org/)** - Progressive JavaScript framework with Composition API
- **[Vite](https://vitejs.dev/)** - Next-generation frontend tooling
- **[Tailwind CSS v4](https://tailwindcss.com/)** - Utility-first CSS framework with new Vite plugin
- **[shadcn-vue](https://shadcn-vue.com/)** - High-quality Vue components
- **[Reka-ui](https://reka-ui.com/)** - Headless Vue components
- **[Pinia](https://pinia.vuejs.org/)** - Vue.js state management
- **[VueUse](https://vueuse.org/)** - Collection of Vue composition utilities
- **[TypeScript](https://www.typescriptlang.org/)** - Type-safe JavaScript

### Backend
- **[Tauri v2](https://v2.tauri.app/)** - Secure, lightweight desktop applications
- **[Rust](https://www.rust-lang.org/)** - Fast, safe systems programming language
- **[tauri-plugin-shell](https://github.com/tauri-apps/tauri-plugin-shell)** - Shell operations plugin

### Development Tools
- **[ESLint](https://eslint.org/)** with **[Antfu Config](https://github.com/antfu/eslint-config)** - Code linting
- **[Cargo Clippy](https://doc.rust-lang.org/clippy/)** - Rust linting
- **[vue-tsc](https://github.com/johnsoncodehk/volar)** - TypeScript checking for Vue

## 🚀 Production Deployment

### Building for Distribution
```bash
# Build the complete application
pnpm run build:tauri
```

This creates platform-specific installers in `src-tauri/target/release/bundle/`:
- **macOS**: `.dmg` and `.app` files
- **Windows**: `.exe` and `.msi` files
- **Linux**: `.deb`, `.rpm`, and `.AppImage` files

### Build Configuration
The build process:
1. Runs `pnpm run build` to compile the Vue frontend
2. Compiles Rust code in release mode
3. Bundles everything into platform-specific packages
4. Creates installers and portable executables

## 📚 Learn More

### Documentation
- [Tauri v2 Documentation](https://v2.tauri.app/start/)
- [Vue 3 Guide](https://vuejs.org/guide/)
- [Tailwind CSS v4 Docs](https://tailwindcss.com/docs/installation/using-vite)
- [shadcn-vue Components](https://shadcn-vue.com/docs/introduction)
- [Pinia Documentation](https://pinia.vuejs.org/core-concepts/)

### Tutorials
- [Tauri + Vite Tutorial](https://v2.tauri.app/start/frontend/vite/)
- [Vue 3 Composition API](https://vuejs.org/guide/extras/composition-api-faq.html)
- [Tailwind CSS Best Practices](https://tailwindcss.com/docs/reusing-styles)

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Tauri Team](https://tauri.app/) for the amazing framework
- [Vue.js Team](https://vuejs.org/) for the reactive framework
- [Tailwind CSS Team](https://tailwindcss.com/) for the utility-first CSS framework
- [shadcn](https://twitter.com/shadcn) for the beautiful component system
- [Anthony Fu](https://github.com/antfu) for the excellent ESLint configuration

---

<p align="center">
  <strong>Built with ❤️ using modern web technologies</strong>
</p>
