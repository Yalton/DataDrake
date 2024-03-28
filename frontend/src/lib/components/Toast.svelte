<!-- Toast.svelte -->

<script lang="ts">
	import { onDestroy } from 'svelte';
	import { fly } from 'svelte/transition';
  
	export let message: string = '';
	export let duration: number = 3000;
	export let type: 'success' | 'error' | 'info' | 'warning' = 'info';
  
	let visible: boolean = false;
	let timeout: number;
  
	$: {
	  if (message) {
		showToast();
	  }
	}
  
	onDestroy(() => {
	  clearTimeout(timeout);
	});
  
	function showToast() {
	  visible = true;
	  clearTimeout(timeout);
	  timeout = setTimeout(() => {
		visible = false;
	  }, duration);
	}
  </script>
  
  {#if visible}
	<div class="toast" class:success={type === 'success'} class:error={type === 'error'} class:info={type === 'info'} class:warning={type === 'warning'} transition:fly="{{ y: 200, duration: 800 }}">
	  {message}
	</div>
  {/if}
  
  <style>
	.toast {
	  position: fixed;
	  bottom: 5rem;
	  right: 1rem;
	  padding: 1rem 2rem;
	  border-radius: 0.01rem;
	  color: white;
	  font-weight: bold;
	  z-index: 9999;
	}
  
	.success {
	  background-color: #4caf50;
	}
  
	.error {
	  background-color: #f44336;
	}
  
	.info {
	  background-color: #2196f3;
	}
  
	.warning {
	  background-color: #ff9800;
	}
  </style>