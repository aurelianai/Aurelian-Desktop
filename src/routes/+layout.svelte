<script lang="ts">
	import '@skeletonlabs/skeleton/themes/theme-crimson.css';
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	import '../app.postcss';

	import hljs from 'highlight.js';
	import 'highlight.js/styles/github-dark.css';
	import { storeHighlightJs } from '@skeletonlabs/skeleton';
	storeHighlightJs.set(hljs);

	import {
		computePosition,
		autoUpdate,
		offset,
		shift,
		flip,
		arrow
	} from '@floating-ui/dom';
	import { storePopup, setInitialClassState } from '@skeletonlabs/skeleton';
	setInitialClassState();
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	import Navbar from '$lib/Navbar.svelte';
</script>

<div class="fixed relative top-0 bottom-0 left-0 right-0">
	<div class="top-0 left-0 right-0">
		<Navbar />
	</div>

	<slot />
</div>
