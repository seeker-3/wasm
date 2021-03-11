import init, * as exports from './pkg/index.js'

void (async () => {
  await init()
  const mod = await exports.default()
  console.log(mod)
  const res = mod.value(null)
  console.log(res)
})().catch(console.error)
