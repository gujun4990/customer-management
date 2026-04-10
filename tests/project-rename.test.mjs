import test from 'node:test'
import assert from 'node:assert/strict'
import { existsSync, readFileSync } from 'node:fs'

test('project app is named customer-management and lives in customer-management directory', () => {
  assert.equal(existsSync('customer-management/package.json'), true)
  assert.equal(existsSync('customer-management/src-tauri/Cargo.toml'), true)
  assert.equal(existsSync('desktop/package.json'), false)

  const pkg = JSON.parse(readFileSync('customer-management/package.json', 'utf8'))
  const cargoToml = readFileSync('customer-management/src-tauri/Cargo.toml', 'utf8')
  const tauriMain = readFileSync('customer-management/src-tauri/src/main.rs', 'utf8')

  assert.equal(pkg.name, 'customer-management')
  assert.equal(cargoToml.includes('name = "customer-management"'), true)
  assert.equal(cargoToml.includes('name = "customer_management_lib"'), true)
  assert.equal(tauriMain.includes('customer_management_lib::run()'), true)
})
