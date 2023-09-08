<script lang="ts">
	import { get_search_suggestions, type SuggestedModel } from '$lib/model';
	import { Icon, MagnifyingGlass } from 'svelte-hero-icons';
	import { onMount } from 'svelte';

	let search_term = '';
	let search_suggestions: SuggestedModel[] = [];
	let ref: HTMLInputElement | null = null;

	$: get_search_suggestions(search_term).then(
		(new_suggestions) => (search_suggestions = new_suggestions)
	);

	onMount(() => {
		if (ref) {
			ref.focus();
		}
	});
</script>

<div class="flex items-center justify-center max-w-md p-10 mx-auto">
	<Icon src={MagnifyingGlass} class="w-5 h-5 text-gray-700" />
	<input
		class="w-full text-gray-700 bg-transparent border-0 border-b border-black outline-none peer focus:ring-0 focus:outline-0"
		type="text"
		placeholder="Search For a Model on Huggingface..."
		bind:value={search_term}
		bind:this={ref}
	/>
</div>

<div class="max-w-md mx-auto space-y-3">
	{#each search_suggestions as suggestion}
		<a href={`/model/download?model_id=${encodeURIComponent(suggestion.id)}`}>
			<div class="p-2 rounded-md shadow hover:variant-soft-surface">
				{suggestion.id}
			</div>
		</a>
	{/each}
</div>
