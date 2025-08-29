import test, { ExecutionContext } from 'ava'
import { main, generatePassword, validateRequirements, Samples, PasswordRequirements } from '../index'

test('main', async (t) => {
  // just ensure the main() function did not panic.
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
    allowRepeats: false,
  }
  t.deepEqual(validateRequirements(config), expected)
})

const LOWERCASE = Samples.lowercase().set
const UPPERCASE = Samples.uppercase().set
const DECIMAL = Samples.decimal().set
const SPECIAL_CHARACTERS = Samples.specialCharacters().set

function countChars(set: Array<String>, phrase: String): number {
  let total = 0
  for (const ch of phrase) {
    if (set.includes(ch)) {
      total += 1
    }
  }
  return total
}

function countRepeats(password: String): number {
  let repeats: Array<String> = []
  for (let i = 0; i < password.length; ++i) {
    const ch = password.charAt(i)
    if (password.substring(0, i).includes(ch) && !repeats.includes(ch)) {
      repeats.push(ch)
    }
  }
  return repeats.length
}

function assertPasswordIsExpected(t: ExecutionContext<unknown>, password: String, config: PasswordRequirements) {
  const conf = validateRequirements(config)
  t.is(password.length, conf.length!)
  if (conf.firstIsLetter!) {
    const first = password.charAt(0)
    const letterIsFirst = LOWERCASE.includes(first) || UPPERCASE.includes(first)
    t.true(letterIsFirst)
  }
  const lowercase = countChars(LOWERCASE, password)
  const uppercase = countChars(UPPERCASE, password)
  const letters = lowercase + uppercase
  t.is(letters, conf.length! - conf.decimal! - conf.specials!)
  const decimals = countChars(DECIMAL, password)
  t.is(decimals, conf.decimal!)
  const specials = countChars(SPECIAL_CHARACTERS, password)
  t.is(specials, conf.specials!)
  const repeats = countRepeats(password)
  t.is(repeats > 0, conf.allowRepeats!)
}

test('generatePassword', (t) => {
  const password = generatePassword({})
  assertPasswordIsExpected(t, password, {})
})

test('allowRepeats', (t) => {
  const config = { length: 20, decimal: 18, specials: 0, allowRepeats: true }
  const password = generatePassword(config)
  assertPasswordIsExpected(t, password, config)
})
