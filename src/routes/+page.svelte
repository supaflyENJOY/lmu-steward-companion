<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import Card from "$lib/components/ui/card/card.svelte";
  import Button from "$lib/components/ui/button/button.svelte";

  /** @type {any[]} */
  let matchedReplays = $state([]);
  let loadingReplays = $state(true);
  let replaysError = $state("");

  onMount(async () => {
    try {
      matchedReplays = await invoke("get_matched_replays");
      matchedReplays.sort((a, b) => a.replay.id - b.replay.id);
    } catch (e) {
      replaysError = String(
        e && typeof e === "object" && "message" in e ? e.message : e
      );
    } finally {
      loadingReplays = false;
    }
  });

  /**
   * @param {number} ts
   */
  function formatTimestamp(ts) {
    if (!ts) return "";
    if (ts < 1e12) ts = ts * 1000;
    const d = new Date(ts);
    return d.toLocaleString();
  }
</script>

<main class="w-full min-h-[80vh] bg-background py-10 px-4">
  <div>
    <h2 class="text-2xl font-bold mb-6 text-center">List of Replays</h2>
    {#if loadingReplays}
      <p class="text-muted-foreground text-center">Loading replays...</p>
    {:else if replaysError}
      <p class="text-destructive text-center">{replaysError}</p>
    {:else if matchedReplays.length === 0}
      <p class="text-muted-foreground text-center">No matched replays found.</p>
    {:else}
      <ul
        class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4"
      >
        {#each matchedReplays as replay}
          <li>
            <Card
              class="bg-accent/50 border-accent/30 hover:shadow-md transition-shadow h-full flex flex-col"
            >
              <a
                href={`/replay/${encodeURIComponent(replay.replay.id)}`}
                class="flex flex-col gap-1 px-6 py-4 no-underline hover:bg-accent/80 rounded-xl transition-colors h-full"
              >
                <span class="font-semibold text-lg"
                  >{replay.replay.file_name}</span
                >
                <span class="text-xs text-muted-foreground"
                  >{formatTimestamp(replay.replay.modified_date)}</span
                >
              </a>
            </Card>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</main>
