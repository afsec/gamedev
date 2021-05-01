## First: Basic Principle

Is Immutable by default and confined.


### Variables must be confined

### Wrong
```js
var x = 10
```

### Right

```js
let x = 10
```

### Function must me immutable

### Wrong
```js
var notRandom = function () { return 10 }
var increment = function (x) { return x+1 }
var add = function(x,y) { return x+y }
```

### Right

Be cool We'll the bellow later.
```js
const notRandom = () => 10
const increment = x => x+1
const add = (x,y) => x+y

```
