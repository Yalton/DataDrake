<svelte:head>
	<title>About DataDrake</title>
	<meta name="description" content="About DataDrake app" />
	<link
		href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.16/dist/tailwind.min.css"
		rel="stylesheet"
	/>
</svelte:head>

<div class="text-column bg-gray-100 p-8 rounded shadow-md">
	<h1 class="text-3xl font-semibold mb-4 text-gray-800">About DataDrake</h1>
	<p class="mb-4 text-gray-700">
		Data Drake is an application with a simple purpose: to provide persistent, on-demand file indexing. Inspired by the GNOME Disk Utility, but with the added benefit of persistent storage. 
	</p>
	<p class="mb-4 text-gray-700">
		Data Drake allows you to scan directories and determine, with as much or as little granularity as you like, what is taking up all your space.
	</p>
	<p class="mb-4 text-gray-700">
		Data Drake is built using Sveltekit for the frontend and Rust for the backend, providing a fast and efficient user experience.
	</p>
	<h2 class="text-2xl font-semibold mb-4 text-gray-800">Features</h2>
	<ul>
		<li>
			Persistent storage of file indexing results
		</li>
		<li>
			Customizable scanning granularity
		</li>
		<li>
			Intuitive and user-friendly interface
		</li>
		<li>
			Fast and efficient performance
		</li>
		<li>
			Cross-platform compatibility		
		</li>
	</ul>
</div>