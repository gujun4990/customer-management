import test from 'node:test'
import assert from 'node:assert/strict'
import { existsSync, readFileSync } from 'node:fs'

test('task 1.1 scaffolds a tauri react typescript app in customer-management', () => {
  assert.equal(existsSync('customer-management/package.json'), true)
  assert.equal(existsSync('customer-management/src-tauri/tauri.conf.json'), true)

  const pkg = JSON.parse(readFileSync('customer-management/package.json', 'utf8'))
  const tauriConfig = JSON.parse(
    readFileSync('customer-management/src-tauri/tauri.conf.json', 'utf8'),
  )

  assert.equal(pkg.name, 'customer-management')
  assert.equal(pkg.type, 'module')
  assert.ok(pkg.scripts.dev)
  assert.ok(pkg.scripts.build)
  assert.equal(tauriConfig.productName, 'Customer Management')
  assert.equal(tauriConfig.app.windows[0].title, 'Customer Management')
  assert.notEqual(tauriConfig.app.security?.csp, null)
})

test('task 1.1 keeps the scaffold free of unused opener and demo command wiring', () => {
  const pkg = JSON.parse(readFileSync('customer-management/package.json', 'utf8'))
  const capability = JSON.parse(
    readFileSync('customer-management/src-tauri/capabilities/default.json', 'utf8'),
  )
  const cargoToml = readFileSync('customer-management/src-tauri/Cargo.toml', 'utf8')
  const tauriLib = readFileSync('customer-management/src-tauri/src/lib.rs', 'utf8')
  const tauriMain = readFileSync('customer-management/src-tauri/src/main.rs', 'utf8')
  const app = readFileSync('customer-management/src/App.tsx', 'utf8')

  assert.equal(pkg.dependencies['@tauri-apps/plugin-opener'], undefined)
  assert.equal(capability.permissions.includes('opener:default'), false)
  assert.equal(cargoToml.includes('tauri-plugin-opener'), false)
  assert.equal(tauriLib.includes('tauri_plugin_opener'), false)
  assert.equal(tauriLib.includes('fn greet'), false)
  assert.equal(tauriMain.includes('customer_management_lib::run()'), true)
  assert.equal(app.includes('invoke('), false)
})
