import { rmSync } from 'node:fs'

rmSync(
    new URL('../data/', import.meta.url),
    {
        recursive: true,
        force: true,
    },
)