## Fifth: Pitfalls (Do not use them)

### `null` is `object` NOT an unary type
```js
typeof(null) // "object"
```

### `"number"` is more than your think
```js
typeof(0) // "number"
typeof(-1) // "number"
typeof(42) // "number"
typeof(1.1) // "number"
typeof(2.1e1) // "number"
typeof(-4.2) // "number"
typeof(NaN) // "number"
```

### `undefined` is a type
```js
typeof(undefined) // "undefined"
```

### `NaN` is a `"number"`, but not a `NaN`  
```js
typeof(NaN) // "number"
typeof(NaN) === typeof(NaN) // true
typeof(NaN) === typeof(0) // true
NaN === NaN // false
```
