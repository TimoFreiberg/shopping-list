import type { Item } from "../types"

interface Props {
    item: Item
    undoItem: () => void
}

export default function DoneItem({ item, undoItem }: Props) {
    return <p>
        <button onClick={undoItem}>🔙</button>
        <del>{item.name}</del>
    </p>
}
