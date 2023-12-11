<script>
    import { Link } from "svelte-navigator";
</script>

<main class="container">
    <h1>HASHGRINDER</h1>

    <p>Select an option: </p>

    <section class="col">
        <div class="row" style="width: 300px;">
            <button class="btn"><Link to="/hash">Decode hash</Link></button>
        </div>
        <div class="row" style="width: 300px;">
            <button class="btn"><Link to="/hashlist">Decode list of hashes</Link></button>
        </div>
    </section>
</main>

<style>
    main {
        transition: .3s;
        transform: translateY(100);
    }

    p {
        font-size: 18px;
    }
</style>
