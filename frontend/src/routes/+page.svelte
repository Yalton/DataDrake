<script>
    import welcome from '$lib/images/logo.png';
</script>

<svelte:head>
    <title>Home</title>
    <meta name="description" content="Svelte demo app" />
</svelte:head>

<section class="flex flex-col justify-center items-center h-screen">
    <h1 class="text-3xl font-bold underline">
        <span class="block relative w-full pb-[calc(100%*4/3)]">
            <picture>
                <source srcset={welcome} type="image/webp" />
                <img src={welcome} alt="Welcome" class="absolute w-full h-full top-0 block"/>
            </picture>
        </span>
        to your new<br />SvelteKit app
    </h1>

    <h2 class="text-3xl font-bold underline bg-emerald-500">Penis - Cock even</h2>
</section>

<!-- <style lang="postcss">
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		display: block;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
	}
</style> -->
