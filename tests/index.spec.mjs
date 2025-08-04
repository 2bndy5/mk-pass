import test from 'ava'
import { main, generatePassword, validateRequirements } from '../index.js'

test('main', async (t) => {
  t.is(main(['mk-pass']), undefined)
})

test('validateRequirements', (t) => {
  const config = {
    numbers: 15,
    specials: 15
  }
  const expected = {
    length: 16,
    numbers: 13,
    specials: 1,
    firstIsLetter: true
  }
  t.deepEqual(validateRequirements(config), expected)
})

test('generatePassword', (t) => {
  const password = generatePassword({})
  t.is(password.length, 16)
  t.true(password.charAt(0).match(/[a-zA-Z]/) != null)
  const letters = Array.from(password.matchAll(/[a-zA-Z]/g)).length
  t.is(letters, 14)
  const numbers = Array.from(password.matchAll(/[0-9]/g)).length
  t.is(numbers, 1)
  const specials = password.length - letters - numbers
  t.is(specials, 1)
})
