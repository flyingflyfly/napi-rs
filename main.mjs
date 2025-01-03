import { asyncFibonacci, fibonacci,readFileAsync, multiThreadedFibonacci } from './index.js'

async function test() {
//   const bufferr = await readFileAsync('index.js')
//   console.log(bufferr.toString())
  console.time('js fibonacci')
  console.log('fibonacci(35)', fibonacci(40))
  console.timeEnd('js fibonacci')
  console.time('rust fibonacci')
  const result = multiThreadedFibonacci(40)
  console.timeEnd('rust fibonacci')
  console.log(result)
}

test()