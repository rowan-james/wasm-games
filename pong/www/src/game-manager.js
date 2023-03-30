import { Game } from '/pkg/pong.js'
import Config from './config.js'
const { Width, Height, Speed, FPS } = Config

import View from './view.js'
import Controller from './controller.js'

const log = (...args) => console.log('[Game]', ...args)
const GameManager = () => {
	// log('Constructor')
	const game = new Game(Width, Height, Speed)
	const controller = new Controller({
		onStart: () => {
			if (!game.started)
				game.start()
		},
		onPause: () => {
			if (game.started) {
				paused = !paused
				if (!paused)
					lastUpdate = Date.now()
			}
		}
	})
	let lastUpdate
	let paused = false

	const view = View({
		width: Width,
		height: Height,
		onViewChange: () => render()
	})

	const render = () => {
		// log('render()')
		view.render(game)
	}

	const tick = () => {
		if (!paused) {
			const now = Date.now()

			if (lastUpdate) {
				const deltaTime = (now - lastUpdate) / 1000
				game.tick(deltaTime, controller.input)
			}

			lastUpdate = now
			render()
		}
	}

	const run = () => {
		// log('run()')
		setInterval(() => tick(), 1000 / FPS)
	}

	return {
		restart: GameManager,
		run
	}
}

export default GameManager
