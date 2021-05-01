### Sixth: Extending Primitives and make your own guarantees


#### Function: `isNumber(x)`

```js
const isNumber = x => x/1 === x ? true : false
```

### Function: `isInteger(x)`?

**What is a Integer?**
Is a `"number"` with no `.` inside.

```js
const isInteger = x => isNumber(x) ? x.toString().indexOf(".") === -1 ? true : false : false 
```
To learn why I use indexOf() see seach about "String.prototype.indexOf()"


### Function: `newInteger(x)` "Type guarantee"

```js
const newInteger = x => isInteger(x) ? x : false
```


### Function: `isFloat(x)`

Concept: A Float must be a `number` **and** `not` an Integer.

```js
const isFloat = x =>  isNumber(x) ? not(isInteger(x)) ? true : false : false
```


### Function: `newFloat(x)` "Type guarantee"
```js
// Dependencies
const isBoolean = bool => typeof(bool) === typeof(true) ? true : false
const not = bool => isBoolean(bool) ? bool === true ? false : true : false
const isNumber = x => x/1 === x ? true : false
const isInteger = x => isNumber(x) ? x.toString().indexOf(".") === -1 ? true : false : false 
const isFloat = x =>  isNumber(x) ? not(isInteger(x)) ? true : false : false
// newFloat definition
const newFloat = x => isFloat(x) ? x : false
```

