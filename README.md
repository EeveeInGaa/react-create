# React Create

A simple tool to create components and custom hooks from the terminal in React projects.

## Usage Component
``` 
rea g c <ComponentName> / rea gen component <ComponentName>

```
(simplest use case; creates **_tsx_** file with selected name in folder)

> Component Names are supposed to be in **PascalCase/Capitalized** (`ComponentName` or `Component`), 
 however, even if you use **camelCase/lowercase** (`componentName` or `component`), 
 it will be **converted to PascalCase** to meet the requirements of component naming in React.

Options:
Option | Alias | Description
----|-----|-----
Props | `--props` | adds **typed props** to component
Path | `--path` | adds option to **set specific path**; write the path after the alias
CSS File | `--css` | creates css file with **empty root class** and add **className** with imported styles to component
Test File | `--test` | creates test file with **basic structure** (for Vitest, needs to be installed)
Docs File | `--docs` | creates mdx file with **heading** (needs to be installed)
Story File | `--story` | creates **empty** Storybook file (needs to be installed)

## Usage Hook
`rea g h <HookName>` (creates tsx file with selected name in folder)

Currently, there are no options for hooks.

## Added in the future
- [ ] more types to generate (like context, store, type, interface, enum ...)
- [ ] options for hooks
- [ ] option to chose between tsx and jsx
- [ ] option to add all files at once (typed props only with tsx)
- [ ] boilerplate code for storybook