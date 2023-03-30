export function debounce(fn, timeout = 100) {
    let timer
    return (...args) => {
        const next = () => fn(...args)
        if (timer)
            clearTimeout(timer)
        timer = setTimeout(next, timeout > 0 ? timeout : 100)
    }
}
