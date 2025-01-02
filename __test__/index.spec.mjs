import test from 'ava'

import { sum } from '../index'

test('sum from native', (t) => {
  t.is(sum(1, 2), 3)
})
