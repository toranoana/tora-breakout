module.exports = {
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/eslint-recommended",
    "plugin:@typescript-eslint/recommended",
    // "plugin:@typescript-eslint/recommended-requiring-type-checking",
    "plugin:prettier/recommended",
    "prettier/@typescript-eslint"
  ],
  plugins: ["@typescript-eslint"],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    sourceType: "module",
    project: "./tsconfig.json",
  },
  env: {
    browser: true,
    node: true,
    es6: true,
  },
  rules: {
    "no-console": "warn",
    "@typescript-eslint/adjacent-overload-signatures": "warn",
    "@typescript-eslint/no-unnecessary-type-assertion": "error",
    "@typescript-eslint/explicit-function-return-type": "off"
  }
}