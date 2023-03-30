import { Movement } from '/pkg/pong.js'

const Keys = {
	[Movement.Up]: ['KeyW', 'ArrowUp'],
	[Movement.Down]: ['KeyD', 'ArrowDown']
}

const PauseKey = 'Space'
const StartKey = 'Enter' // Enter

const findKey = input => Object.keys(Keys).find(key => Keys[key].includes(input))

const log = (...args) => console.log('[Controller]', ...args)

export default function ({ onInput = () => { }, onStart = () => { }, onPause = () => { } } = {}) {
	this.input = null

	window.addEventListener('keydown', ({ code }) => {
		const input = findKey(code)
		if (input && this.input !== input) {
			this.input = input
			onInput(this.input)
		}
	})

	window.addEventListener('keyup', ({ code }) => {
		const input = findKey(code)
		if (input && this.input === input) {
			this.input = null
		}

		if (code == StartKey)
			onStart()
		else if (code == PauseKey)
			onPause()
	})
}
