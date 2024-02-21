import test from 'ava'

import { levenshteinDistance, findMostSimilar } from '../index.js'

test('sum from native', (t) => {
  t.is(levenshteinDistance('1', '1'), '0')
})


test('find most similar', (t) => {
  t.is(findMostSimilar( ["apple", "banana", "orange", "grape", "kiwi"], 'pineapple'), 'apple')
})