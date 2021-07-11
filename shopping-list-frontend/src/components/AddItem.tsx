import { FormEvent, useState } from "react"
import { Item } from "../Types"

type Props = {
    addItem: (item: Item) => void
}
const AddItem = ({ addItem }: Props) => {
    const [newName, setNewName] = useState('')
    const createItem = (e: FormEvent) => {
        e.preventDefault()
        const newItem: Item = { id: 0, createdAt: new Date(), name: newName }
        addItem(newItem)
        setNewName('')
    }

    const inputClassName = newName.length === 0 ? 'emptyNewName' : ''
    return <div>
        <form onSubmit={createItem}>
            <input
                className={inputClassName}
                value={newName}
                onChange={(e) => setNewName(e.target.value)}
            />
            <button type="submit">add</button>
        </form>
    </div>
}

export default AddItem
