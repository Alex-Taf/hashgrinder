<script>
  import { Router, Route } from "svelte-navigator";
  import Main from "./views/Main.svelte";
  import Hash from "./views/Hash.svelte";
  import Hashlist from "./views/Hashlist.svelte";
  import Fade from "./components/utils/animations/Slide.svelte";
</script>

<main class="container">
  <Fade>
    <div>
      <Router>
        <Route path="/" component={Main}></Route>
        <Route path="/hash" component={Hash}></Route>
        <Route path="/hashlist" component={Hashlist}></Route>
      </Router>
    </div>
  </Fade>
</main>
