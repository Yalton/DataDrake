<!-- scans/report/+page.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import Toast from '$lib/components/Toast.svelte';
	import { toast } from '@zerodevx/svelte-toast';
	import { decodeBase64, encodeBase64 } from '$lib/utils/base64';
	import { Jumper } from 'svelte-loading-spinners';
	import { formatSize, buildFileTree, sortFileTree, type FileNode } from '$lib/utils/fileUtils';
	import Node from './Node.svelte';

	let sortBy: 'name' | 'size' = 'size';
	let sortOrder: 'asc' | 'desc' = 'desc';
	let toastMessage: string = '';
	let toastType: 'success' | 'error' | 'info' | 'warning' = 'info';
	let data: string[] = [];
	let fileTree: FileNode[] = [];
	let path: string | null;
	let scanDataPromise: Promise<void>;

	async function fetchScanData() {
		try {
			path = $page.url.searchParams.get('path');
			const encodedPath = path ? encodeURIComponent(path) : '';
			const response = await fetch(`/api/scans/report?path=${encodedPath}`);

			if (response.ok && response.body) {
				const reader = response.body.getReader();
				const decoder = new TextDecoder('utf-8');
				let buffer = '';

				while (true) {
					const { done, value } = await reader.read();
					if (done) break;

					buffer += decoder.decode(value);
					let position;
					while ((position = buffer.indexOf('\n')) !== -1) {
						const chunk = buffer.slice(0, position);
						buffer = buffer.slice(position + 1);

						try {
							const parsedData = JSON.parse(chunk);
							data.push(parsedData);
						} catch (error) {
							console.error('Error parsing JSON:', error);
							// Handle the parsing error, e.g., skip the invalid chunk
						}
					}
				}

				fileTree = buildFileTree(data, path || '');
				showToast('Scan Report Fetched', 'info');
			} else {
				throw new Error('Error fetching data');
			}
		} catch (error) {
			console.error('Error fetching data:', error);
			showToast('Error fetching Data', 'error');
		}
	}

	onMount(() => {
		scanDataPromise = fetchScanData();
	});
	function handleSort() {
		fileTree = sortFileTree(fileTree, sortBy, sortOrder);
	}

	function showToast(message: string, type: 'success' | 'error' | 'info' | 'warning') {
		toastMessage = message;
		toastType = type;

		setTimeout(() => {
			toastMessage = '';
		}, 3000);
	}
</script>

<svelte:head>
	<title>Scans</title>
	<meta name="description" content="List of scans" />
</svelte:head>

<div class="p-6 mx-auto bg-white rounded-xl shadow-md flex flex-col items-center space-x-4">
	<h1 class="text-3xl font-bold text-gray-800">Scan Results for</h1>
	<h2 class="text-2xl font-bold text-gray-800">{decodeBase64(path)}</h2>
</div>

{#await scanDataPromise}
	<div class="flex items-center justify-center mt-6">
		<Jumper size="60" color="#2196f3" unit="px" duration="1s" />
	</div>
{:then}
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
{:catch error}
	<p class="mt-6 text-red-500">Error loading scan data: {error.message}</p>
{/await}

<Toast message={toastMessage} type={toastType} />
