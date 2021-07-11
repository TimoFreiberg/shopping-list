import { FormEvent, useState } from "react"
import { Item } from "../Types"

type Props = {
    item: Item
    finishItem: () => void
    editItem: (id: number, newName: string) => void
    deleteItem: (id: number) => void
}
const OpenItem = ({ item, finishItem, editItem }: Props) => {
    const [editState, setEditState] = useState(false)
    const [newName, setNewName] = useState(item.name)
    if (editState) {
        const submitEdit = (e: FormEvent) => {
            e.preventDefault()
            editItem(item.id, newName)
            setEditState(false)
        }
        const abortEdit = (e: FormEvent) => {
            e.preventDefault()
            setNewName(item.name)
            setEditState(false)
        }
        return <form onSubmit={submitEdit}>
            <input
                value={newName}
                onChange={(e) => setNewName(e.target.value)}>
            </input>
            <button type='submit'>💾</button>
            <button onClick={abortEdit}>❌</button>
        </form>
    }
    return <p>
        <button onClick={finishItem}>✔️</button>
        {item.name}
        <button onClick={() => setEditState(true)}>✏️</button>
    </p>
}

export default OpenItem
