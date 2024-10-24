Backslashes implementations required.

Given broken JSON:

```js
{
  name: "Jesse",
  age: 0069,
  something: None,
  foods: [, 'chocolate'
```

Returns:

```json
{
  "name": "Jesse",
  "age": 69,
  "something": null,
  "foods": ["chocolate"]
}
```
