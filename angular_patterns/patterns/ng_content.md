# ng-content
## Advantages
- Hide complex DOM for layouting
- Access from the parent component to the inserted component for communication
- Grant components access to local services

## Example
### Layout-Component
```html
<div class="left-side">
    <ng-content select=".sidebar"></ng-content>
</div>
<div class="right-side">
    <ng-content></ng-content>
</div>
```
### Parent-Component
```html
<prefix-layout-component>
    <prefix-some-other-comp class="sidebar" (click)="doSomething()"></prefix-some-other-comp>
    <div>Other stuff</div>
</prefix-layout-component>

```
