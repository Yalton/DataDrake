<script>
	import Header from './Header.svelte';
	import Footer from './Footer.svelte';
	import Layout from '$lib/components/Layout.svelte';

	import './styles.css';
	import '../app.css';
</script>

<div>
	<Header />

	<main class="p-4 max-w-6xl mx-auto mt-10">
		<Layout>
			<slot />
		</Layout>
	</main>

	<Footer />
</div>
