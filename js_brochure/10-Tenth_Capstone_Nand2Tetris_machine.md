## Capstone Project: Nand2tetris project. --> Virtual CPU

Nand2tetris

Logical pOrts

### Elementary logic gates

#### Nand
```js
const Nand = (x,y) => x === true ? y === true ? false : true : true

const testNand = () => {
	console.log("testNand()")
	console.warn(`Nand(false,false):${Nand(false,false)}`)
	console.warn(`Nand(false,true):${Nand(false,true)}`)
	console.warn(`Nand(true,false):${Nand(true,false)}`)
	console.warn(`Nand(true,true):${Nand(true,true)}`)

}

testNand()
```

#### NOT

```js
const Nand = (x,y) => x === true ? y === true ? false : true : true

const Not = x => Nand(x,x)

const testNot = () =>  {
	console.log("testNot()")
	console.warn(`Not(false):${Not(false)}`)
	console.warn(`Not(true):${Not(true)}`)
}

testNot()
```

#### AND

```js
const Nand = (x,y) => x === true ? y === true ? false : true : true
const Not = x => Nand(x,x)

const And = (x,y) => Not(Nand(x,y))

const testAnd = () => {
	console.log("testAnd()")
	console.warn(`And(false,false):${And(false,false)}`)
	console.warn(`And(false,true):${And(false,true)}`)
	console.warn(`And(true,false):${And(true,false)}`)
	console.warn(`And(true,true):${And(true,true)}`)

}

testAnd()
```
#### OR

```js
const Nand = (x,y) => x === true ? y === true ? false : true : true
const Not = x => Nand(x,x)
const And = (x,y) => Not(Nand(x,y))
const Or = (x,y) => Not(And(Not(x),Not(y)))


const testOr = () => {
	console.log("test_Or()")
	console.warn(`Or(false,false):${Or(false,false)}`)
	console.warn(`Or(false,true):${Or(false,true)}`)
	console.warn(`Or(true,false):${Or(true,false)}`)
	console.warn(`Or(true,true):${Or(true,true)}`)

}

testOr()
```

#### XOR
```js
const Nand = (x,y) => x === true ? y === true ? false : true : true
const Not = x => Nand(x,x)
const And = (x,y) => Not(Nand(x,y))
const Or = (x,y) => Not(And(Not(x),Not(y)))

const Xor = (x,y) => Or(And(Not(x),y),And(x,Not(y)))

const testXor = () => {
	console.log("testXor()")
	console.warn(`Xor(false,false):${Xor(false,false)}`)
	console.warn(`Xor(false,true):${Xor(false,true)}`)
	console.warn(`Xor(true,false):${Xor(true,false)}`)
	console.warn(`Xor(true,true):${Xor(true,true)}`)

}

testXor()
```

#### Mux

#### DMux

### 16-bit variants

#### Not16
#### And16
#### Or16
#### Mux16


### Multi-way variants

#### Or8Way
#### Mux4Way16
#### Mux8Way16
#### DMux4Way
#### DMux8Way