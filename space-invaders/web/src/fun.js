export const pipe = (...args) => args.reduce((acc, el) => el(acc))
