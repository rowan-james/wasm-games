export const everyFrame = () => Bacon.fromBinder(sink => {
	const request =
		window.requestAnimationFrame ||
		window.webkitRequestAnimationFrame ||
		window.mozRequestAnimationFrame ||
		window.oRequestAnimationFrame ||
		window.msRequestAnimationFrame ||
		(f => window.setTimeout(f, 1000 / 60))

	let subscribed = true

	const handler = () => {
		if (subscribed) {
			sink()
			return request(handler)
		}
	}

	request(handler)

	return () => subscribed = false
})