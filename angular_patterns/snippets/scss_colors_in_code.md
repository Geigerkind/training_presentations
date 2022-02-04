# SCSS Colors in Code

## Purpose
- Expose SCSS colors to the `ts`-Code but keep colors in one place

## Usage
1. Define your colors in some `colors.scss` file
```scss
$myFancyColor: #123abc;

:root {
    --some-prefix-myFancyColor: #{$myFancyColor};
}
```

2. Make sure to load them everywhere, either by putting them into the `style.scss` file or by adding them to the `angular.json` at styles.
3. Create a class to expose `static` bindings
```ts
export class CustomColors {
  static myFancyColor(transparency: number = 1): string {
    return RrpBwColors.getCssColorVariable("--some-prefix-myFancyColor", transparency);
  }


  private static getCssColorVariable(variableName: string, transparency: number): string {
    // In Some environments (e.g. Chrome) the result of the CSS variable can have spaces.
    let color = window.getComputedStyle(document.documentElement).getPropertyValue(variableName).trim();
    if (transparency === 1) {
      return color;
    }

    if (color.length === 4) {
      // Angular minifies colors like #RRGGBB at runtime to #RGB, so we have to restore them.
      color = "#" + color[1] + color[1] + color[2] + color[2] + color[3] + color[3];
    }

    const transparencyHex = Math.min(255, Math.ceil(transparency * 255)).toString(16);
    if (transparencyHex.length === 1) {
      return color + "0" + transparencyHex;
    }
    return color + transparencyHex;
  }

}
```
