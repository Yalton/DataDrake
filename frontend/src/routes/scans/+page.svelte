<!-- scans/+page.svelte -->

<script lang="ts">
	import { onMount } from 'svelte';
	import { Jumper } from 'svelte-loading-spinners';
	import { decodeBase64, encodeBase64 } from '$lib/utils/base64';
	import Toast from '$lib/components/Toast.svelte';

	let data: string[] = [];
	let selectedDirectory: string = '';
	let toastMessage: string = '';
	let toastType: 'success' | 'error' | 'info' | 'warning' = 'info';
	let scanDataPromise: Promise<string[]>;

	async function fetchScanData(): Promise<string[]> {
		try {
			const response = await fetch('/api/scans');
			if (response.ok) {
				const data = await response.json();
				// //console.log(data);
				showToast('Scans Fetched', 'info');
				return data;
			} else {
				throw new Error('Error fetching data');
			}
		} catch (error) {
			console.error('Error fetching data:', error);
			showToast('Error fetching data', 'error');
			throw error;
		}
	}

	onMount(async () => {
		scanDataPromise = fetchScanData();
	});

	async function handleSubmit() {
		////console.log(selectedDirectory)
		try {
			const response = await fetch('/api/scans', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ path: selectedDirectory })
			});

			if (response.ok) {
				let message = await response.json();
				////console.log(message.job_id);
				showToast(`Scan Job create with ID: ${message.job_id}`, 'success');
				// Optionally, you can refresh the data after a successful POST request
				// const updatedData = await response.json();
				// data = updatedData;
			} else {
				throw new Error('Error scanning directory');
			}
		} catch (error) {
			console.error('Error scanning directory:', error);
			showToast('Error scanning directory', 'error');
		}
	}

	async function handleDelete(directory: string) {
		try {
			const response = await fetch('/api/scans', {
				method: 'DELETE',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ path: directory })
			});

			if (response.ok) {
				showToast(`Scan deleted for directory: ${directory}`, 'success');
				// Refresh the data after successful deletion
				const updatedData = await fetch('/scans');
				data = await updatedData.json();
			} else {
				throw new Error('Error deleting scan');
			}
		} catch (error) {
			console.error('Error deleting scan:', error);
			showToast('Error deleting scan', 'error');
		}
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

<div class="scans bg-white p-8 rounded-lg shadow-md">
	<h1 class="text-4xl font-bold mb-4 text-gray-800">Directory Scans</h1>
	<hr class="mb-4 border-gray-300" />
	<h2 class="text-3xl font-bold mb-4 text-gray-800">Scan new Directory</h2>

	<form class="mb-4 flex space-x-2">
		<input
			type="text"
			bind:value={selectedDirectory}
			placeholder="Enter directory path"
			class="flex-1 px-3 py-2 border rounded-md focus:outline-none focus:ring focus:ring-blue-500 focus:border-blue-500"
		/>
		<button
			type="button"
			on:click={handleSubmit}
			class="px-3 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600"
		>
			Scan Directory
		</button>
	</form>
	{#await scanDataPromise}
	<h2 class="text-2xl font-bold mb-4 text-gray-800 flex items-center justify-center">Loading Scans...</h2>
		<div class="flex items-center justify-center">
			<Jumper size="60" color="#2196f3" unit="px" duration="1s" />
		</div>
	{:then data}
		{#if data}
			<h2 class="text-3xl font-bold mb-4 text-gray-800">Scans List</h2>
			<p class="mb-4 text-gray-600">
				Creating a new scan of an existing directory will overwrite a previous one
			</p>
			<p class="mb-4 text-gray-600">
				<a href="/jobs" class="underline decoration-blue-500">Check the Jobs Page</a> to see the status
				of a ScanJob
			</p>

			<div class="flex items-center space-x-2 mb-4">
				<span class="font-bold text-gray-800">🗑:</span>
				<p class="text-gray-600">Delete Scan Data</p>
			</div>
			<hr class="mb-4 border-gray-300" />
			<ul class="mb-4 list-disc pl-6 space-y-2 text-gray-600">
				{#each data as currentScan}
					<li class="flex items-center justify-between">
						<a
							href="/scans/report?path={encodeURIComponent(currentScan)}"
							class="block mb-2 text-gray-600 hover:text-gray-800 underline decoration-blue-500"
							>{decodeBase64(currentScan)}</a
						>
						<button
							on:click={() => handleDelete(currentScan)}
							class=" text-red-500 hover:text-red-600"
						>
							<span>🗑</span>
						</button>
					</li>
				{/each}
			</ul>
		{:else}
			<div class="mb-4">
				<h2 class="text-3xl font-bold mb-4 text-gray-800">No Scans exist</h2>
				<p class="text-gray-600">Get Scanning!</p>
			</div>
		{/if}
	{:catch error}
		<p>Error loading scan data: {error.message}</p>
	{/await}
</div>

<Toast message={toastMessage} type={toastType} />
