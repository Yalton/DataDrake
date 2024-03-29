<script>
	import 'tailwindcss/base.css';
	import 'tailwindcss/components.css';
	import 'tailwindcss/utilities.css';

	// Your existing imports
	import { page } from '$app/stores';
	import logo from '$lib/images/logo.png';
</script>

<header class="bg-transparent fixed w-full top-0 z-50">
	<div class="flex flex-col items-center py-4">
		<nav class="container mx-auto flex justify-between items-center px-4">
			<ul class="flex justify-center items-center h-8 list-none p-0 m-0 bg-transparent">
				<li aria-current={$page.url.pathname === '/' ? 'page' : undefined} class="h-full">
					<a
						href="/"
						class="flex items-center h-full px-2 text-sm font-bold uppercase tracking-wider no-underline transition-colors duration-200 ease-linear text-black hover:text-theme-1"
					>
						<img src={logo} width="60" height="60" alt="SvelteKit" />
					</a>
				</li>
				<li aria-current={$page.url.pathname === '/scans' ? 'page' : undefined} class="h-full">
					<a
						href="/scans"
						class="flex items-center h-full px-2 text-sm font-bold uppercase tracking-wider no-underline transition-colors duration-200 ease-linear text-black hover:text-theme-1"
						>Scans</a
					>
				</li>
				<li aria-current={$page.url.pathname === '/jobs' ? 'page' : undefined} class="h-full">
					<a
						href="/jobs"
						class="flex items-center h-full px-2 text-sm font-bold uppercase tracking-wider no-underline transition-colors duration-200 ease-linear text-black hover:text-theme-1"
						>Jobs</a
					>
				</li>
			</ul>
		</nav>

	</div>
</header>
