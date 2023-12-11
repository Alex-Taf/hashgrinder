<script>
    import { Link } from "svelte-navigator";
</script>

<div>
    <Link to="/">
        <svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg"
            ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
                id="SVGRepo_tracerCarrier"
                stroke-linecap="round"
                stroke-linejoin="round"
            ></g><g id="SVGRepo_iconCarrier">
                <rect width="48" height="48" fill="white" fill-opacity="0.01"
                ></rect>
                <path
                    d="M12 23.9917L36 23.9917"
                    stroke="#000000"
                    stroke-width="4"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                ></path>
                <path
                    d="M24 36L12 24L24 12"
                    stroke="#000000"
                    stroke-width="4"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                ></path>
            </g></svg
        >
    </Link>
</div>

<style>
    div {
        position: absolute;
        width: 60px;
        height: 60px;
        top: 40px;
        left: 20px;
        transition: .2s;
    }

    div:hover {
        transform: scale(1.3);
    }
</style>
