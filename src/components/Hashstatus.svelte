<script>
    import { onDestroy } from "svelte";
    import { validHashLengths, hashStatus } from "../store/hash";
    import { checkHashLength } from "../utils";
    import Fade from "./utils/animations/Fade.svelte";
    import Slide from "./utils/animations/Slide.svelte";
    import Output from "./utils/Output.svelte";

    /** @param {string} hashValue */
    export let hashValue = "";

    const unsubscribe = hashStatus.subscribe((value) => console.log(value));

    function checkLength(value) {
        const newStatus = checkHashLength(value.length, $validHashLengths);
        hashStatus.set({
            status: newStatus.status,
            format: newStatus.format,
        });
    }

    onDestroy(() => {
        unsubscribe();
    });

    $: hashValue !== "" && checkLength(hashValue);
</script>

<div>
    {#if hashValue !== ""}
        {#if $hashStatus.status === "invalid"}
            <Fade>
                <Slide>
                    <Output type="error" fontSize="18px" text={$hashStatus.format} />
                </Slide>
            </Fade>
        {/if}
        {#if $hashStatus.status === "valid"}
            <Fade>
                <Slide>
                    <Output type="success" fontSize="18px" text={$hashStatus.format} />
                </Slide>
            </Fade>
        {/if}
    {/if}
</div>
