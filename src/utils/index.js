import { crossfade } from 'svelte/transition';
import { quintOut } from 'svelte/easing';

export const checkHashLength = (length, lengthsList) => {
    let hashKeyName = ""
    Object.entries(lengthsList).forEach(([key, value]) => {
        if (value === length) {
            hashKeyName = key
        }
    })

    if (hashKeyName === "") {
        return {
            status: "invalid",
            format: "Invalid hash length."
        }
    } else {
        return {
            status: "valid",
            format: `Loaded ${hashKeyName} hash.`
        }
    }
}

export const [send, receive] = crossfade({
	duration: (d) => Math.sqrt(d * 200),

	fallback(node, params) {
		const style = getComputedStyle(node);
		const transform = style.transform === 'none' ? '' : style.transform;

		return {
			duration: 600,
			easing: quintOut,
			css: (t) => `
				transform: ${transform} scale(${t});
				opacity: ${t}
			`
		};
	}
});
