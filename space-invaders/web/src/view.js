export const createCanvas = (view) => {
	const [child] = view.container.children

	if (child) {
		view.container.removeChild(child)
	}

	const canvas = document.createElement('canvas')

	view.container.appendChild(canvas)
	const context = canvas.getContext('2d')

	canvas.setAttribute('width', view.projectDistance(view.width))
	canvas.setAttribute('height', view.projectDistance(view.height))

	return context
}

export const createView = (gameWidth, gameHeight, container) => {
	const { width, height } = container.getBoundingClientRect()

	// Preserve aspect ratio
	const unit = Math.min(
		width / gameWidth,
		height / gameHeight
	)
	const projectDistance = distance => distance * unit
	const projectPosition = position => position.scale_by(unit)

	const view = {
		unit,
		container,
		width: gameWidth,
		height: gameHeight,
		projectDistance,
		projectPosition
	}

	return {
		...view,
		context: createCanvas(view)
	}
}

const clearCanvas = ctx => ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height)

const getRange = length => [...Array(length).keys()]

export const createRenderer = view => state => {
	clearCanvas(view.context)

	// Temporary background
	view.context.globalAlpha = 0.2
	view.context.fillStyle = 'black'
	getRange(view.width).forEach(column =>
		getRange(view.height)
			.filter(row => (column + row) % 2 === 1)
			.forEach(row =>
				view.context.fillRect(
					column * view.unit,
					row * view.unit,
					view.unit,
					view.unit
				)
			)
	)
	view.context.globalAlpha = 1
}
