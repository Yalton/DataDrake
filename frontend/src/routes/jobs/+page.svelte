<!-- /routes/jobs/+page.svelte -->

<script lang="ts">
	import { onMount } from 'svelte';
	import type { Job } from '$lib/types';
	import Toast from '$lib/components/Toast.svelte';


	let toastMessage: string = '';
	let toastType: 'success' | 'error' | 'info' | 'warning' = 'info';

	let jobs: Job[] = [];

	onMount(async () => {
		//console.log('Loading Jobs');
		try {
			const response = await fetch('/api/jobs');
			if (response.ok) {
				jobs = await response.json();
				showToast('Jobs Fetched', 'info');

				//console.log(jobs);
			} else {
				throw new Error('Error fetching jobs');
			}
		} catch (error) {
			console.error('Error fetching jobs:', error);
			showToast('Error fetching Jobs', 'error');

		}
	});
	function showToast(message: string, type: 'success' | 'error' | 'info' | 'warning') {
		toastMessage = message;
		toastType = type;

		setTimeout(() => {
			toastMessage = '';
		}, 3000);
	}
</script>

<svelte:head>
	<title>Jobs</title>
	<meta name="description" content="List of jobs" />
</svelte:head>

<div class="jobs overflow-auto">
	<h1 class="text-4xl font-bold mb-4">Jobs List</h1>
	<table class="w-full table-auto border-collapse text-left">
		<thead>
			<tr>
				<th class="px-4 py-2 border border-gray-300">ID</th>
				<th class="px-4 py-2 border border-gray-300">Root Path</th>
				<th class="px-4 py-2 border border-gray-300">State</th>
				<th class="px-4 py-2 border border-gray-300">Created At</th>
			</tr>
		</thead>
		<tbody>
			{#each jobs as job}
				<tr>
					<td class="border border-gray-300 px-4 py-2">{job.id}</td>
					<td class="border border-gray-300 px-4 py-2">{job.root_path}</td>
					<td class="border border-gray-300 px-4 py-2">{Object.keys(job.state)[0]}</td>
					<td class="border border-gray-300 px-4 py-2"
						>{new Date(job.created_at).toLocaleString()}</td
					>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
<Toast message={toastMessage} type={toastType} />
