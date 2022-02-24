# Direct communication via Component-ID
Should be used to issue commands

## Advantages
- Interaction logic stays within the template
- Prevent ViewChild

## Disadvantages
- More complicated to test

## Example
### ChildComponent
```ts
@Component({
    selector: "prefix-child-component",
    // ...
})
export class ChildComponent {
    someCommand(): void {
        // does something
    }
}
```
### ParentComponent
```html
<prefix-some-button (click)="childComponent.someCommand()">
</prefix-some-button>
<prefix-child-component #childComponent>
</prefix-child-component>
```

