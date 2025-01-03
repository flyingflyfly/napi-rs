import { asyncFibonacci, readFileAsync } from './index.js'

async function test() {
//   const bufferr = await readFileAsync('index.js')
//   console.log(bufferr.toString())
  const result = await asyncFibonacci(50)
  console.log(result)
}

test()