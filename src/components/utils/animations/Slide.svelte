<script>
    import { slide } from "svelte/transition";
</script>

<div transition:slide|global={{ delay: 100, duration: 200, axis: 'y' }}>
    <slot/>
</div>
