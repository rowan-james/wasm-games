import init from './build/maze_war.js'
import { GameManager } from './src/game-manager.js'

// IIFE to bootstrap WASM
(async () => {
	const wasm = await init()
	const gameManager = new GameManager(wasm)
	gameManager.run()
})()
