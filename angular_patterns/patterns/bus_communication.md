# Bus communication

## Advantages
- Decouples communication structure and component structure
- Reduces noise in Input/Output
- Uses dependency injection

## Disadvantages
- Risk of creating a "God"-Bus
- Another layer of async behaviour
- Update loops possible

## Example
### (Local) Bus
```ts
@Injectable()
export class LocalBusService {

    private $something: Subject<Something> = new Subject();

    get something(): Observable<Something> {
        return this.$something.asObservable();
    }

    pushSomething(input): void {
        return this.$something.next(input);
    }

}

```

### Consumer Component
```ts
export class ConsumerComponent {
    constructor(private localBusService: LocalBusService) {
        this.localBusService.something.subscribe(something => {  
            // ...
        });
    }
}
```

### Producer (and optional Provider) Component
```ts
@Component({
    // ...
    providers: [
        LocalBusService
    ]
})
export class ProducerComponent {
    constructor(private localBusService: LocalBusService) {
        this.localBusService.pushSomething(new Something());
    }
}
``` 
