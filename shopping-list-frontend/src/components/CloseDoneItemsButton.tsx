type Props = {
    closeDoneItems: () => void
}
const CloseDoneItemsButton = ({ closeDoneItems }: Props) => <button onClick={ closeDoneItems}>[^]</button>

export default CloseDoneItemsButton
