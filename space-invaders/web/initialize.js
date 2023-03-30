import init from './build/space_invaders.js'
import { start } from './src/game.js'

// IIFE to bootstrap WASM
(async () => {
	await init()
	start()
})()
