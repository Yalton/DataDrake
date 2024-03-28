<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let title = '';
	export let content = '';
	export let confirmText = 'Close';
	export let cancelText: string | null = 'Cancel';
	export let onConfirm: (() => void) | undefined;
	export let onCancel: (() => void) | undefined;
	let modalRef: HTMLDialogElement | undefined;

	const dispatch = createEventDispatcher();

	function handleConfirm() {
		if (onConfirm) {
			onConfirm();
		} else {
			hideModal();
		}
	}

	function handleCancel() {
		if (onCancel) {
			onCancel();
		} else {
			hideModal();
		}
	}

	function showModal() {
		document.body.style.overflow = 'hidden';
		if (modalRef) {
			modalRef.showModal();
		}
	}

	function hideModal() {
		document.body.style.overflow = 'unset';
		if (modalRef) {
			modalRef.close();
		}
	}
</script>

<dialog bind:this={modalRef} class="rounded-lg bg-white shadow-xl">
	<div class="p-6">
		<h2 class="text-xl font-medium text-gray-800 mb-4">{title}</h2>
		<p class="text-gray-700">{content}</p>
	</div>
	<div class="flex items-center p-6 space-x-2 bg-gray-100 border-t border-gray-200">
		{#if cancelText}
			<button on:click={handleCancel} class="px-4 py-2 font-medium rounded bg-gray-300 text-gray-700 hover:bg-gray-400 focus:outline-none">{cancelText}</button>
		{/if}
		<button on:click={handleConfirm} class="px-4 py-2 font-medium rounded bg-blue-500 text-white hover:bg-blue-600 focus:outline-none">{confirmText}</button>
	</div>
</dialog>

