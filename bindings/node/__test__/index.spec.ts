import test from 'ava'
import { main, generatePassword, validateRequirements, Samples } from '../index'

test('main', async (t) => {
  t.is(main(['mk-pass']), undefined)
})

test('validateRequirements', (t) => {
  const config = {
    decimal: 15,
    specials: 15,
  }
  const expected = {
    length: 16,
    decimal: 13,
    specials: 1,
    firstIsLetter: true,
  }
  t.deepEqual(validateRequirements(config), expected)
})

const LOWERCASE = Samples.lowercase().set
const UPPERCASE = Samples.uppercase().set
const DECIMAL = Samples.decimal().set
const SPECIAL_CHARACTERS = Samples.specialCharacters().set

function count_chars(set: Array<String>, phrase: String): number {
  let total = 0
  for (const ch of phrase) {
    if (set.includes(ch)) {
      total += 1
    }
  }
  return total
}

test('generatePassword', (t) => {
  const password = generatePassword({})
  t.is(password.length, 16)
  const first = password.charAt(0)
  const letterIsFirst = LOWERCASE.includes(first) || UPPERCASE.includes(first)
  t.true(letterIsFirst)
  const lowercase = count_chars(LOWERCASE, password)
  const uppercase = count_chars(UPPERCASE, password)
  t.is(lowercase + uppercase, 14)
  const decimals = count_chars(DECIMAL, password)
  t.is(decimals, 1)
  const specials = count_chars(SPECIAL_CHARACTERS, password)
  t.is(specials, 1)
})
