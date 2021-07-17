import { FormEvent, useState } from "react"
import type { Item } from "../types"

interface Props {
    addItem: (item: Item) => void
}

export default function AddItem({ addItem }: Props) {
    const [newName, setNewName] = useState('')
    const createItem = (e: FormEvent) => {
        e.preventDefault()
        const newItem: Item = { id: 0, createdAt: new Date(), name: newName }
        addItem(newItem)
        setNewName('')
    }

    const inputClassName = newName.length === 0 ? 'emptyNewName' : ''
    return (
        <form onSubmit={createItem}>
            <input
                className={inputClassName}
                value={newName}
                onChange={(e) => setNewName(e.target.value)} />
            <button type="submit">add</button>
        </form>
    )
}
