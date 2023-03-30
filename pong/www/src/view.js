import { debounce } from './utils/timer.js'
import { roundRect } from './utils/canvas-helpers.js'

const range = length => [...Array(length).keys()]
const log = (...args) => console.log('[View]', ...args)

const calculateScreenUnits = ({ width, height, container }) => {
	log(`calculateScreenUnits(${width}, ${height}, ${container})`)

	const rect = container.getBoundingClientRect()
	log(`calculateScreenUnits: rect (${rect.width}, ${rect.height})`)
	return Math.min(
		rect.width / width,
		rect.height / height
	)
}

const configureCanvas = ({ width, height }) => {
	const canvas = document.createElement('canvas')

	log(`configureCanvas(${width}, ${height})`)
	canvas.setAttribute('width', width)
	canvas.setAttribute('height', height)

	return canvas
}

const isEven = n => n % 2 == 1

const ScreenRenderer = ({ context }) => () => {
	context.clearRect(0, 0, context.canvas.width, context.canvas.height)
	context.globalAlpha = 1
	context.fillStyle = '#111'
	context.fillRect(0, 0, context.canvas.width, context.canvas.height)

}

const PaddleRenderer = ({ context, units }) => game => {
	context.globalAlpha = 1
	context.fillStyle = 'white'

	context.fillRect(
		game.left_paddle.bounds.position.x * units,
		game.left_paddle.bounds.position.y * units,
		game.left_paddle.bounds.width * units,
		game.left_paddle.bounds.height * units
	)

	context.fillRect(
		game.right_paddle.bounds.position.x * units,
		game.right_paddle.bounds.position.y * units,
		game.right_paddle.bounds.width * units,
		game.right_paddle.bounds.height * units
	)
}

const BallRenderer = ({ context, units }) => game => {
	context.globalAlpha = 1
	context.fillStyle = 'white'

	context.fillRect(
		game.ball.bounds.position.x * units,
		game.ball.bounds.position.y * units,
		game.ball.bounds.width * units,
		game.ball.bounds.height * units
	)
}

const ScoreRenderer = ({ context }) => {
	const x = context.canvas.width / 4
	const x2 = context.canvas.width - x
	const y = context.canvas.height / 6

	return game => {
		context.globalAlpha = 1
		context.fillStyle = 'white'
		context.font = '72px "VT323"'
		context.fillText(game.left_paddle.score, x, y)
		context.fillText(game.right_paddle.score, x2, y)
	}
}

const NetRenderer = ({ context, width, height, units }) => {
	const x = context.canvas.width / 2
	const size = [units, units]

	return game => {
		context.globalAlpha = 1
		context.strokeStyle = 'white'
		context.setLineDash(size)
		context.lineWidth = units
		context.beginPath()
		context.moveTo(x, 0)
		context.lineTo(x, context.canvas.height)
		context.stroke()
	}
}

const StartRenderer = ({ context }) => {
	const centerX = context.canvas.width / 2
	const centerY = context.canvas.height / 2

	const text = 'Press Enter to start'
	const height = 72
	const fontStyle = `${height}px "VT323"`

	context.font = fontStyle

	const size = context.measureText(text)
	const textX = centerX - (size.width / 2)
	const textY = centerY

	return game => {
		context.globalAlpha = 1
		context.fillStyle = 'white'
		context.font = fontStyle
		context.fillText(text, textX, textY)
	}
}

const View = ({ width, height, container = document.querySelector('#container') }) => {
	log(`Constructor(${width}, ${height}, ${container})`)
	const units = calculateScreenUnits({ width, height, container })
	const canvas = configureCanvas({ width: width * units, height: height * units })
	container.appendChild(canvas)
	const context = canvas.getContext('2d')

	const destroy = () => {
		container.removeChild(canvas)
	}

	const view = { context, width, height, units }

	const renderScreen = ScreenRenderer(view)
	const renderPaddles = PaddleRenderer(view)
	const renderBall = BallRenderer(view)
	const renderScore = ScoreRenderer(view)
	const renderNet = NetRenderer(view)
	const renderStart = StartRenderer(view)
	const render = game => {
		renderScreen(game)
		if (!game.started) {
			renderStart(game)
		} else {
			renderPaddles(game)
			renderBall(game)
			renderScore(game)
			renderNet(game)
		}
		// log(`render(${game})`)
	}

	return {
		size: { width, height },
		context,
		render,
		destroy
	}
}

export default ({ onViewChange = () => { }, ...args }) => {
	let view = View(args)

	window.addEventListener('resize', debounce(e => {
		view.destroy()
		view = View(args)
		onViewChange()
	}, 200))

	return {
		size: () => view.size,
		context: () => view.context,
		render: (...args) => view.render(...args)
	}
}
