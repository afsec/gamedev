## Eighth: String and character.

### Function: `isString(str)`

```js
const isString = str => typeof(str) === typeof("") ? true : false
```

### Function `isChar(chr)`

```js
const isChar = chr => (isString(chr)) ? ( chr.length === 1 ? true : false) : false
```

### Function: `newChar(chr)`

```js
// Dependencies
const isString = str => typeof(str) === typeof("") ? true : false
const isChar = chr => (isString(chr)) ? ( chr.length === 1 ? true : false) : false
// newChar definition
const newChar = chr => isChar(chr) ? chr : false
```

### Fancy DEBUG Messages
```js
const DEBUG = true

const debug = message => DEBUG === true ? console.log('DEBUG:', message) : false

debug("Just a useful debug message")
```


### Function: `isNotANumber()`
```js
// Dependencies
const isBoolean = bool => typeof(bool) === typeof(true) ? true : false
const not = bool => (isBoolean(bool) ? ( bool === true ? false : true ) : false)
const isNumber = x => x/1 === x ? true : false
// isNotANumber definition
const isNotANumber = x => not(isNumber(x)) ? true : false
```


### Function `isNotString()`

```js
// Dependencies
const isBoolean = bool => typeof(bool) === typeof(true) ? true : false
const not = bool => (isBoolean(bool) ? ( bool === true ? false : true ) : false)
const isString = str => typeof(str) === typeof("") ? true : false
// isNotString definition
const isNotString = chr => not(isString(chr)) ? true : false
```


### Function `isNotChar(chr)` 
```js
// Dependencies
const isBoolean = bool => typeof(bool) === typeof(true) ? true : false
const not = bool => (isBoolean(bool) ? ( bool === true ? false : true ) : false)
const isString = str => (typeof(str) === typeof("")) ? true : false
// isNotChar definition
const isChar = chr => (isString(chr)) ? ( chr.length === 1 ? true : false) : false
```
