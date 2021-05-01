## Third: Guarantees

The modern computing is forged by using Boolean Logic, so let's make our own guarantees by defining Boolean type with bare hands.

Let's make check primitive guarantes that most browsers could give us.

### Default Primitives
```js
typeof(null) // object
typeof(undefined) // undefined
typeof(true) // "boolean"
typeof(false) // "boolean"
typeof(0) // "number"
typeof(0.1) // "number"
typeof("H") // "string"
typeof("Hello") // "string"
typeof(NaN) // "number" really!? Not a Number is a Number?!
```

#### Boolean Type

What is Boolean? Cause, it is a type with two states. **That's A guarantee!**

##### Wrong
```js
typeof(1) // "number"
typeof(0) // "number"
typeof("true") // "string"
typeof("false") // "string"
typeof(TRUE) // "undefined"
typeof(FALSE) // "undefined"
```

##### Right
```js
typeof(true) // "boolean"
typeof(false) // "boolean"
```

##### Function: `isBoolean(bool)` 

```js
const isBoolean = bool => typeof(bool) === "boolean" ? true : false
```

##### Function: `not(bool)`
```js
const not = bool => isBoolean(bool) ? bool === true ? false : true : false
```
