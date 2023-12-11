<script>
    /** @param {string} type */
    export let type;

    /** @param {string} color */
    export let color = "";

    /** @param {string} fontSize */
    export let fontSize = "";

    /** @param {number} fontSize */
    export let fontWeight = 700;

    /** @param {string} text */
    export let text;

    const validTypes = {
        success: {
            name: "success",
            color: "green"
        },
        warning: {
            name: "warning",
            color: "#d48806"
        },
        error: {
            name: "error",
            color: "red"
        },
        info: {
            name: "info",
            color: "blueviolet"
        },
        format: {
            name: "format",
            color: "black"
        }
    }
</script>

<div style="color: {color ? color : validTypes[type].color}; font-size: {fontSize === "" ? "12px" : fontSize}; font-weight: {fontWeight};">
    {text}
</div>

<style>
    div {
        margin-top: 14px;
    }
</style>
