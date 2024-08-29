<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  let name = '';
  let uri  = '';

  function add() {
    if (!name.trim() || !uri.trim()) return;
    dispatch('add', { name, uri });
  }

  function cancel() {
    dispatch('cancel');
  }
</script>

<div class="fixed inset-0 bg-black/50 h-full  flex items-center justify-center z-50">
  <div class="bg-white rounded-lg shadow-lg p-6 w-full max-w-md">
    <h2 class="text-xl font-semibold mb-4">Add Torrent</h2>

    <div class="space-y-4">
      <div>
        <label class="block text-sm font-medium mb-1">Name</label>
        <input
          type="text"
          bind:value={name}
          placeholder="e.g. Ubuntu ISO"
          class="w-full border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring"
        />
      </div>

      <div>
        <label class="block text-sm font-medium mb-1">Magnet URI</label>
        <input
          type="text"
          bind:value={uri}
          placeholder="magnet:?xt=urn:btih:..."
          class="w-full border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring"
        />
      </div>
    </div>

    <div class="mt-6 flex justify-end space-x-2">
      <button
        on:click={cancel}
        class="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
      >Cancel</button>
      <button
        on:click={add}
        class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700"
      >Add</button>
    </div>
  </div>
</div>
