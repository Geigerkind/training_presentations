# Environment variables in code

## Purpose
- Enable runtime configuration through environment variables

## Usage
1. Configure a static javascript file that is loaded manually within the `<head>` tag in the `index.html`.
```html
<script src="/assets/environment.js"></script>
```

```js
window.__env = { ... };
```

2. Build a class that provides `static` wrapper for your environment variables
```ts
export class Environment {
  static someEnvironmentVariable(): boolean {
    return this.retrieveEnvironmentValue("someEnvironmentVariable");
  }

  private static retrieveEnvironmentValue(key: string): boolean {
    // @ts-ignore
    return window.__env[key];
  }
}
```

3. Define the environment variables in the `javascript` file and mount it into the project on runtime
