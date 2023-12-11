<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { open } from "@tauri-apps/api/dialog";
    import { listen } from "@tauri-apps/api/event";
    import Hashstatus from "../components/Hashstatus.svelte";
    import Fade from "../components/utils/animations/Fade.svelte";
    import Slide from "../components/utils/animations/Slide.svelte";
    import Output from "../components/utils/Output.svelte";
    import GoToMain from "../components/GoToMain.svelte";

    let wordlistFilePath = "";
    let cmpHash = "";
    let loadStatus = "";
    let crackStatus = "";

    async function loadDictFile() {
        try {
            const selectedPath = await open({
                multiple: false,
                filters: [
                    {
                        name: "",
                        extensions: ["txt"],
                    },
                ],
                title: "Open Dict File",
            });

            if (!selectedPath) return;
            wordlistFilePath = selectedPath;
        } catch (error) {
            console.log(error);
        }
    }

    async function decode() {
        await invoke("dict", { file_path: wordlistFilePath, cmp_hash: cmpHash });
    }

    async function logLoaded() {
        await listen("wordlist-loaded", (event) => {
            loadStatus = event.payload.message;
        });
    }

    async function logCrack() {
        await listen("dict-cracked", (event) => {
            crackStatus = event.payload.message;
        });
    }

    $: logLoaded(), logCrack();
</script>

<main class="container">
    <GoToMain/>
    
    <h1>HASHGRINDER</h1>

    <p>Input hash and choose word-list file:</p>

    <div class="col">
        <form class="row">
            <input
                style="width: 70rem"
                placeholder="Dictionary path..."
                bind:value={wordlistFilePath}
            />
            <button class="btn" on:click|preventDefault={() => loadDictFile()}>Load</button>
        </form>
        <form class="row">
            <input
                style="width: 70rem"
                placeholder="Enter a hash..."
                on:input={(e) => {}}
                bind:value={cmpHash}
            />
            <button class="btn" on:click|preventDefault={() => decode()}>Decode hash</button>
        </form>
        <div class="row">
            <Hashstatus bind:hashValue={cmpHash}></Hashstatus>
        </div>
        <div class="row">
            <Fade>
                <Slide>
                    <Output type="warning" fontSize="18px" text={loadStatus} />
                </Slide>
            </Fade>
        </div>
        <div class="row">
            <Fade>
                <Slide>
                    <Output type="info" fontSize="18px" text={crackStatus} />
                </Slide>
            </Fade>
        </div>
    </div>
</main>
