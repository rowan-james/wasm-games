export const compose = (fn, ...funcs) => {
	return (...args) => {
		return funcs.reduce((acc, func) => func(acc), fn(...args))
	}
}

const generate = (fn, dependency = LazyIterable, descriptor) => {
	return Object.create(
		Object.assign({ [Symbol.iterator]: fn }, dependency),
		descriptor
	)
}

export const map = (fn, iterable) => {
	return generate(() => {
		const iterator = iterable[Symbol.iterator]()

		return {
			next: () => {
				const { done, value } = iterator.next()

				return ({ done, value: done ? undefined : fn(value) })
			}
		}
	})
}

export const reduce = (fn, seed, iterable) => {
	const iterator = iterable[Symbol.iterator]()
	let iterationResult
	let accumulator = seed
	do {
		iterationResult = iterator.next()
		accumulator = iterationResult.done ? accumulator : fn(accumulator, iterationResult.value)
	} while (!iterationResult.done)
	return accumulator
}

export const filter = (fn, iterable) => {
	return generate(() => {
		const iterator = iterable[Symbol.iterator]()

		return {
			next: () => {
				do {
					var { done, value } = iterator.next()
				} while (!done && !fn(value))
				return { done, value }
			}
		}
	})
}

export const until = (fn, iterable) => {
	return generate(() => {
		const iterator = iterable[Symbol.iterator]()

		return {
			next: () => {
				let { done, value } = iterator.next()
				done = done || fn(value)
				return ({ done, value: done ? undefined : value })
			}
		}
	})
}

export const first = (iterable) => {
	return iterable[Symbol.iterator]().next().value
}

const rest = (iterable) => {
	return generate(() => {
		const iterator = iterable[Symbol.iterator]()

		iterator.next()
		return iterator
	})
}

export const take = (numberToTake, iterable) => {
	return generate(() => {
		const iterator = iterable[Symbol.iterator]()
		let remainingElements = numberToTake

		return {
			next: () => {
				let { done, value } = iterator.next()

				done = done || remainingElements-- <= 0

				return ({ done, value: done ? undefined : value })
			}
		}
	})
}

export function* range(start = 0, end = Infinity, step = 1) {
    let iterationCount = 0
    for (let i = start; i < end; i += step) {
        iterationCount++
        yield i
    }
    return iterationCount
}

export const getValue = Array.from
