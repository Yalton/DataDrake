<script>
    import welcome from '$lib/images/home-page.png';
</script>

<svelte:head>
    <title>Data Drake</title>
    <meta name="description" content="Data Drake - Persistent, on-demand file indexing application" />
    <!-- <link
        href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.16/dist/tailwind.min.css"
        rel="stylesheet"
    /> -->
</svelte:head>

<div class="text-column bg-gray-100 p-8 rounded shadow-md">
    <h1 class="text-3xl font-semibold mb-4 text-gray-800">Data Drake - Persistent, On-Demand File Indexing</h1>
    <img src={welcome} alt="Welcome" class="block w-full h-auto mb-6 rounded-lg"/>
    <p class="mb-4 text-gray-700">
        Data Drake is an application designed to provide persistent, on-demand file indexing with the added benefit of persistent storage. Inspired by GNOME Disk Utility, Data Drake enables you to scan directories and determine what's taking up space with customizable granularity. The app leverages Sveltekit for the frontend and Rust for the backend, offering fast and efficient performance across platforms.
    </p>
    <p class="mb-4 text-gray-700">
        Written in rust, so filescans are very quick, but for massive directories with deep subdirectory structures it might take a bit.
    </p>
    <h2 class="text-2xl font-semibold mb-4 text-gray-800">Features</h2>
    <ul style="list-style-type: disc;">
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

<style>
  .text-column {
    max-width: 800px;
    margin: auto;
  }

  ul {
    padding-left: 2rem;
  }
</style>