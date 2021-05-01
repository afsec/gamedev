const sleep = (milliseconds) => {
  return new Promise(resolve => setTimeout(resolve, milliseconds))
}

console.log('START')
const demo = async () => {
  for (let i=0;i<9999;i++) {
    await sleep (1)
    console.log(i)  
  }
  console.log('END')
}

demo()
