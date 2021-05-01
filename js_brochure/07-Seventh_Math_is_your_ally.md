## Seventh: Math is your ally

### Function: `add(x,y)` Safe version
```js
const isNumber = x => x/1 === x ? true : false

const add = (x,y) =>  isNumber(x) && isNumber(y) ? x+y : false
```

### Function: `power(x)` safe version
```js
const power = x => isNumber(x) ? x * x : false
```

### Function: `isEven(x)`
Evaluate integers is easy. But to evaluate floats is necessary to use `String.prototype.slice()` to evaluate only the last digit of a float.

```js
const isEven = x => isNumber(x) ? (x.toString().slice(-1) % 2) === 0 ? true : false : false
```

### Function: `isOdd(x)`
```js
// Dependencies
const isBoolean = bool => typeof(bool) === typeof(true) ? true : false
const not = bool => isBoolean(bool) ? bool === true ? false : true : false
const isNumber = x => x/1 === x ? true : false
const isEven = x => isNumber(x) ? (x.toString().slice(-1) % 2) === 0 ? true : false : false
// isOdd definition
const isOdd = x => isNumber(x) ? not(isEven(x)) ? true : false : false
```


