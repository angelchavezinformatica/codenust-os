export interface EntryI {
  value: string
  isShell: boolean
  isEditing: boolean
  path: string
  user: string
  process: (value: string) => Promise<void>
}
