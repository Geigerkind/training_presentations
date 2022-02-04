# Testing

## Jest
Use jest instead of Karma/Jasmin. It is extremely fast, because it needs no browser to run and can make use of caching. Also it does not appear to have random errors, like Karma/Jasmin sometimes has.

### Advantages
- Very stable
- CLI support
- Coverage out of the box
- Extremely fast
- Useful mocking API that is easier than `ts-mockito`

### Disadvantages
- Some libraries require transpiling
- DOM tests only work partially or may require additional plugins

### Configuration/Transpilation on the example of OpenLayers
OpenLayers is one of the libraries that requires transpilation using Babel.
The setup is not really straight forward, so in the following some snippets.

1. Install `jest` and remove everything that is related to `Karma` or `Jasmine`.
```json
# package.json

{
  "scripts": {
    ...
    "test": "jest",
    "test:coverage": "jest --coverage",
  },
  "devDependencies": {
    "@types/jest": "^27.0.3",
    "jest": "^27.3.1",
    "jest-canvas-mock": "^2.3.1",
    "jest-junit": "^13.0.0",
    "jest-preset-angular": "^11.0.1",
  }
}
``` 

2. Create a file named `setupJest.ts` in `<root>`
```ts
import "jest-preset-angular/setup-jest";
import "jest-canvas-mock";
```

3. Configure `tsconfig.spec.json` to use jest
```json
...
    "types": [
      "jest",
      "node"
    ]
...
```

4. Create a file named `jest.config.js` in `<root>`
```js
module.exports = {
  preset: "jest-preset-angular",
  setupFilesAfterEnv: ["<rootDir>/setupJest.ts"],
  testPathIgnorePatterns: ["<rootDir>/node_modules/", "<rootDir>/dist/"],
  globals: {
    "ts-jest": {
      tsconfig: "<rootDir>/tsconfig.spec.json",
      stringifyContentPathRegex: "\\.html$",
    },
  },
  transform: {
    "^node_modules/lodash-es/.+\\.js$": "babel-jest", // <---
    "^node_modules/ol/.+\\.js$": "babel-jest" // <- Here happens the transpilation
  },
  transformIgnorePatterns: ["node_modules/(?!(lodash-es/|ol/|.*\\.mjs$))"], // <---
  modulePaths: ["<rootDir>"],
  reporters: ["default", "jest-junit"],
  testResultsProcessor: "jest-junit",
  collectCoverage: true,
  coverageReporters: ["html", "cobertura", "text"],
  cacheDirectory: "./jestCache",
};
```

5. Add to your `.gitignore` the folder `jestCache`. You may also want to ignore the coverage files that are created.
6. Create a file named `.babelrc`
```json
{
  "plugins": [
    "transform-es2015-modules-commonjs"
  ]
}
```

### Using Jest in tests
TODO

## Ng-Mocks
### Purpose
Ng-mocks is a convenient way to construct the TestBed in Angular tests.
It should be considered a must use!

### Advantages
- Simplifies the TestBed setup extremely
- Very nice fluent interface to construct TestBed

### Disdavantages
- Known to have issues in Karma/Jasmine in combination with ViewChilds

### Examples
```ts
describe(SomeComponent.name, () => {
  let fixture: MockedComponentFixture<SomeComponent>;
  let component: SomeComponent;

  beforeEach(() => {
    return MockBuilder(SomeComponent, ModuelWhereComponentIsDeclared);
  });

  beforeEach(() => {
    fixture = MockRender(SomeComponent);
    component = fixture.point.componentInstance;
    fixture.detectChanges();
  });


});
```

