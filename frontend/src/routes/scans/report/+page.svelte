<!-- scans/report/+page.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { toast } from '@zerodevx/svelte-toast';
	import { decodeBase64, encodeBase64 } from '$lib/utils/base64';
	import { formatSize, buildFileTree, sortFileTree, type FileNode } from '$lib/utils/fileUtils';
	import Node from './Node.svelte';

	let sortBy: 'name' | 'size' = 'name';
  	let sortOrder: 'asc' | 'desc' = 'asc';

	let data: string[] = [];
	let fileTree: FileNode[] = [];
	let path: string | null;
	onMount(async () => {
		console.log('Loading scans');

		try {
			path = $page.url.searchParams.get('path');
			const encodedPath = path ? encodeURIComponent(path) : '';
			const response = await fetch(`/scans/report?path=${encodedPath}`);

			if (response.ok) {
				data = await response.json();
				console.log(data);
				fileTree = buildFileTree(data, path || '');
				console.log(fileTree);
			} else {
				throw new Error('Error fetching data');
			}
		} catch (error) {
			console.error('Error fetching data:', error);
			toast.push('Error fetching data');
		}
	});

	function handleSort() {
    fileTree = sortFileTree(fileTree, sortBy, sortOrder);
  }
</script>

<svelte:head>
	<title>Scans</title>
	<meta name="description" content="List of scans" />
</svelte:head>

<div class="p-6 max-w-sm mx-auto bg-white rounded-xl shadow-md flex flex-col items-center space-x-4">
	<h1 class="text-3xl font-bold text-gray-800">Scan Results for</h1>
	<h2 class="text-2xl font-bold text-gray-800">{decodeBase64(path)}</h2>
</div>

<div class="sort-options">
	<label for="sort-by">Sort By:</label>
	<select id="sort-by" bind:value={sortBy} on:change={handleSort}>
	  <option value="name">Name</option>
	  <option value="size">Size</option>
	</select>
  
	<label for="sort-order">Sort Order:</label>
	<select id="sort-order" bind:value={sortOrder} on:change={handleSort}>
	  <option value="asc">Ascending</option>
	  <option value="desc">Descending</option>
	</select>
  </div>
  
<ul class="mt-6 space-y-2">
	{#each fileTree as node}
		<li><Node {node} /></li>
	{/each}
</ul>
