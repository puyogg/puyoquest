/** @type {import('ts-jest/dist/types').InitialOptionsTsJest} */
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  globalSetup: './test/read.global-setup.ts',
  globalTeardown: './test/read.global-teardown.ts'
};
