import init from './pkg/index.js'

void (async () => {
  await init()
})().catch(console.error)

// const button = document.createElement('button')
// button.innerText = 'click'
// document.body.appendChild(button)

// void (async () => {
//   await init()
//   // const mod = await exports.default()
//   const { Domm } = exports

//   const d = Domm.new()
//   button.onclick = () => {
//     d.print()
//   }
// })().catch(console.error)
