export type Chat = {
   id: number,
   title: string,
}

export type Message = {
   id: number,
   role: 'USER' | 'MODEL',
   content: string
}