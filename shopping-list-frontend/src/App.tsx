import { useEffect, useState } from 'react';
import './App.css';
import { Item } from './Types'
import Header from './components/Header'
import OpenItemSection from './components/OpenItemSection'
import DoneItemSection from './components/DoneItemSection'
import axios from 'axios'

const now = () => new Date()

// const initialItems: Item[] = [
//   { id: 0, name: "rabbit food", createdAt: now() },
//   { id: 0, name: "soy milk", createdAt: now() },
//   { id: 0, name: "rice", createdAt: now() },
// ]
// const initialDoneItems: Item[] = [
//   { id: 0, name: "toilet paper", createdAt: now(), doneAt: now() },
//   { id: 0, name: "peanut butter", createdAt: now(), doneAt: now() },
// ]

function App() {
  const emptyItems: Item[] = []
  const [openItems, setOpenItems] = useState(emptyItems)
  const [doneItems, setDoneItems] = useState(emptyItems)
  useEffect(() => {
    axios.get('http://localhost:8000/items').then(resp => {
      console.log("getOpenItems response", resp)
      return setOpenItems(resp.data.items);
    })
  }, undefined)

  const addItem = (newItem: Item) => {
    console.log("creating item", newItem)
    axios.post('http://localhost:8000/items', newItem).then(resp => {
      setOpenItems(items => items.concat(resp.data))
    })
  }

  const finishItem = (item: Item) => {
    item.doneAt = now()
    console.log("finishing item", item)
    setOpenItems(items => items.filter(i => i.id !== item.id))
    setDoneItems(items => items.concat(item))
  }

  const undoItem = (item: Item) => {
    item.doneAt = undefined
    console.log("undoing item", item)
    setOpenItems(items => items.concat(item))
    setDoneItems(items => items.filter(i => i.id !== item.id))
  }

  return (
    <div className="App">
      <header className="App-header">
        <Header />
        <OpenItemSection
          items={openItems}
          setItems={setOpenItems}
          addItem={addItem}
          finishItem={finishItem}
        />
        <DoneItemSection
          items={doneItems}
          undoItem={undoItem}
        // doneItemsCollapsed={doneItemsCollapsed}
        // setDoneItemsCollapsed={setDoneItemsCollapsed}
        />
      </header>
    </div>
  );
}

export default App;
