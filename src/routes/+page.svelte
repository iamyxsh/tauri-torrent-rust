<script lang="ts">
  import { writable } from 'svelte/store';
  import {
    Play,
    Pause,
    Users,
    FileText,
    Clock,
    Plus,
    UploadCloud,
    Trash2
  } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core'
  import { onMount } from 'svelte'

  export const torrents = writable<Torrent[]>([]);

  export async function loadTorrents() {
    const list: Torrent[] = await invoke('get_torrents');

    torrents.set(list as any);
  }

  onMount(() => {
    loadTorrents();
  });

  async function togglePause(id: number) {
    const torrent: Torrent = await invoke("get_torrent_by_id", {id});

    if (torrent.status == 'paused') {
      await invoke('resume_torrent', { id });
    } else {
      await invoke('pause_torrent', { id });
    }

    await loadTorrents();
  }

  function removeTorrent(id: number) {
    if (confirm('Remove this torrent?')) {
      torrents.update(list => list.filter(t => t.id !== id));
    }
  }

  function addTorrent() {
    alert('Add Torrent clicked');
  }

  function uploadFile() {
    alert('Upload File clicked');
  }
</script>

<main class="p-6 max-w-4xl mx-auto space-y-6">
  <div class="flex justify-between items-center">
    <h1 class="text-2xl font-bold">Torrent Client</h1>
    <div class="flex space-x-2">
      <button
        on:click={addTorrent}
        class="flex items-center px-4 py-2 bg-white border border-gray-200 rounded hover:bg-gray-50"
      >
        <Plus class="w-5 h-5 mr-2"/> Add Torrent
      </button>
      <button
        on:click={uploadFile}
        class="flex items-center px-4 py-2 bg-white border border-gray-200 rounded hover:bg-gray-50"
      >
        <UploadCloud class="w-5 h-5 mr-2"/> Upload File
      </button>
    </div>
  </div>

  <div class="space-y-4">
    {#each $torrents as t (t.id)}
      <div class="flex justify-between items-center bg-white border border-gray-100 rounded-lg p-4 shadow-sm">
        <div class="flex items-start space-x-4">
          <button
            on:click={() => togglePause(t.id)}
            class="p-2 rounded-full bg-green-100 hover:bg-green-200"
          >
            {#if t.status === 'paused'}
              <Play class="w-6 h-6 text-green-600"/>
            {:else}
              <Pause class="w-6 h-6 text-green-600"/>
            {/if}
          </button>

          <div class="min-w-0">
            <h2 class="font-medium truncate">{t.name}</h2>

            <div class="relative h-2 bg-gray-200 rounded-full mt-2 overflow-hidden">
              <div
                class="absolute top-0 left-0 h-full bg-green-500"
                style="width: {t.progress}%"
              ></div>
            </div>

            <div class="flex flex-wrap text-sm text-gray-500 mt-2 space-x-6">
              <span class="flex items-center">
                <Users class="w-4 h-4 mr-1"/> {t.peers}
              </span>
              <span class="flex items-center">
                <FileText class="w-4 h-4 mr-1"/> {t.downloaded} / {t.total}
              </span>
              <span class="flex items-center">
                <Clock class="w-4 h-4 mr-1"/> {t.eta}
              </span>
              {#if t.downSpeed}
                <span class="flex items-center">
                  <svg class="w-4 h-4 mr-1"><path d="M3 10l4-4 4 4M7 6v12"/></svg>
                  {t.downSpeed}
                </span>
              {/if}
            </div>
          </div>
        </div>

        <div class="flex items-center space-x-2">
          <button
            on:click={() => togglePause(t.id)}
            class="p-2 bg-green-600 text-white rounded hover:bg-green-700"
          >
            {#if t.status === 'paused'}
              <Play class="w-5 h-5"/>
            {:else}
              <Pause class="w-5 h-5"/>
            {/if}
          </button>
          <button
            on:click={() => removeTorrent(t.id)}
            class="p-2 bg-red-100 rounded hover:bg-red-200"
          >
            <Trash2 class="w-5 h-5 text-red-600"/>
          </button>
        </div>
      </div>
    {/each}
  </div>
</main>
