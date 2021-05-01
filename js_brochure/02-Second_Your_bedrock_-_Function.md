## Second: Your Bedrock - Function

Think about Function concept not `function () {}`

### Scenario: Add two numbers

Scenario: You want to make a Function to **add two numbers**:

#### Unsafe: ~Ancient (`function`)~ 

```js
function add(x,y) {
	return x+y
}
```
But it's not safe. Why?

Cause, you can rewrite your own function:
```js
function add(x,y) {
	return x+y
}

add(2,2) // 4


function add(x,y) {
	return 0
}

add(2,2) // 0
```
#### "Safe": But still ~Old~

Using **Basic Principle** (anonymous function)

```js
const add = function(x,y) { return x+y }
```

#### Modern (Arrow Function) but not ~~final~~ safe

```js
const add = (x,y) => x+y
```

### Ternary operator

```js
0 === 0 ? true : false // true
0 === 1 ? true : false // false
```


```js
0 === 0 ? "That is true" : "That is false" // "That is true"
0 === 1 ? "That is true" : "That is false" // "That is false"
```
### My way or highway

This is a good convention:

Return what I want or `false`

Lets make a simplist unsafe power Function:

```js
// First test if x is a "number" by divide by one.
//  Of course all number is divided by one  
const power = x => x/1 ? x * x : false

power(2) // 4 That is a "number"
power("2") // 4 --- Wrong! That is a "string" not a "number"
```
**ALERT**: Do not use the function above! Go to *Chapter 07 - "Math is you ally"* to make a safe version of `power()` Function.

Now we must learn about Functions. Go ahead.


### Functions. Pure or not pure?

Why we want pure? Case **Basic Principle** *rules*

#### Unsafe: Not pure
```js
const DEBUG = true
const debug = message => 
  (DEBUG) ? console.log(message)
  : false

debug("Some debug message")
```
Why?
DEBUG is not defined inside function.

### Safe: Yes, is pure

```js
const DEBUG = true
const debug = (flag,message) => 
  (flag) ? console.log(message)
  : false

debug(DEBUG,"Some debug message")
```
