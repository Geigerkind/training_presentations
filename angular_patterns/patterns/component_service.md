# ComponentServices

## Purpose
- Prevent bubbling or model a command between components
- Prevent unnecessary async code
- Prevent third service that handles communication

## Usage
- Components implement interfaces
- Components then provide these interfaces as a service by refering to themselves
- OR they are used as ViewChilds but casted as this interface

## Advantages
- Intuitive way to program
- Prevents async code
- Can easily be mocked in tests
- More performant, as ChangeDetection can be explicit
- Reduces noise in the component interface itself
- A component can provide multiple interfaces (facedes) for different purposes
- Prevents that a third service is needed for communication and thus prevents that this service is abused in a place where its not supposed to be used

## Disadvantages
- Can cause life cycle issues, as ViewChilds are only available after AfterViewInit

## Examples
TODO
