import test from 'ava'
import { sum, fibonacci  } from './index.js'

test('sum from native', (t) => {
  t.is(sum(1, 2), 3)
})

test('fibonacci from native', (t) => {
  t.is(fibonacci(20), 6765)
})

