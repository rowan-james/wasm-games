import { Wall } from '../build/maze_war.js'
import match from './utils/match.js'
import { debounce } from './utils/timer.js'

const getRange = length => [...Array(length).keys()]
const getIndex = width => (row, column) => row * width + column

const isBitSet = (n, arr) => {
	const byte = Math.floor(n / 8)
	const mask = 1 << (n % 8)
	return (arr[byte] & mask) === mask
}

const hasWall = (room, wall) => room[wall]
export class Renderer {
	constructor({ width, height, container = document.querySelector('.container'), onChange = () => { } } = {}) {
		this.width = width
		this.height = height
		this.container = container

		this.initialize()

		window.addEventListener('resize', debounce(() => {
			this.destroy()
			this.initialize()
			onChange()
		}, 100))
	}

	initialize() {
		const { width, height } = this.container.getBoundingClientRect()

		this.screenUnits = Math.min(
			width / this.width,
			height / this.height
		)

		const canvas = document.createElement('canvas')
		this.context = canvas.getContext('2d')

		canvas.setAttribute('width', this.projectDistance(this.width))
		canvas.setAttribute('height', this.projectDistance(this.height))
		this.container.appendChild(canvas)
	}

	projectDistance(distance) {
		return this.screenUnits * distance
	}

	clear() {
		this.context.clearRect(0, 0, this.context.canvas.width, this.context.canvas.height)
	}

	render(maze) {
		this.clear()
		this.drawBackground()
		this.drawMaze(maze)
	}

	drawBackground() {
		this.context.globalAlpha = 0.2
		this.context.fillStyle = 'black'
		getRange(this.width).forEach(column =>
			getRange(this.height)
				.filter(row => (column + row) % 2 === 1)
				.forEach(row =>
					this.context.fillRect(
						column * this.unitOnScreen,
						row * this.unitOnScreen,
						this.unitOnScreen,
						this.unitOnScreen
					)
				)
		)
	}

	getIndex(row, column) {
		return row * this.width
	}

	drawMaze(maze) {
		// console.log('WALL', Wall)
		this.context.globalAlpha = 1
		this.context.fillStyle = 'black'
		// console.log('Drawing maze', maze)
		const u = this.screenUnits

		getRange(this.width).forEach(column =>
			getRange(this.height)
				.forEach(row => {
					const idx = this.getIndex(row, column)
					console.log(`Index ${idx} is ${isBitSet(idx, maze)}`)

					// if (isBitSet(idx, maze)) {
					// 	const x = this.projectDistance(column)
					// 	const y = this.projectDistance(row)

					// 	const [from, to] = match(idx % 4)
					// 		.on(w => w === Wall.North, () => [[x, y], [x + u, y]])
					// 		.on(w => w === Wall.South, () => [[x, y + u], [x + u, y]])
					// 		.on(w => w === Wall.East, () => [[x + u, y], [x, y + u]])
					// 		.on(w => w === Wall.West, () => [[x, y], [x, y + u]])
					// 		.otherwise(() => [])
					// 	console.log('from', from, 'to', to)

					// 	this.context.beginPath()
					// 	this.context.moveTo(from[0], from[1])
					// 	this.context.lineTo(to[0], to[1])
					// 	this.context.stroke()
					// }
					if (isBitSet(idx, maze) && idx % 4) {
						this.context.beginPath()
						const x = this.projectDistance(column)
						const y = this.projectDistance(row)
						this.context.moveTo(x, y)
						this.context.lineTo(x + u, y)
						this.context.stroke()
					}
				})
		)
	}
}

/*
North {
	move_to x, y
	draw x + unit, y
}

South {
	move_to x, y + unit
	draw x + unit, y
}

East {
	move_to x + unit, y
	draw x, y + unit
}

West {
	move_to x, y
	draw x, y + unit
}
*/