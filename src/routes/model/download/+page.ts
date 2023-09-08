import type { PageLoad } from './$types'
import { get_model_info } from '$lib/model'

export const load = (async ({ url }) => {
   let id = url.searchParams.get("model_id") as string
   return get_model_info(id)
}) satisfies PageLoad