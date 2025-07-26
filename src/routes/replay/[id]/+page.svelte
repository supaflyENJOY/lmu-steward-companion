<script>
  import { page } from "$app/stores";
  $: id = $page.params.id;

  import { invoke } from "@tauri-apps/api/core";
  import { save } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";
  import Card from "$lib/components/ui/card/card.svelte";
  import { Play, FileSpreadsheet, RefreshCw } from "@lucide/svelte";
  import { badgeVariants } from "$lib/components/ui/badge/index.js";

  let loading = false;
  let exportLoading = false;
  let error = "";
  let success = "";
  let exportError = "";
  let exportSuccess = "";

  // Google Sheets export state
  let sheetsExportLoading = false;
  let sheetsExportError = "";
  let sheetsExportSuccess = "";

  let contacts = [];
  let standings = [];
  let carToDriver = {};
  let viewedContacts = new Set(); // Track which contacts have been viewed
  async function playContactRow(contact, index) {
    viewedContacts.add(index); // Mark this contact as viewed
    viewedContacts = viewedContacts; // Trigger reactivity
    const playerId = contact.players[0];
    const et = contact.et;
    try {
      await invoke("play_contact", { replayIdx: id, playerId: playerId, et });
    } catch (e) {
      error = String(
        e && typeof e === "object" && "message" in e ? e.message : e
      );
      alert(error);
    }
  }

  // Play contact with a specific driver (for driver name clicks)
  async function playContactWithDriver(contact, index, playerId) {
    viewedContacts.add(index); // Mark this contact as viewed
    viewedContacts = viewedContacts; // Trigger reactivity
    const et = contact.et;
    try {
      await invoke("play_contact", { replayIdx: id, playerId: playerId, et });
    } catch (e) {
      error = String(
        e && typeof e === "object" && "message" in e ? e.message : e
      );
      alert(error);
    }
  }
  let contactsLoading = false;
  let contactsError = "";

  async function startReplay() {
    loading = true;
    error = "";
    success = "";
    try {
      await invoke("play_replay", { replayIdx: id });
      success = "Replay started successfully.";
    } catch (e) {
      error = String(
        e && typeof e === "object" && "message" in e ? e.message : e
      );
    } finally {
      loading = false;
    }
  }

  async function exportToExcel() {
    exportLoading = true;
    exportError = "";
    exportSuccess = "";
    try {
      const path = await save({
        title: "Export Contacts to Excel",
        defaultPath: "contacts.xlsx",
        filters: [{ name: "Excel Files", extensions: ["xlsx"] }],
      });
      if (!path) {
        exportError = "Export cancelled.";
        exportLoading = false;
        return;
      }
      await invoke("export_contacts_to_excel_command", { replayIdx: id, path });
      exportSuccess = `Contacts exported to ${path}.`;
    } catch (e) {
      exportError = String(
        e && typeof e === "object" && "message" in e ? e.message : e
      );
    } finally {
      exportLoading = false;
    }
  }
  // Export to Google Sheets
  async function exportToGoogleSheets() {
    sheetsExportLoading = true;
    sheetsExportError = "";
    sheetsExportSuccess = "";
    try {
      const url = await invoke("export_contacts_to_google_sheets_command", {
        replayIdx: id,
      });
      sheetsExportSuccess = url;
    } catch (e) {
      sheetsExportError = String(
        e && typeof e === "object" && "message" in e ? e.message : e
      );
    } finally {
      sheetsExportLoading = false;
    }
  }

  $: if (id) {
    loadContacts();
  }

  function etToTime(et) {
    et = Math.round(et);
    const minutes = Math.floor(et / 60);
    const seconds = et % 60;
    return `${minutes.toFixed(0).padStart(2, "0")}:${seconds.toFixed(0).padStart(2, "0")}`;
  }

  async function loadContacts() {
    contactsLoading = true;
    contactsError = "";
    viewedContacts.clear(); // Reset viewed contacts when reloading
    viewedContacts = viewedContacts; // Trigger reactivity
    try {
      const result = await invoke("get_contacts_for_replay", { replayIdx: id });
      contacts = result.contacts;
      standings = result.standings;
      carToDriver = {};
      for (const v of standings) {
        carToDriver[v.slotID] = v.driverName;
      }
    } catch (e) {
      contactsError = String(
        e && typeof e === "object" && "message" in e ? e.message : e
      );
      contacts = [];
    } finally {
      contactsLoading = false;
    }
  }
</script>

<main class="w-full min-h-[80vh] bg-background py-10 px-4">
  <Card class="w-full px-0 py-0 shadow-md">
    <div class="px-8 pt-8 pb-4">
      <h2 class="text-2xl font-bold mb-2 text-center">Replay Dashboard</h2>
      <p class="mb-4 text-center text-muted-foreground">
        Replay ID: <strong>{decodeURIComponent(id)}</strong>
      </p>
      <div class="mb-6 flex gap-4 justify-start">
        <button
          class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-9 px-4 py-2"
          on:click={startReplay}
          disabled={loading}
        >
          <Play class="size-4" />
          {#if loading}
            Starting...
          {:else}
            Start Replay
          {/if}
        </button>
        <button
          class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2"
          on:click={exportToExcel}
          disabled={exportLoading}
        >
          <FileSpreadsheet class="size-4" />
          {#if exportLoading}
            Exporting...
          {:else}
            Export to Excel
          {/if}
        </button>
        <button
          class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2"
          on:click={exportToGoogleSheets}
          disabled={sheetsExportLoading}
        >
          <FileSpreadsheet class="size-4" />
          {#if sheetsExportLoading}
            Creating Google Sheet...
          {:else}
            Export to Google Sheets
          {/if}
        </button>
        <button
          class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2"
          on:click={loadContacts}
          disabled={contactsLoading}
        >
          <RefreshCw class="size-4" />
          {#if contactsLoading}
            Reloading...
          {:else}
            Reload Contacts
          {/if}
        </button>
      </div>
      {#if error}
        <p class="text-destructive text-center mb-2">{error}</p>
      {/if}
      {#if success}
        <p class="text-success text-center mb-2">{success}</p>
      {/if}
      {#if exportError}
        <p class="text-destructive text-center mb-2">{exportError}</p>
      {/if}
      {#if exportSuccess}
        <p class="text-success text-center mb-2">{exportSuccess}</p>
      {/if}
      <h3 class="text-lg font-semibold mt-6 mb-2 text-center">Contacts</h3>
      {#if contactsLoading}
        <p class="text-muted-foreground text-center">Loading contacts...</p>
      {:else if contactsError}
        <p class="text-destructive text-center">{contactsError}</p>
      {:else if contacts.length === 0}
        <p class="text-muted-foreground text-center">
          No contacts found for this replay.
        </p>
      {:else}
        <div
          class="overflow-x-auto rounded-lg border border-accent/30 bg-accent/50 w-full"
        >
          <table class="w-full divide-y divide-accent/30">
            <thead>
              <tr class="bg-accent/80">
                <th
                  class="px-4 py-2 text-left text-xs font-semibold text-muted-foreground"
                  >#</th
                >
                <th
                  class="px-4 py-2 text-left text-xs font-semibold text-muted-foreground"
                  >Players</th
                >
                <th
                  class="px-4 py-2 text-left text-xs font-semibold text-muted-foreground"
                  >ET</th
                >
              </tr>
            </thead>
            <tbody>
              {#each contacts as contact, i}
                <tr
                  on:click={() => playContactRow(contact, i)}
                  class="cursor-pointer transition-colors {viewedContacts.has(i)
                    ? 'bg-emerald-500 text-white'
                    : 'hover:bg-accent/60'}"
                >
                  <td class="px-4 py-2">{i + 1}</td>
                  <td class="px-4 py-2">
                    {#each contact.players as playerId, playerIndex}
                      <button
                        class={`${badgeVariants({ variant: "outline" })} cursor-pointer hover:bg-primary hover:text-primary-foreground transition-colors duration-200 hover:border-primary`}
                        on:click={(e) => {
                          e.stopPropagation();
                          playContactWithDriver(contact, i, playerId);
                        }}
                      >
                        {carToDriver[playerId] || playerId}
                      </button>
                      {#if playerIndex < contact.players.length - 1}
                        <span class="mx-1"></span>
                      {/if}
                    {/each}
                  </td>
                  <td class="px-4 py-2">{etToTime(contact.et)}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
      <p class="mt-6 text-center text-muted-foreground">
        This is the dashboard for the selected replay. Additional functionality
        will be added here.
      </p>
    </div>
  </Card>
</main>
