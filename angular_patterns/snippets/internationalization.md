# Internationalization
## Purpose
Do not hardcode specific language strings in your application, but rather do internationalization from the start, even if there is no demand for it.

## Advantages
- Less duplication
- One file to manage localization
- Another language can be easily added
- Less noise in template code

## Example using i18n
1. Declare the TranslateLoader in the `app.module.ts`
```ts
imports: [
    TranslateModule.forRoot({
      loader: {
        provide: TranslateLoader,
        useFactory: (httpClient) => new TranslateHttpLoader(http, "./assets/i18n/", ".json"),
        deps: [HttpClient],
      },
    }),
    ...
]
```
2. Use the translate pipe in the template
```html
<h1>{{ "some.string" | translate }}</h1>
```
3. Or use the `TranslateService`
4. Declare the strings in some external json, eg. `/assets/i18n/en.json`
```json
{
    "some": {
        "string": "WAMBO"
    }
}
```

