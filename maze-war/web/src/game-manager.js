import { Game, Wall } from '../build/maze_war.js'
import config from '../config.js'
const { width, height } = config
import { Renderer } from './renderer.js'

export class GameManager {
	constructor(wasm) {
		console.log('Constructing')
		this.restart()
		this.renderer = new Renderer({
			width,
			height,
			onChange: this.render.bind(this)
		})

		const mazePtr = this.game.maze_as_ptr()
		this.maze = new Uint8Array(wasm.memory.buffer, mazePtr, config.width * config.height / 8)
	}

	restart() {
		console.log('Initializing Game')
		this.game = new Game(width, height)
		this.game.start()
	}

	run() {
		this.render()
	}

	render() {
		this.renderer.render(this.maze)
	}
}
