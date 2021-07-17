import type { Item } from "../types"
import DoneItem from "./DoneItem"

interface Props {
    undoItem: (item: Item) => void
    items: Item[]
}

export default function DoneItems({ undoItem, items }: Props) {
    return (
        <>
            {items.map((i, ix) => <DoneItem key={ix} item={i} undoItem={() => undoItem(i)} />)}
        </>
    )
}
