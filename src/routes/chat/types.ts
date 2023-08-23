export type Chat = {
   id: number,
   title: string,
}

export type Message = {
   role: 'USER' | 'MODEL',
   content: string
}