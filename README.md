# React Create

A simple tool to create components and custom hooks from the terminal in React projects.

## What can be created?

| Option        | Command                   |
|---------------|---------------------------|
| Component (c) | `rea g c <ComponentName>` |
| Hook (h)      | `rea g h <hookName>`      |
| Function (fn) | `rea g fn <functionName>` |
| Interface (i) | `rea g i <InterfaceName>` |
| Type (t)      | `rea g t <TypeName>`      |
| Enum (e)      | `rea g e <EnumName>`      |

## Usage
### Component
```
rea g c <ComponentName> / rea gen component <ComponentName>
```
(simplest use case; creates **_tsx_** file with selected name in a 'components' folder) 

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

### Hook
```
rea g h <hookName> / rea gen hook <hookName>
```
(creates **_ts_** file with selected name in a 'hooks' folder)

> Hook name will be in **camelCase/lowercase** (`useHookName` or `useName`; 'use' gets added automatically), <br>
> File name will equal hook name (`useHookName` or `useName`; no suffix). <br>
> (gets converted automatically)

Currently, there are no options for hooks.

### Function
```
rea g fn <functionName> / rea gen function <functionName>
```
(creates **_ts_** file with selected name in a 'functions' folder)

> Function name will be in **camelCase/lowercase** (`functionName` or `name`), <br>
> File name will be in **kebab-case** (`function-name` or `name`; no suffix). <br>
> (gets converted automatically)

Currently, there are no options for functions.

### Interface
```
rea g i <InterfaceName> / rea gen interface <InterfaceName>
```
(creates **_ts_** file with selected name in an 'interfaces' folder)

> Interface name will be in **PascalCase/Capitalized** (`InterfaceName` or `Name`), <br>
> File name will be in **kebab-case** (`interface-name` or `name`; no suffix). <br>
> (gets converted automatically)

Currently, there are no options for interfaces.

### Type
```
rea g t <TypeName> / rea gen type <TypeName>
```
(creates **_ts_** file with selected name in a 'types' folder)

> Type name will be in **PascalCase/Capitalized** (`TypeName` or `Name`), <br>
> File name will be in **kebab-case** (`type-name` or `name`; no suffix). <br>
> (gets converted automatically)

Currently, there are no options for types.

### Enum
```
rea g e <EnumName> / rea gen enum <EnumName>
```
(creates ts file with selected name in an 'enums' folder)

> Enum name will be in **PascalCase/Capitalized** (`EnumName` or `Name`), <br>
> File name will be in **kebab-case** (`enum-name` or `name`; no suffix). <br>
> (gets converted automatically)

Currently, there are no options for enums.

## Added in the future
- [ ] more types to generate 
  - [ ] context
  - [ ] store
  - [x] type
  - [x] interface
  - [x] enum
  - [x] function
- [ ] options for hooks
- [ ] option to choose between tsx and jsx
- [ ] option to add all files at once (typed props only with tsx)
- [ ] boilerplate code for storybook
- [ ] tests for different generators