# OnPush instead of default behaviour

## Purpose
By default angular imposes `ChangeDetection` onto your frontend project. 
This means that Angular has a heartbeat where it checks every few milliseconds the entire component tree, if any variable has changed. 
This is extremely resource intensive and can induce lag for large applications or devices with poor computational capability.
By changing the behaviour to `OnPush` only input changes and interactions will triger the ChangeDetection. 
Setting the strategy to `OnPush` will prevent the automatic change detection from visiting this node or its children in the component tree.

## Advantages
- Better performance
- Better awareness for changes in the code, thus easier to debug performance issues

## Disadvantages
- May be a little bit harder to handle, because it may require manual ChangeDetection-Handling in some places.

## Usage
1. Set the default `ChangeDetectionStrategy` in the `angular.json` to `OnPush`
```json
{
  ...
  "projects": {
    "frontend": {
      "projectType": "application",
      "schematics": {
        "@schematics/angular:component": {
          "style": "scss",
          "changeDetection": "OnPush"
        },
      }
    }
  }
  ...
}
```
2. Or change certain components to `OnPush` strategy
```ts
@Component({
  ...
  changeDetection: ChangeDetectionStrategy.OnPush,
})
```

3.  Inject the `ChangeDetectorRef` when you need to trigger manual change detection with `changeDetectorRef.markForCheck()`
