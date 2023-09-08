import { fetch } from '@tauri-apps/api/http'

export type SearchSuggestions = {
   models: SuggestedModel[]
}
export type SuggestedModel = {
   id: string
}

export const get_search_suggestions = async (key_word: string): Promise<SuggestedModel[]> => {
   const res = await fetch(`https://huggingface.co/api/quicksearch?q=${key_word}&type=all`)
   return (res.data as SearchSuggestions).models
}

export type HFModelInfo = {
   _id: string,
   id: string,
   private: boolean,
   disabled: boolean,
   gated: false,
   downloads: number,
   likes: 54,
   config: {
      architectures: string[]
      model_type: string
   },
   cardData: {
      license: string,
   },
   siblings: HFFileSiblings[]
}
export type HFFileSiblings = {
   "rfilename": string
}


export const get_model_info = async (model_id: string): Promise<HFModelInfo> => {
   const res = await fetch(`https://huggingface.co/api/models/${model_id}`) as any
   let model_info = res.data as HFModelInfo
   model_info.siblings = model_info.siblings.filter((sib) => sib.rfilename.includes("ggml"))
   return model_info
}


export type SupportedQuants = ""
