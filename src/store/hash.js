import { readonly, writable } from "svelte/store"

export const validHashLengths = readonly(writable({
    md5: 32,
    sha1: 40,
    sha256: 64,
    sha512: 128
}))

export const hashStatus = writable({})
