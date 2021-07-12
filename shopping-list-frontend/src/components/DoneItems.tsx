import { Item } from "../Types"
import DoneItem from "./DoneItem"

type Props = {
    undoItem: (item: Item) => void
    items: Item[]
}
const DoneItems = ({ undoItem, items }: Props) => {
    return <div>
        {items.map(i => <DoneItem key={i.id} item={i} undoItem={() => undoItem(i)} />)}
    </div>
}

export default DoneItems
