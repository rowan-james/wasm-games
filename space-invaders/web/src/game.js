import { Game } from '../build/space_invaders.js'
import { everyFrame } from './gameLoop.js'
import { createView, createRenderer } from './view.js'
import config from './config.js'

export const start = () => {
	const game = Game.new(
		config.width,
		config.height,
		config.speed
	)

	const container = document.querySelector('#app')

	const renderStream = Bacon.fromEvent(window, 'resize')
		.map(() =>
			createRenderer(
				createView(config.width, config.height, container)
			)
		)

	const loopStream = everyFrame().map(() => game.tick())

	renderStream.combine(loopStream, (render) => render(game)).onValue()

	// Hacky way to kickstart render stream with initial value
	window.dispatchEvent(new Event('resize'))
}
