export const fileName = './build/loop.wasm'

export const importObject = {
  console: {
    log: (x: number) => console.log(x),
  },
}
