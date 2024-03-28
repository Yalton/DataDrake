<!-- Node.svelte -->

<script lang="ts">
  import { formatSize, type FileNode } from '$lib/utils/fileUtils';

  export let node: FileNode;
  let expanded = false;

  function toggleExpanded() {
    expanded = !expanded;
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      toggleExpanded();
    }
  }
</script>

<div class="node">
  {#if node.isFile}
    <div class="file" tabindex="0" on:click={toggleExpanded} on:keydown={handleKeyDown}>
      
      <span class="icon">📄</span>
      <span class="name">{node.name}</span>
      <strong class="size">[{formatSize(node.size)}]</strong>
    </div>
  {:else}
    <button class="folder" on:click={toggleExpanded} on:keydown={handleKeyDown}>
      <span class="icon">{expanded ? '📁': '📂'}</span>
      <span class="icon">{expanded ? '▼' : '▶'}</span>
      <span class="name">{node.name}</span>
      <strong class="size ">[{formatSize(node.size)}]</strong>
    </button>
    {#if expanded && node.children}
      <ul class="children">
        {#each node.children as child}
          <li><svelte:self node={child} /></li>
        {/each}
      </ul>
    {/if}
  {/if}
</div>
<style>
  .node {
    margin-left: 20px;
  }

  .file {
    cursor: pointer;
  }

  .folder {
    cursor: pointer;
    background: none;
    border: none;
    text-align: left;
    padding: 0;
    font-size: inherit;
    font-family: inherit;
  }

  .children {
    margin-left: 20px;
  }
</style>