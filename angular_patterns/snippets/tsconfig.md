# Advanced configuration of tsconfig
## Purpose
The TSConfig can be configured for different environments, e. g. development, production etc.  
In development, you may not want to have a strict compiler that refuses to compile if you have an unsed import statement, wheras in the CI you would like to fail if this is the case

## Advantages
- Less annoyance during development

## Examples
```json
# tsconfig.json
{
  "compileOnSave": false,
  "compilerOptions": {
    "baseUrl": "./",
    "outDir": "./dist/out-tsc",
    "forceConsistentCasingInFileNames": true,
    "strict": true,
    "strictPropertyInitialization": false,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true,
    "sourceMap": true,
    "declaration": false,
    "downlevelIteration": true,
    "experimentalDecorators": true,
    "moduleResolution": "node",
    "importHelpers": true,
    "target": "es2017",
    "module": "es2020",
    "esModuleInterop": true,
    "skipLibCheck": true,
    "lib": [
      "es2018",
      "dom"
    ]
  },
  "angularCompilerOptions": {
    "enableI18nLegacyMessageIdFormat": false,
    "strictInjectionParameters": true,
    "strictInputAccessModifiers": true,
    "strictTemplates": true,

    "fullTemplateTypeCheck": true,
    "noUnusedLocals": true,
    "forceConsistentCasingInFileNames": true,
    "noFallthroughCasesInSwitch": true,
    "noImplicitReturns": true,
    "strictBindCallApply": true,
    "strictNullChecks": true,
    "strictFunctionTypes": true,
    "noImplicitThis": true,
    "alwaysStrict": true
  }
}
```
```json
# tsconfig.dev
{
  "extends": "./tsconfig.app.json",
  "angularCompilerOptions": {
    "allowUnusedLabels": true,
    "noUnusedLocals": false
  }
}
```
