# ReactiveForms usage

## Purpose
Split presentation from the form control

## Usage
- Create a component for the presentational purpose
- Create a form control model (class) that extends from `FormControl` for this component
- Add the model as input to the presentational component
- Implement everything regarding this model in the model class

## Advantages
- Better cohesion
- Seperation of concerns
- Sub tools can provide domain specific methods
- Sub tools can describe their Form behaviour themselves, e. g. validation, reset etc.
- Main tools can still orchestrate sub tools like they used to do
- Component only needs to deal with presentation and not the form control part

## Where is the difference to simple ControlValueAccessors ?
- CVA create custom control value accessors, i. e. they dont need to depend on already defined ReactiveForm components
- More verbose code
- Less technical code

## Example
```ts
// Custom FormControl model
export class SomeCustomControlFormControl extends FormControl {
    constructor() {
        super({
            controlComp: new FormControl()
        })
    }

    // ... rest of the model logic ...
}

```

```ts
// Presentational Component
export class SomeCustomControl {
    @Input() someCustomControlFormControl: SomeCustomControlFormControl;

    // Do fancy stuff for presentation
}

```

```ts
// Parent compoenent
export class Parent {
    constructor() {
        formGroup = new FormGroup({
            someCustomControl: new SomeCustomControlFormGroup()
        });
    }
}

```


