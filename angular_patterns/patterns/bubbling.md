# Bubbling
Bubbling is the natural way of communicating with components in a hierarical adjacency.

## Advantages
- Utilizes Angular's Life Cycle mechanism (ChangeDetection)

## Disadvantages
- "Extrarenous" code
- Components interfaces can be dirtyed

## Example
This scenario describes the communication from Child1 to Child2 via the Parent.
### Parent
```ts
export class ParentComponent {
    someInputForChild2: string;

    onChild1DoesSomething(input: string): void {
        // Do some computation that changes an input for Child2
        someInputForChild2 = input;
    }
}
```
```html
<child1
    (child1Output)="onChild1DoesSomething($input)"
>
</child1>
<child2
    [child2Input]="someInputForChild2"
>
</child2>
```

### Child 1
```ts
export class Child1Component {
    @Output() child1Output: EventEmitter<string> = new EventEmitter();

    doSomething(): void {
        this.child1Output.emit("WAMBO");
    }
}
``` 

### Child 2
```ts
export class Child2Component {
    @Input() child2Input: string;
}
``` 
