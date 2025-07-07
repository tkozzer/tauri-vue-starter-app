import antfu from "@antfu/eslint-config";

export default antfu({
  // Enable TypeScript support
  typescript: true,

  // Enable Vue support
  vue: true,

  // Disable formatters since you don't want Prettier
  formatters: false,

  stylistic: {
    indent: 2,
    semi: true,
    quotes: "double",
  },

  // Configure for your project structure
  ignores: [
    "dist",
    "dist-ssr",
    "node_modules",
    "src-tauri/target",
    "src-tauri/gen",
    "*.local",
    ".vscode",
    ".idea",
    "src/components/ui/*",
  ],

  // You can add custom rules here if needed
  rules: {
    "ts/no-redeclare": "off",
    "ts/consistent-type-definitions": ["error", "type"],
    "no-console": ["warn"],
    "antfu/no-top-level-await": ["off"],
    "node/prefer-global/process": ["off"],
    "perfectionist/sort-imports": [
      "error",
      {
        tsconfigRootDir: ".",
      },
    ],
  },
}, {
  // Allow process.env in config files
  files: ["*.config.*", "vite.config.ts", "tailwind.config.*"],
  rules: {
    "node/no-process-env": "off",
  },
});
