#!/usr/bin/env node
const passGen = require('./index.js')

process.argv.shift() // pop the path to `node` interpreter

passGen.main(process.argv)
