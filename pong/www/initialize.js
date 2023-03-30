import GameManager from './src/game-manager.js'
import init from '/pkg/pong.js'

;(async () => {
	await init()

	const gameManager = GameManager()
	gameManager.run()
})()
