import { useEffect, useState } from 'react';
import './App.css';
import { Item } from './Types'
import Header from './components/Header'
import OpenItemSection from './components/OpenItemSection'
import DoneItemSection from './components/DoneItemSection'
import axios from 'axios'

function App() {
  const emptyItems: Item[] = []
  const [openItems, setOpenItems] = useState(emptyItems)
  const [doneItems, setDoneItems] = useState(emptyItems)
  const [doneItemsCollapsed, setDoneItemsCollapsed] = useState(true)
  useEffect(() => {
    axios.get('/items', {
      params: {
        done_items_collapsed: doneItemsCollapsed
      }
    }).then(resp => {
      console.log("getOpenItems response", resp)
      setOpenItems(resp.data.open)
      if (resp.data.done) {
        setDoneItems(resp.data.done)
      }
    })
  }, [doneItemsCollapsed])

  const addItem = (newItem: Item) => {
    console.log("creating item", newItem)
    axios.post('/items', newItem).then(resp => {
      setOpenItems(items => items.concat(resp.data))
    })
  }

  const finishItem = (item: Item) => {
    console.log("finishing item", item)
    axios.put(`/items/${item.id}/complete`,
      null,
      {
        params: {
          done_items_collapsed: doneItemsCollapsed
        }
      }
    ).then(resp => {
      console.log("finishItem response", resp)
      setOpenItems(resp.data.open)
      if (resp.data.done) {
        setDoneItems(resp.data.done)
      }
    })
  }

  const undoItem = (item: Item) => {
    item.doneAt = undefined
    console.log("undoing item", item)
    axios.put(`/items/${item.id}/undo`,
      null,
      {
        params: {
          done_items_collapsed: doneItemsCollapsed
        }
      }
    ).then(resp => {
      console.log("undoItem response", resp)
      setOpenItems(resp.data.open)
      if (resp.data.done) {
        setDoneItems(resp.data.done)
      }
    })
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
          doneItemsCollapsed={doneItemsCollapsed}
          setDoneItemsCollapsed={setDoneItemsCollapsed}
        />
      </header>
    </div>
  );
}

export default App;
