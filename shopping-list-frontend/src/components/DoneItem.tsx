import { Item } from "../Types"

type Props = {
    item: Item
    undoItem: () => void
}
const DoneItem = ({ item, undoItem }: Props) => <p>
    <button onClick={undoItem}>🔙</button>
    <del>{item.name}</del>
</p>

export default DoneItem
