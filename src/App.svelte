<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/dialog";
  import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';

  let filePath = "";
  let cmpHash = "";
  let hashStatus = {
    status: "unknown",
    format: "",
  }
  let result = "";

  const validHashLengths = {
    md5: 32,
    sha1: 40,
    sha256: 64,
    sha512: 128
  }

  const checkHashLength = (length) => {
    console.log(length)
    let hashKeyName = ""
    Object.entries(validHashLengths).forEach(([key, value]) => {
      if (value === length) {
        hashKeyName = key
      }
    })

    if (hashKeyName === "") {
      hashStatus.status = "invalid"
      hashStatus.format = "Invalid hash length."
    } else {
      hashStatus.status = "valid"
      hashStatus.format = `Loaded ${hashKeyName} hash.`
    } 
  } 

  async function loadDictFile() {
    try {
      const selectedPath = await open({
        multiple: false,
        filters: [{
          name: '',
          extensions: ['txt']
        }],
        title: "Open Dict File"
      })
      if (!selectedPath) return
      filePath = selectedPath
    } catch (error) {
        console.log(error)
    }
  }

  const decode = async () => {
    let decrypt = await invoke("start", { file_path: filePath, cmp_hash: cmpHash })
    if (decrypt) {
        result = await readTextFile('/saved/hashes.saved', { dir: BaseDirectory.AppConfig });
        console.log(result);
    }
  }
</script>

<main class="container">
  <h1>HASHGRINDER</h1>

  <p>Load a dict and input hash</p>

  <div class="row">
    <div>
      <form class="row">
        <input
          id="greet-input"
          placeholder="Dictionary path..."
          bind:value={filePath}
        />
        <button on:click|preventDefault={() => loadDictFile()}>Load</button>
      </form>
      <form class="row">
        <input
          id="custom-input"
          placeholder="Enter a hash..."
          on:input={(e) => checkHashLength(e.currentTarget.value.length)}
          bind:value={cmpHash}
        />
        <button type="submit" on:click|preventDefault={() => decode()}>Decode hash</button>
      </form>
      {#if (hashStatus.status === "invalid")}
        <h4 style="color: red;">{hashStatus.format}</h4>
      {/if}
      {#if (hashStatus.status === "valid")}
        <h4 style="color: green;">{hashStatus.format}</h4>
      {/if}
      <h3>{result}</h3>
    </div>
  </div>
</main>
